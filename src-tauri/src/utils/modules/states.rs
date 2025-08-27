use std::{
    backtrace::Backtrace,
    sync::{Arc, Mutex},
};

use crate::{
    app::{self, client::AppState, Settings},
    cache::client::CacheState,
    notification::client::NotificationState,
    APP,
};
use tauri::{async_runtime::block_on, Manager};
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

pub fn notify_client() -> Result<NotificationState, Error> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<NotificationState>>();
    let guard = state.lock()?;
    Ok(guard.clone())
}

pub fn cache_client() -> Result<CacheState, Error> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<CacheState>>();
    let guard = state.lock()?;
    Ok(guard.clone())
}
