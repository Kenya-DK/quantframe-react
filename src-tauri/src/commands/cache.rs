use std::sync::Mutex;

use utils::Error;

use crate::cache::{client::CacheState, types::CacheTradableItem};

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
