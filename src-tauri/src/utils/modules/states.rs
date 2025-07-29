use std::{
    backtrace::Backtrace,
    sync::{Arc, Mutex},
};

use crate::{
    app::client::AppState, cache::client::CacheState, notification::client::NotificationState, APP,
};
use qf_api::errors::ApiError as QFRequestError;
use tauri::{async_runtime::block_on, Manager};
use utils::{Error, LogLevel};
use wf_market::errors::ApiError as WFRequestError;

pub fn app_state() -> Result<AppState, Error> {
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<AppState>>();
    let guard = state.lock()?;
    Ok(guard.clone())
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

pub trait ErrorFromExt {
    fn from_wfm(
        component: impl Into<String>,
        message: impl Into<String>,
        error: WFRequestError,
        location: impl Into<String>,
    ) -> Self;
    fn from_qf(
        component: impl Into<String>,
        message: impl Into<String>,
        error: QFRequestError,
        location: impl Into<String>,
    ) -> Self;
}

impl ErrorFromExt for Error {
    fn from_wfm(
        component: impl Into<String>,
        message: impl Into<String>,
        mut error: WFRequestError,
        location: impl Into<String>,
    ) -> Self {
        error.mask_sensitive_data(&["email", "password", "authorization"]);
        Error {
            component: format!("WFMClient:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            context: Some(error.to_json()),
            location: Some(location.into()),
        }
    }
    fn from_qf(
        component: impl Into<String>,
        message: impl Into<String>,
        mut error: QFRequestError,
        location: impl Into<String>,
    ) -> Self {
        error.mask_sensitive_data(&["email", "password", "authorization"]);
        Error {
            component: format!("WFMClient:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            context: Some(error.to_json()),
            location: Some(location.into()),
        }
    }
}
