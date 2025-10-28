use qf_api::errors::ApiError as QFRequestError;
use serde_json::json;
use utils::{Error, LogLevel};
use wf_market::errors::ApiError as WFRequestError;

/// Extension trait for creating Error instances from different error types
pub trait ErrorFromExt {
    /// Create an Error from a Warframe Market API error
    fn from_wfm(
        component: impl Into<String>,
        message: impl Into<String>,
        error: WFRequestError,
        location: impl Into<String>,
    ) -> Self;

    /// Create an Error from a QuantFrame API error
    fn from_qf(
        component: impl Into<String>,
        message: impl Into<String>,
        error: QFRequestError,
        location: impl Into<String>,
    ) -> Self;
    fn new_permission_denied(flag: impl Into<String>) -> Self;
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
            component: format!("QFClient:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            context: Some(error.to_json()),
            location: Some(location.into()),
        }
    }
    fn new_permission_denied(flag: impl Into<String>) -> Self {
        Error {
            component: "AuthModule".into(),
            cause: "Permission Denied".to_string(),
            message: format!("User does not have permission: {}", flag.into()),
            log_level: LogLevel::Warning,
            context: None,
            location: None,
        }
    }
}
