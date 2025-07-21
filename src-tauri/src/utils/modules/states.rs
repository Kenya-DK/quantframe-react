use std::sync::Arc;

use tauri::{async_runtime::block_on, Manager};
use tokio::sync::Mutex;

use crate::{app::client::AppState, notification::client::NotificationState, APP};

use super::error::AppError;

pub fn app_state() -> Result<AppState, AppError> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<AppState>>();
    let guard = block_on(state.lock());
    Ok(guard.clone())
}

pub fn notify_client() -> Result<NotificationState, AppError> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<std::sync::Mutex<NotificationState>>();
    let guard = state.lock().expect("Failed to lock notification state");
    Ok(guard.clone())
}
