use std::sync::Mutex;

use utils::Error;

use crate::cache::{
    client::CacheState,
    types::{RivenPriceEstimate, RivenPriceInput},
};

/// Returns true when `price_model.onnx` is present in the app cache models dir.
#[tauri::command]
pub fn riven_pricer_available(cache: tauri::State<'_, Mutex<CacheState>>) -> Result<bool, Error> {
    let cache = cache.lock()?;
    Ok(cache.riven_pricer().is_available())
}

/// Runs a riven price prediction. Returns `None` if the model is unavailable
/// or the weapon is not in the model vocabulary.
#[tauri::command]
pub async fn riven_price_predict(
    input: RivenPriceInput,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Option<RivenPriceEstimate>, Error> {
    let pricer = {
        let cache = cache.lock()?;
        cache.riven_pricer()
    };
    Ok(
        tauri::async_runtime::spawn_blocking(move || pricer.predict(&input))
            .await
            .unwrap_or_default(),
    )
}

/// Forces the model to be (re)loaded from disk. Returns whether it loaded.
#[tauri::command]
pub fn riven_pricer_reload(cache: tauri::State<'_, Mutex<CacheState>>) -> Result<bool, Error> {
    let pricer = {
        let cache = cache.lock()?;
        cache.riven_pricer()
    };
    pricer.reload()?;
    Ok(pricer.is_loaded())
}

/// The weapon keys known to the model vocabulary.
#[tauri::command]
pub async fn get_known_riven_weapons(
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<String>, Error> {
    let pricer = {
        let cache = cache.lock()?;
        cache.riven_pricer()
    };
    Ok(
        tauri::async_runtime::spawn_blocking(move || pricer.get_weapon_names())
            .await
            .unwrap_or_default(),
    )
}
