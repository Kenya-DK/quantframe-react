use std::sync::Mutex;

use crate::{
    app::{client::AppState, Settings},
    cache::client::CacheState,
    APP,
};
use tauri::Manager;
use utils::Error;

pub fn app_state() -> Result<AppState, Error> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<AppState>>();
    let guard = state.lock()?;
    Ok(guard.clone())
}

pub fn get_settings() -> Result<Settings, Error> {
    let app = app_state()?;
    let settings = app.settings.clone();
    Ok(settings)
}

pub fn cache_client() -> Result<CacheState, Error> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<CacheState>>();
    let guard = state.lock()?;
    Ok(guard.clone())
}
