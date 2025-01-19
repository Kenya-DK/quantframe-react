use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::{
    app::client::AppState, auth::AuthState, cache::client::CacheClient,
    log_parser::client::LogParser, notification::client::NotifyClient, qf_client::client::QFClient,
    settings::SettingsState, wfm_client::client::WFMClient, APP,
};

use super::error::AppError;

pub fn qf_client() -> Result<QFClient, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<QFClient>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:QFClient", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn settings() -> Result<SettingsState, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<SettingsState>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:SettingsState", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn app_state() -> Result<AppState, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<AppState>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:AppState", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn notify_client() -> Result<NotifyClient, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<NotifyClient>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:NotifyClient", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn wfm_client() -> Result<WFMClient, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<WFMClient>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:WFMClient", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn log_parser() -> Result<LogParser, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<LogParser>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:LogParser", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn auth() -> Result<AuthState, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<AuthState>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:AuthState", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}

pub fn cache() -> Result<CacheClient, AppError> {
    let app = APP.get().expect("App not initialized");
    let state = app.state::<Arc<Mutex<CacheClient>>>();
    let state_lock = state
        .lock()
        .map_err(|_| AppError::new("States:CacheClient", eyre::eyre!("Failed to lock state")))?;
    let state = state_lock.clone();
    Ok(state)
}
