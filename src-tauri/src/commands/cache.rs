use std::{process::Command, sync::Mutex};

use qf_api::enums::ApplicationEvent;
use serde_json::Value;
use utils::{get_location, Error};

use crate::{
    cache::{client::CacheState, types::*},
    track_event,
    types::ChatLink,
};

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
            e.log("cache_get_tradable_items.log");
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_get_syndicates(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheSyndicate>, Error> {
    let cache = cache.lock()?;
    match cache.syndicate().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            e.log("cache_get_syndicates.log");
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_get_riven_attributes(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheAttribute>, Error> {
    let cache = cache.lock()?;
    match cache.attribute().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            e.log("cache_get_riven_attributes.log");
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_get_riven_weapons(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheWeaponBase>, Error> {
    let cache = cache.lock()?;
    match cache.weapon().get_all_items() {
        Ok(items) => {
            let riven_weapons = items
                .into_iter()
                .filter(|item| item.wfm_riven_url != "")
                .collect::<Vec<_>>();
            return Ok(riven_weapons);
        }
        Err(e) => {
            e.log("cache_get_riven_weapons.log");
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn cache_get_chat_icons(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<CacheChatIcon>, Error> {
    let cache = cache.lock()?;
    match cache.chat_icon().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            e.log("cache_get_chat_icons.log");
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
            e.log("cache_get_theme_presets.log");
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

    let result = cache
        .theme()
        .create_theme(name, author, properties)
        .and_then(|_| cache.theme().load());

    match &result {
        Ok(_) => track_event!(
            ApplicationEvent::ThemeCreate,
            [("success", "true".to_string())]
        ),
        Err(err) => {
            track_event!(
                ApplicationEvent::ThemeCreate,
                [
                    ("success", "false".to_string()),
                    ("error_type", "create_failed".to_string()),
                ]
            );
            err.log("cache_create_theme.log");
        }
    }

    result
}

#[tauri::command]
pub fn cache_open_theme_folder(cache: tauri::State<'_, Mutex<CacheState>>) -> Result<(), Error> {
    let cache = cache.lock()?;
    let folder = cache.theme().get_theme_folder();

    let result = Command::new("explorer")
        .args(["/select,", &folder.to_string_lossy()])
        .spawn()
        .map(|_| ())
        .map_err(|e| {
            Error::new(
                "Commands:CacheOpenThemeFolder",
                &format!("Failed to open theme folder: {e}"),
                get_location!(),
            )
        });

    match &result {
        Ok(_) => track_event!(
            ApplicationEvent::ThemeOpenFolder,
            [("success", "true".to_string())]
        ),
        Err(err) => {
            track_event!(
                ApplicationEvent::ThemeOpenFolder,
                [
                    ("success", "false".to_string()),
                    ("error_type", "open_folder_failed".to_string()),
                ]
            );
            err.log("cache_open_theme_folder.log");
        }
    }

    result
}

#[tauri::command]
pub fn cache_get_chat_link(
    unique_name: String,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<ChatLink, Error> {
    let cache = cache.lock()?;

    let item = cache.all_items().get_chat_link(unique_name)?;
    Ok(item)
}
