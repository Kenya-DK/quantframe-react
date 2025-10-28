use sea_orm::DbErr;
use serde_json::json;
use utils::{Error, LogLevel};

/// Extension trait for creating Error instances from different error types
pub trait ErrorFromExt {
    /// Create an Error from a database error
    fn from_db(
        component: impl Into<String>,
        message: impl Into<String>,
        error: DbErr,
        location: impl Into<String>,
    ) -> Self;
    fn new_permission_denied(flag: impl Into<String>) -> Self;
}

impl ErrorFromExt for Error {
    fn from_db(
        component: impl Into<String>,
        message: impl Into<String>,
        error: DbErr,
        location: impl Into<String>,
    ) -> Self {
        Error {
            component: format!("Service:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            context: Some(json!(error.to_string())),
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
