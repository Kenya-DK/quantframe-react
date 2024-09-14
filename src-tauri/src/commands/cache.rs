use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::{
        client::CacheClient,
        types::{
            cache_relics::CacheRelics, cache_riven::{CacheRivenWFMAttribute, CacheRivenWeapon}, cache_tradable_item::CacheTradableItem
        },
    }, qf_client::client::QFClient, utils::modules::error::{self, AppError}
};

#[tauri::command]
pub async fn cache_reload(
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let cache = cache.lock()?.clone();
    let qf = qf.lock()?.clone();
    match cache.load().await {
        Ok(_) => {
            qf.analytics()
                .add_metric("Cache_Reload", "manual");
        }
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
) -> Result<Vec<CacheRivenWeapon>, AppError> {
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
) -> Result<Vec<CacheRivenWFMAttribute>, AppError> {
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
pub fn cache_get_tradable_item(
    input: String,
    by: String,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<Option<CacheRelics>, AppError> {
    let cache = cache.lock()?.clone();
    match cache.relics().get_by(&input, &by) {
        Ok(item) => {
            return Ok(item);
        }
        Err(e) => {
            return Err(e);
        }
    }
}