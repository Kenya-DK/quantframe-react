use std::sync::Mutex;

use serde_json::{json, Value};

use crate::{
    app::{client::AppState, Settings},
    notification::client::NotificationState,
    utils::modules::error::AppError,
    APP,
};

#[tauri::command]
pub async fn app_get_app_info() -> Result<Value, AppError> {
    let tauri_app = APP.get().expect("App handle not found");
    let info = tauri_app.package_info().clone();
    Ok(json!({
        "version": info.version,
        "name": info.name,
        "description": info.description,
        "authors": info.authors,
        "is_dev": cfg!(dev),
    }))
}

#[tauri::command]
pub async fn app_get_settings(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Settings, AppError> {
    let app = app.lock().expect("Failed to lock AppState");
    Ok(app.settings.clone())
}

#[tauri::command]
pub async fn app_update_settings(
    settings: Settings,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Settings, AppError> {
    let mut app = app.lock().expect("Failed to lock AppState");
    app.settings = settings.clone();
    // Save settings to file or database if necessary
    app.settings.save()?;
    Ok(settings.clone())
}
