use std::sync::{Arc, Mutex};

use crate::{app::client::AppState, helper, utils::modules::error::AppError};

#[tauri::command]
pub fn analytics_set_last_user_activity(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let app = app.lock().expect("Failed to lock AppState");
    app.qf_client.analytics().set_last_user_activity();
    Ok(())
}

#[tauri::command]
pub fn analytics_add_metric(
    key: String,
    value: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let app = app.lock().expect("Failed to lock AppState").clone();
    app.qf_client.analytics().add_metric(&key, &value);
    Ok(())
}
