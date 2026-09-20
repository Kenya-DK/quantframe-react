use std::sync::Mutex;

use entity::{dto::PaginatedResult, enums::FieldChange};
use qf_api::enums::ApplicationEvent;
use serde_json::json;
use utils::{filters_by, get_location, info, sorting::SortDirection, Error, LoggerOptions};
use wf_market::types::{Chat, ChatMessage};

use crate::{
    app::{set_active_chat_id, AppState},
    emit_update_user,
    helper::paginate,
    track_event,
    utils::{ErrorFromExt, WfmChatPaginationQueryDto},
};

// --------------------------------------------------
// Refresh
// --------------------------------------------------

#[tauri::command]
pub async fn chat_refresh(app: tauri::State<'_, Mutex<AppState>>) -> Result<(), Error> {
    let app = app.lock()?.clone();

    let result = app.wfm_client.chat().get_chats().await;

    match result {
        Ok(chats) => {
            info(
                "Commands:ChatRefresh",
                format!("Refreshed {} chats", chats.len()),
                &LoggerOptions::default(),
            );

            emit_update_user!(json!({ "unread_messages": chats.total_unread_count() }));

            track_event!(
                ApplicationEvent::ChatRefresh,
                [
                    ("success", "true".to_string()),
                    ("count", chats.len().to_string()),
                ]
            );

            Ok(())
        }
        Err(e) => {
            let error_type = e.error_type().to_string();

            track_event!(
                ApplicationEvent::ChatRefresh,
                [("success", "false".to_string()), ("error_type", error_type),]
            );

            let err = Error::from_wfm("Command", "failed to refresh chats", e, get_location!());
            err.log("command_chat_refresh.log");
            Err(err)
        }
    }
}
#[tauri::command]
pub async fn get_chat_pagination(
    query: WfmChatPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<PaginatedResult<Chat>, Error> {
    let app = app.lock()?;
    let mut filtered_chats = filters_by(&app.wfm_client.chat().cache_chats().chats, |o| {
        match &query.query {
            FieldChange::Value(q) => {
                let item_name = o.chat_name.to_lowercase();
                if !item_name.contains(&q.to_lowercase()) {
                    return false;
                }
            }
            _ => {}
        }

        true
    });

    match &query.sort_by {
        FieldChange::Value(sort_by) => {
            let dir = match &query.sort_direction {
                FieldChange::Value(dir) => dir,
                _ => &SortDirection::Asc,
            };
            // Only allow sorting by known columns for safety
            match sort_by.as_str() {
                "last_update" => filtered_chats.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.last_update.cmp(&b.last_update),
                    SortDirection::Desc => b.last_update.cmp(&a.last_update),
                }),
                // "last_update" => stmt = stmt.order_by(stock_item::Column::Bought, order),
                // "chat_name" => stmt = stmt.order_by(stock_item::Column::Status, order),
                _ => {}
            }
        }
        _ => {}
    }

    let paginate = paginate(
        &filtered_chats,
        query.pagination.page,
        query.pagination.limit,
    );
    Ok(paginate)
}

// --------------------------------------------------
// Delete
// --------------------------------------------------

#[tauri::command]
pub async fn chat_delete(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app = app.lock()?.clone();

    let result = app.wfm_client.chat().leave_chat(&id).await;

    match result {
        Ok(id) => {
            info(
                "Commands:ChatDelete",
                format!("Deleted chat with id {}", id),
                &LoggerOptions::default(),
            );

            track_event!(
                ApplicationEvent::ChatDelete,
                [("success", "true".to_string())]
            );

            Ok(id)
        }
        Err(e) => {
            track_event!(
                ApplicationEvent::ChatDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

            let err = Error::from_wfm("Command", "failed to delete chat", e, get_location!());
            err.log("command_chat_delete.log");
            Err(err)
        }
    }
}
// --------------------------------------------------
// Get Messages
// --------------------------------------------------

#[tauri::command]
pub async fn chat_get_messages_by_id(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<ChatMessage>, Error> {
    let app = app.lock()?.clone();
    match app.wfm_client.chat().get_chat_messages(&id).await {
        Ok(messages) => {
            info(
                "Commands:ChatGetMessagesByChatId",
                format!("Fetched {} messages for chat {}", messages.len(), id),
                &LoggerOptions::default(),
            );

            set_active_chat_id(Some(id));
            emit_update_user!(
                json!({ "unread_messages": app.wfm_client.chat().cache_chats().total_unread_count() })
            );

            track_event!(
                ApplicationEvent::ChatGetMessages,
                [
                    ("success", "true".to_string()),
                    ("count", messages.len().to_string()),
                ]
            );

            Ok(messages)
        }
        Err(e) => {
            track_event!(
                ApplicationEvent::ChatGetMessages,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

            let err = Error::from_wfm(
                "Command",
                "failed to get messages by chat id",
                e,
                get_location!(),
            );
            err.log("command_chat_get_messages_by_chat_id.log");
            Err(err)
        }
    }
}
#[tauri::command]
pub async fn chat_set_active(id: Option<String>) -> Result<(), Error> {
    set_active_chat_id(id);
    Ok(())
}
// --------------------------------------------------
// Send Message
// --------------------------------------------------

#[tauri::command]
pub async fn chat_send_message(
    id: String,
    msg: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();

    let socket = match app.wfm_chat_socket.as_ref() {
        Some(socket) => socket,
        None => {
            track_event!(
                ApplicationEvent::ChatSendMessage,
                [
                    ("success", "false".to_string()),
                    ("error_type", "websocket_not_connected".to_string()),
                ]
            );

            return Err(Error::new(
                "Commands:ChatSendMessage",
                "WebSocket is not connected, please login first.",
                get_location!(),
            ));
        }
    };

    let result = socket.send_request(
        "@WS/chats/SEND_MESSAGE",
        json!({
            "chat_id": id,
            "message": msg,
        }),
    );

    match result {
        Ok(_) => {
            info(
                "Commands:ChatSendMessage",
                &format!("Sent message to chat {}", id),
                &LoggerOptions::default(),
            );

            track_event!(
                ApplicationEvent::ChatSendMessage,
                [("success", "true".to_string())]
            );

            Ok(())
        }
        Err(e) => {
            track_event!(
                ApplicationEvent::ChatSendMessage,
                [
                    ("success", "false".to_string()),
                    ("error_type", "websocket_send_failed".to_string()),
                ]
            );

            let err = Error::new(
                "Commands:ChatSendMessage",
                &format!("Failed to send message: {:?}", e),
                get_location!(),
            );
            err.log("command_chat_send_message.log");

            Err(err)
        }
    }
}
