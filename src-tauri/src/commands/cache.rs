use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::{
        self,
        client::CacheClient,
        types::{
            cache_riven::{
                CacheRivenDataByRivenInternalID, CacheRivenWfmAttribute, CacheRivenWfmWeapon,
                CacheWeaponStat, RivenStat,
            },
            cache_tradable_item::CacheTradableItem,
        },
    },
    utils::modules::error::{self, AppError},
};

#[tauri::command]
pub async fn cache_reload(
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<(), AppError> {
    let cache = cache.lock()?.clone();
    match cache.load().await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("cache.log".to_string(), &e);
            return Err(e);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn cache_get_tradable_items(
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Vec<CacheTradableItem>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.tradable_items().get_items() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            error::create_log_file("cache_get_tradable_items.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn cache_get_riven_weapons(
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Vec<CacheRivenWfmWeapon>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.riven().get_wfm_riven_types() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            error::create_log_file("cache_get_riven_weapons.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn cache_get_riven_attributes(
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Vec<CacheRivenWfmAttribute>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.riven().get_wfm_riven_attributes() {
        Ok(items) => {
            return Ok(items);
        }
        Err(e) => {
            error::create_log_file("cache_get_riven_attributes.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn cache_get_riven_raw_mod(
    internal_id: String,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Option<CacheRivenDataByRivenInternalID>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.riven().get_riven_raw_mod(&internal_id) {
        Some(riven) => {
            return Ok(Some(riven.clone()));
        }
        None => {
            return Ok(None);
        }
    }
}
#[tauri::command]
pub fn cache_get_weapon_stat(
    internal_id: String,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Option<CacheWeaponStat>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.riven().get_weapon_stat(&internal_id) {
        Some(riven) => {
            return Ok(Some(riven.clone()));
        }
        None => {
            return Ok(None);
        }
    }
}
#[tauri::command]
pub fn cache_get_weapon_upgrades(
    internal_id: String,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Option<HashMap<String, RivenStat>>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.riven().get_weapon_upgrades(&internal_id) {
        Some(riven) => {
            return Ok(Some(riven.clone()));
        }
        None => {
            return Ok(None);
        }
    }
}
