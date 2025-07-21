use std::path::PathBuf;

use ::serde::{Deserialize, Serialize};
use chrono::serde;
use regex::Regex;
use serde_json::{json, Value};

use crate::utils::enums::log_level::LogLevel;
use qf_api::errors::ApiError as QFRequestError;
use wf_market::errors::ApiError as WFRequestError;

use super::logger::{self, LoggerOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub component: String,
    pub message: String,
    pub cause: String,
    pub log_level: LogLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
}
impl AppError {
    // Custom constructor
    pub fn new(component: &str, message: &str) -> Self {
        AppError {
            component: component.to_string(),
            message: message.to_string(),
            log_level: LogLevel::Critical,
            cause: String::new(),
            context: None,
        }
    }

    pub fn from_wfm(component: &str, message: &str, mut error: WFRequestError) -> Self {
        error.mask_sensitive_data(&["email", "password", "authorization"]);
        AppError {
            component: format!("WFMClient:{}", component),
            cause: error.to_string(),
            message: message.to_string(),
            log_level: LogLevel::Critical,
            context: Some(error.to_json()),
        }
    }

    pub fn from_qf(component: &str, message: &str, mut error: QFRequestError) -> Self {
        error.mask_sensitive_data(&["email", "password", "authorization"]);
        AppError {
            component: format!("WFMClient:{}", component),
            cause: error.to_string(),
            message: message.to_string(),
            log_level: LogLevel::Critical,
            context: Some(error.to_json()),
        }
    }

    pub fn from_io(component: &str, path: &PathBuf, message: &str, err: std::io::Error) -> Self {
        AppError {
            component: format!("IOError:{}", component),
            message: format!("An I/O error occurred while {}: {}", message, err),
            cause: err.to_string(),
            log_level: LogLevel::Critical,
            context: Some(json!({ "path": path})),
        }
    }
    pub fn from_json(
        component: &str,
        content: &str,
        message: &str,
        err: serde_json::Error,
    ) -> Self {
        AppError {
            component: format!("ParseError:{}", component),
            message: message.to_string(),
            cause: err.to_string(),
            log_level: LogLevel::Critical,
            context: Some(json!({ "content": content })),
        }
    }

    pub fn with_context(mut self, context: Value) -> Self {
        self.context = Some(context);
        self
    }
    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = cause.into();
        self
    }
    pub fn set_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self
    }
    pub fn log(&self, file: Option<impl Into<String>>) {
        let mut options = LoggerOptions::default();
        if let Some(file) = file {
            options.set_file(file.into());
        }

        logger::log_error(&self, options);
    }
}

// impl<T> From<std::sync::PoisonError<T>> for AppError {
//     fn from(e: std::sync::PoisonError<T>) -> Self {
//         AppError::new("PoisonError", eyre!(e.to_string()))
//     }
// }
