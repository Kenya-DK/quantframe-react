use std::sync::{Arc, Mutex};

use qf_api::types::ManualUpdate;
use serde_json::{json, Value};
use utils::{get_location, Error};

use crate::{
    add_metric,
    app::{client::AppState, Settings},
    log_parser::LogParserState,
    send_system_notification,
    utils::ErrorFromExt,
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
        "is_dev": app.is_development,
        "use_temp_db": app.use_temp_db,
        "tos_uuid": app.settings.tos_uuid.clone(),
        "patreon_usernames": vec!["Willjsnider s", "Hmh", "Jessie"],
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
    app.update_settings(settings.clone())?;
    log_parser.set_path(&app.settings.advanced_settings.wf_log_path)?;
    send_system_notification!(
        "Quantframe Started",
        "The application has started successfully.",
        None,
        None
    );
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
    app.settings.tos_uuid = id.clone();
    app.settings.save()?;
    add_metric!("app_accept_tos", id);
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
#[tauri::command]
pub async fn app_check_for_updates(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<ManualUpdate, Error> {
    let app = app.lock()?.clone();
    let target = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let tauri_app = APP.get().expect("App handle not found");
    match app
        .qf_client
        .check_updates(
            target,
            arch,
            tauri_app.package_info().clone().version.to_string(),
        )
        .await
    {
        Ok(release) => Ok(release),
        Err(e) => {
            return Err(Error::from_qf(
                "Command::AppCheckForUpdates",
                "Failed to check for updates: {}",
                e,
                get_location!(),
            ));
        }
    }
}
