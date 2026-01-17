use std::sync::{Arc, Mutex};

use crate::{notification::client::NotifyClient, utils::modules::error::AppError};

#[tauri::command]
pub fn send_system_notification(
    title: String,
    message: String,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let notify = notify.lock()?.clone();
    notify
        .system()
        .send_notification(&title, &message, None, None);
    Ok(())
}
