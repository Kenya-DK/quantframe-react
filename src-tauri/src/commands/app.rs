use std::sync::Mutex;

use serde_json::{json, Value};
use tauri::{path::BaseDirectory, Manager};
use utils::{get_location, info, Error, LoggerOptions};

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
    let app2 = APP.get().expect("App handle not found");
    let resource_path = app2
        .path()
        .resolve("resources/themes/", BaseDirectory::Resource)
        .unwrap();
    // Get All files in the themes directory
    let themes = std::fs::read_dir(resource_path.clone()).map_err(|e| {
        Error::new(
            "AppState:GetSettings",
            "Failed to read themes directory",
            get_location!(),
        )
    })?;
    let themes: Vec<String> = themes
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    path.file_stem().and_then(|s| s.to_str()).map(String::from)
                } else {
                    None
                }
            })
        })
        .collect();
    info(
        "Commands:AppGetSettings",
        &format!("Available themes: {:?}", themes),
        LoggerOptions::default(),
    );
    println!("Resource path: {:?}", resource_path);
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
