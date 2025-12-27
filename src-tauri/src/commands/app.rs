use std::sync::{Arc, Mutex};

use crate::{
    app::{client::AppState, Settings},
    log_parser::LogParserState,
    APP, HAS_STARTED,
};
use serde_json::{json, Value};
use utils::Error;

#[tauri::command]
pub async fn initialized() -> Result<bool, Error> {
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
        "is_dev": app.is_development,
        "use_temp_db": app.use_temp_db,
        "tos_uuid": app.settings.tos_uuid.clone(),
        "is_pre_release": app.is_pre_release,
        "patreon_usernames": vec!["Willjsnider s", "DAn IguEss"],
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
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Settings, Error> {
    let mut app = app.lock()?;
    let log_parser = log_parser.lock()?;
    log_parser.set_path(&app.settings.advanced_settings.wf_log_path)?;
    if settings.http_server.uuid() != app.settings.http_server.uuid() {
        let operation = app
            .http_server
            .set_host(&settings.http_server.host, settings.http_server.port);
        match (settings.http_server.enable, operation.as_str()) {
            (true, "NO_CHANGE") => app.http_server.start(),
            (false, "NO_CHANGE") => app.http_server.stop(),
            (true, "CHANGED") => app.http_server.restart(),
            (false, "CHANGED") => app.http_server.stop(),
            _ => {}
        }
    }
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
    let mut settings = app.settings.clone();
    settings.tos_uuid = id.clone();
    app.update_settings(settings)?;
    Ok(())
}
#[tauri::command]
pub async fn app_notify_reset(id: String) -> Result<Value, Error> {
    let value = json!(crate::app::NotificationsSetting::default());
    if value[id.clone()].is_object() {
        return Ok(value[id.clone()].clone());
    }
    Ok(json!({}))
}
