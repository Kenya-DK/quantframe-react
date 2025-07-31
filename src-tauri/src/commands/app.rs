use std::sync::Mutex;

use serde_json::{json, Value};
use utils::Error;

use crate::{
    app::{client::AppState, Settings},
    APP, HAS_STARTED,
};

#[tauri::command]
pub async fn was_initialized() -> Result<bool, Error> {
    let started = HAS_STARTED.get().cloned().unwrap_or(false);
    return Ok(started);
}
#[tauri::command]
pub async fn app_get_app_info(app: tauri::State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    let app = app.lock()?;
    let tauri_app = APP.get().expect("App handle not found");
    let info = tauri_app.package_info().clone();
    Ok(json!({
        "version": info.version,
        "name": info.name,
        "description": info.description,
        "authors": info.authors,
        "is_dev": cfg!(dev),
        "tos_uuid": app.settings.tos_uuid.clone()
    }))
}

#[tauri::command]
pub async fn app_get_settings(app: tauri::State<'_, Mutex<AppState>>) -> Result<Settings, Error> {
    let app = app.lock()?;
    Ok(app.settings.clone())
}

#[tauri::command]
pub async fn app_update_settings(
    settings: Settings,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Settings, Error> {
    let mut app = app.lock()?;
    app.update_settings(settings.clone())?;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn app_exit() -> Result<Settings, Error> {
    std::process::exit(0);
}
#[tauri::command]
pub async fn app_accept_tos(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut app = app.lock()?;
    app.settings.tos_uuid = id;
    app.settings.save()?;
    Ok(())
}
