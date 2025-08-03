use std::sync::Mutex;

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
            return Ok(());
        }
        Err(e) => {
            e.log(Some("cache_create_theme.log"));
            return Err(e);
        }
    }
}
