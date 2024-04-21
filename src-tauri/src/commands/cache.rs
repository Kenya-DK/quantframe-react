use std::sync::{Arc, Mutex};



use crate::{
    cache::{client::CacheClient, types::{cache_riven::{CacheRivenWfmAttribute, CacheRivenWfmWeapon}, cache_tradable_item::CacheTradableItem}}, utils::modules::error::{self, AppError}
};

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
