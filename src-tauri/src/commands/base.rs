use entity::transaction;
use serde_json::{json, Value};
use service::{StockItemQuery, StockRivenQuery, TransactionMutation, TransactionQuery};
use std::sync::{Arc, Mutex};
use tokio::process::Command;

use crate::{
    app::client::AppState,
    auth::AuthState,
    cache::client::CacheClient,
    debug::DebugClient,
    helper, logger,
    notification::client::NotifyClient,
    settings::SettingsState,
    utils::{
        enums::log_level::LogLevel,
        modules::error::{self, AppError},
    },
    wfm_client::{client::WFMClient, types::chat_message::ChatMessage},
};

#[tauri::command]
pub async fn update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;

    // Set Log in Settings
    my_lock.debug = settings.debug;

    // Set Live Scraper Settings
    my_lock.live_scraper = settings.live_scraper;

    // Set Whisper Scraper Settings
    my_lock.notifications = settings.notifications;

    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}

#[tauri::command]
pub async fn open_logs_folder() {
    Command::new("explorer")
        .args(["/select,", &logger::get_log_folder().to_str().unwrap()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn show_notification(
    title: String,
    message: String,
    _icon: Option<String>,
    sound: Option<String>,
    notify: tauri::State<'_, Arc<std::sync::Mutex<NotifyClient>>>,
) {
    let notify = notify.lock().unwrap();
    notify
        .system()
        .send_notification(&title, &message, None, sound.as_deref());
}

#[tauri::command]
pub fn log(
    component: String,
    msg: String,
    level: LogLevel,
    console: Option<bool>,
    file: Option<&str>,
) {
    let (console, file) = match (console, file) {
        (Some(console), Some(file)) => (console, Some(file)),
        (Some(console), None) => (console, None),
        (None, Some(file)) => (false, Some(file)),
        (None, None) => (false, None),
    };
    logger::dolog(level, &component, &msg, console, file);
}

#[tauri::command]
pub fn export_logs(notify: tauri::State<'_, Arc<std::sync::Mutex<NotifyClient>>>) {
    let notify = notify.lock().unwrap().clone();
    logger::export_logs();

    notify
        .system()
        .send_notification("Logs Exported", "Logs exported to desktop", None, None);
}
