use std::{io::Read, path::PathBuf, sync::Mutex};

use serde_json::{json, Value};
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{
    app::{client::AppState, Settings},
    helper, APP, HAS_STARTED,
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
const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn base64_encode_bytes(data: &[u8]) -> String {
    let mut encoded = String::new();
    let mut i = 0;

    while i < data.len() {
        let b1 = data[i];
        let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        let triple = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        encoded.push(BASE64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if i + 1 < data.len() {
            encoded.push(BASE64_CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            encoded.push('=');
        }

        if i + 2 < data.len() {
            encoded.push(BASE64_CHARS[(triple & 0x3F) as usize] as char);
        } else {
            encoded.push('=');
        }

        i += 3;
    }

    encoded
}
