use std::sync::{Arc, Mutex};


use once_cell::sync::Lazy;

use serde_json::{json};


// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_chat.log".to_string()));

use crate::{
    error::{self, AppError},
    wfm_client::{
        client::WFMClient,
        modules::chat::{ChatData, ChatMessage},
    },
};

#[tauri::command]
pub async fn refresh_chats(
    exclude: Vec<String>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<Vec<ChatData>, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.chat().get_chats().await {
        Ok(chat) => {
            let mut chats = chat.clone();
            for id in exclude {
                chats.retain(|x| x.id != id);
            }
            for chat in chats.clone() {
                wfm.chat().emit("CREATE_OR_UPDATE", json!(chat.clone()));
            }
            Ok(chats)
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn get_chat(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<Vec<ChatMessage>, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.chat().get_chat(id).await {
        Ok(chat) => Ok(chat),
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn delete_chat(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<String, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.chat().delete(id).await {
        Ok(chat) => Ok(chat),
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
