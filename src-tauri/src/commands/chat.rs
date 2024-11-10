use std::sync::{Arc, Mutex};

use serde_json::json;

use crate::{
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::{
        client::WFMClient,
        types::{chat_data::ChatData, chat_message::ChatMessage},
    },
};

#[tauri::command]
pub async fn chat_refresh(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<Vec<ChatData>, AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    match wfm.chat().get_chats().await {
        Ok(chats) => {
            notify.gui().send_event_update(
                UIEvent::UpdateChats,
                UIOperationEvent::Set,
                Some(json!(chats)),
            );
            return Ok(chats);
        }
        Err(e) => {
            error::create_log_file("command_chat_refresh.log".to_string(), &e);
            return Err(e);
        }
    };
}

#[tauri::command]
pub async fn chat_delete(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    match wfm.chat().delete(id.clone()).await {
        Ok(_) => {
            notify.gui().send_event_update(
                UIEvent::UpdateChats,
                UIOperationEvent::Delete,
                Some(json!({ "id": id })),
            );
        }
        Err(e) => {
            error::create_log_file("command_chat_delete.log".to_string(), &e);
            return Err(e);
        }
    };
    Ok(())
}

#[tauri::command]
pub async fn chat_get_messages(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<Vec<ChatMessage>, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.chat().get_chat_messages(id.clone()).await {
        Ok(messages) => {
            return Ok(messages);
        }
        Err(e) => {
            error::create_log_file("command_chat_get_messages.log".to_string(), &e);
            return Err(e);
        }
    };
}

#[tauri::command]
pub async fn chat_on_message(
    msg: ChatMessage,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();

    // Check if the chat exists
    let cache_chats = wfm.chat().chats.clone();
    let chat = cache_chats.iter().find(|c| c.id == msg.chat_id);
    println!("{:?}", chat);
    if chat.is_none() {
        let chats = wfm.chat().get_chats().await?;
        notify.gui().send_event_update(
            UIEvent::UpdateChats,
            UIOperationEvent::Set,
            Some(json!(chats)),
        );
        return Ok(());
    } else {
        wfm.chat().receive_message(msg.clone()).await?;
    }

    Ok(())
}
#[tauri::command]
pub async fn chat_set_active(
    id: Option<String>,
    unread: i64,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    wfm.chat()
        .set_active_chat(id.clone(), unread.clone())
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn chat_delete_all(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();

    let chats = wfm.chat().get_chats().await?;
    for chat in chats {
        match wfm.chat().delete(chat.id.clone()).await {
            Ok(_) => {
                notify.gui().send_event_update(
                    UIEvent::UpdateChats,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": chat.id })),
                );
            }
            Err(e) => {
                error::create_log_file("command_chat_delete_all.log".to_string(), &e);
                return Err(e);
            }
        }
    }
    Ok(())
}
