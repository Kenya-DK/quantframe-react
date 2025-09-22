use std::sync::Mutex;

use entity::{
    dto::{PaginatedResult, SortDirection},
    enums::FieldChange,
};
use serde_json::json;
use utils::{filters_by, get_location, info, Error, LoggerOptions};
use wf_market::types::{update_user_private, Chat, ChatMessage};

use crate::{
    app::client::{set_active_chat_id, AppState, ACTIVE_CHAT_ID},
    emit_update_user,
    helper::paginate,
    utils::{ErrorFromExt, WfmChatPaginationQueryDto},
};

#[tauri::command]
pub async fn chat_refresh(app: tauri::State<'_, Mutex<AppState>>) -> Result<(), Error> {
    let app = app.lock()?.clone();
    match app.wfm_client.chat().get_chats().await {
        Ok(chats) => {
            info(
                "Commands:ChatRefresh",
                format!("Refreshed {} chats", chats.len()),
                &LoggerOptions::default(),
            );
            emit_update_user!(json!({ "unread_messages": chats.total_unread_count() }));
            Ok(())
        }
        Err(e) => {
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

#[tauri::command]
pub async fn chat_delete(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app = app.lock()?.clone();
    match app.wfm_client.chat().leave_chat(&id).await {
        Ok(id) => {
            info(
                "Commands:ChatDelete",
                format!("Deleted chat with id {}", id),
                &LoggerOptions::default(),
            );
            return Ok(id);
        }
        Err(e) => {
            let err = Error::from_wfm("Command", "failed to delete chat", e, get_location!());
            err.log("command_chat_delete.log");
            return Err(err);
        }
    }
}

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
            Ok(messages)
        }
        Err(e) => {
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
#[tauri::command]
pub async fn chat_send_message(
    id: String,
    msg: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app_state = app.lock().unwrap().clone();
    if app_state.wfm_socket.is_none() {
        return Err(Error::new(
            "Commands:ChatSendMessage",
            "WebSocket is not connected, please login first.",
            get_location!(),
        ));
    }
    let wfm_socket = app_state.wfm_socket.as_ref().unwrap();
    match wfm_socket.send_request(
        "@WS/chats/SEND_MESSAGE",
        json!({
                "chat_id": id,
                "message": msg
        }),
    ) {
        Ok(_) => {
            info(
                "Commands:ChatSendMessage",
                &format!("Sent message to chat {}", id),
                &LoggerOptions::default(),
            );
        }
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
