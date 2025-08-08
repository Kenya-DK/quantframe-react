use std::{process::Command, sync::Mutex};

use serde_json::Value;
use utils::Error;

use crate::cache::{client::CacheState, types::*};

#[tauri::command]
pub async fn cache_get_tradable_items(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheTradableItem>, Error> {
    let cache = cache.lock()?;
    match cache.tradable_item().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            e.log(Some("cache_get_tradable_items.log"));
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_get_theme_presets(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheTheme>, Error> {
    let cache = cache.lock()?;
    match cache.theme().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            e.log(Some("cache_get_theme_presets.log"));
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_create_theme(
    name: String,
    author: String,
    properties: Value,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<(), Error> {
    let cache = cache.lock()?;
    match cache.theme().create_theme(name, author, properties) {
        Ok(_) => {
            cache.theme().load()?;
            return Ok(());
        }
        Err(e) => {
            e.log(Some("cache_create_theme.log"));
            return Err(e);
        }
    }
}
#[tauri::command]
pub fn cache_open_theme_folder(cache: tauri::State<'_, Mutex<CacheState>>) {
    let cache = cache.lock().expect("Failed to lock cache state");
    Command::new("explorer")
        .args([
            "/select,",
            &cache.theme().get_theme_folder().to_str().unwrap(),
        ])
        .spawn()
        .unwrap();
}
