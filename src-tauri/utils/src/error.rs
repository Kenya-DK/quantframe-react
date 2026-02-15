use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fmt::Display,
    path::PathBuf,
    sync::{Arc, MutexGuard, PoisonError},
};
// const MAX_LOCATION_LENGTH: usize = 1024;
// const MAX_CONTEXT_LENGTH: usize = 4048;
const MAX_LOCATION_LENGTH: usize = 1024;
const MAX_CONTEXT_LENGTH: usize = 1048;
const MAX_CAUSE_LENGTH: usize = 1024;
/// A comprehensive error type for the Uties logging library
///
/// This error type captures detailed information about errors that occur
/// during logging operations, including component context, error causes,
/// stack traces, and additional metadata for debugging.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// The component where the error occurred (e.g., "FileLogger", "ZipLogger", "Network")
    pub component: String,
    /// A human-readable error message
    pub message: String,
    /// The underlying cause of the error (if available)
    pub cause: String,
    /// The severity level of this error
    pub log_level: LogLevel,
    /// Additional context information (optional JSON data)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
    /// Location information for debugging (captured when error is created)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl Error {
    /// Create a new Error with the specified component and message
    ///
    /// # Arguments
    /// * `component` - The component where the error occurred
    /// * `message` - A descriptive error message
    ///
    /// # Example
    /// ```
    /// let error = Error::new("FileLogger", "Failed to write to file");
    /// ```
    pub fn new(
        component: impl Into<String>,
        message: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Error {
            component: component.into(),
            message: message.into(),
            log_level: LogLevel::Critical,
            cause: String::new(),
            context: None,
            location: Some(location.into()),
        }
    }

    /// Set the underlying cause of this error
    ///
    /// # Arguments
    /// * `cause` - A string describing the root cause
    ///
    /// # Example
    /// ```
    /// let error = Error::new("Network", "Connection failed")
    ///     .with_cause("Connection timeout after 30 seconds");
    /// ```
    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = cause.into();
        self
    }

    /// Set the log level for this error
    ///
    /// # Arguments
    /// * `level` - The severity level of this error
    ///
    /// # Example
    /// ```
    /// let error = Error::new("Cache", "Cache miss")
    ///     .set_log_level(LogLevel::Warning);
    /// ```
    pub fn set_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    /// Set the component for this error
    ///
    /// # Arguments
    /// * `component` - The component where the error occurred
    ///
    /// # Example
    /// ```
    /// let error = Error::new("FileLogger", "Failed to write to file")
    ///     .set_component("FileLogger");
    /// ```
    pub fn set_component(mut self, component: impl Into<String>) -> Self {
        self.component = component.into();
        self
    }

    /// Set the message for this error
    ///
    /// # Arguments
    /// * `message` - A descriptive error message
    ///
    /// # Example
    /// ```
    /// let error = Error::new("FileLogger", "Failed to write to file")
    ///     .set_message("Unable to write to log file");
    /// ```
    pub fn set_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Add contextual information to this error
    ///
    /// # Arguments
    /// * `context` - Additional JSON data providing context
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// let error = Error::new("Database", "Query failed")
    ///     .with_context(json!({
    ///         "query": "SELECT * FROM users",
    ///         "execution_time_ms": 5000
    ///     }));
    /// ```
    pub fn with_context(mut self, context: Value) -> Self {
        self.context = Some(context);
        self
    }

    /// Append a location to this error
    ///
    /// # Arguments
    /// * `location` - A string describing the location of the error
    ///
    /// # Example
    /// ```
    /// let error = Error::new("FileLogger", "Failed to write to file")
    ///     .with_location("src/logger.rs:42");
    /// ```
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        if let Some(ref mut loc) = self.location {
            loc.push_str(&format!(" -> {}", location.into()));
        } else {
            self.location = Some(location.into());
        }
        self
    }

    /// Mask sensitive data in the context JSON
    ///
    /// # Arguments
    /// * `properties` - A list of property names to mask
    /// # Example
    /// ```
    /// use serde_json::json;
    /// let mut error = Error::new("API", "Sensitive data leaked")
    ///    .with_context(json!({
    ///         "user_id": 42,
    ///         "email": "user@example.com"
    ///     });
    /// error.mask_sensitive_data(&["email"]);
    /// ```
    pub fn mask_sensitive_data(&mut self, properties: &[&str]) {
        if let Some(Value::Object(ref mut context)) = self.context {
            crate::helper::mask_sensitive_data(context, properties);
        }
    }

    /// Mask sensitive data in a JSON file and return the masked content
    ///
    /// # Arguments
    /// * `file_path` - Path to the JSON file
    /// * `properties` - A list of property names to mask
    /// * `save_masked` - Whether to save the masked content back to file
    ///
    /// # Returns
    /// Result containing the masked JSON content as a string
    ///
    /// # Example
    /// ```
    /// let masked_content = Error::mask_sensitive_data_in_file(
    ///     "config.json",
    ///     &["password", "api_key", "secret"],
    ///     false
    /// )?;
    /// ```
    pub fn mask_sensitive_data_in_file(
        file_path: impl Into<PathBuf>,
        properties: &[&str],
        save_masked: bool,
    ) -> Result<String, Error> {
        let path = file_path.into();

        // Read the file content
        let content = std::fs::read_to_string(&path).map_err(|e| {
            Error::from_io(
                "FileMasking",
                &path,
                "reading file for masking",
                e,
                "Error::mask_sensitive_data_in_file",
            )
        })?;

        // Parse JSON
        let mut json_value: Value = serde_json::from_str(&content).map_err(|e| {
            Error::from_json(
                "FileMasking",
                &path,
                &content,
                "Failed to parse JSON for masking",
                e,
                "Error::mask_sensitive_data_in_file",
            )
        })?;

        // Mask sensitive data
        if let Value::Object(ref mut obj) = json_value {
            crate::helper::mask_sensitive_data(obj, properties);
        }

        // Convert back to string
        let masked_content = serde_json::to_string_pretty(&json_value).map_err(|e| {
            Error::from_json(
                "FileMasking",
                &path,
                "masked_json",
                "Failed to serialize masked JSON",
                e,
                "Error::mask_sensitive_data_in_file",
            )
        })?;

        // Optionally save back to file
        if save_masked {
            std::fs::write(&path, &masked_content).map_err(|e| {
                Error::from_io(
                    "FileMasking",
                    &path,
                    "writing masked content to file",
                    e,
                    "Error::mask_sensitive_data_in_file",
                )
            })?;
        }

        Ok(masked_content)
    }

    /// Convert this error to a formatted log message with custom options
    /// Automatically truncates context if it's too large for console output (max 2048 chars
    /// Includes stack trace information when available
    ///
    /// # Returns
    /// A formatted string suitable for logging
    pub fn log_with_options(&self, file: impl Into<String>, options: &LoggerOptions) -> Self {
        let mut options = options.clone();
        let file_name = file.into();
        if !file_name.is_empty() && options.file.is_none() {
            options.file = Some(file_name);
        }
        let mut message = format!("{}", self.message);

        if !self.cause.is_empty() {
            message.push_str(&format!(" | Cause: {}", self.cause));
        }

        // Handle location
        if let Some(location) = &self.location {
            if location.len() > MAX_LOCATION_LENGTH {
                let truncated_location = format!(
                    "{}... [LOCATION TRUNCATED - {} total chars]",
                    &location[..MAX_LOCATION_LENGTH.saturating_sub(50)],
                    location.len()
                );
                message.push_str(&format!(" | Location: {}", truncated_location));
            } else {
                message.push_str(&format!(" | Location: {}", location));
            }
        }

        if let Some(context) = &self.context {
            let context_str = context.to_string();

            if context_str.len() > MAX_CONTEXT_LENGTH {
                // Truncate context for console output but keep full context for file
                let truncated_context = format!(
                    "{}... [TRUNCATED - {} total chars]",
                    &context_str[..MAX_CONTEXT_LENGTH.saturating_sub(50)],
                    context_str.len()
                );

                // For console: use truncated context
                if options.console {
                    let console_message = format!("{} | Context: {}", message, truncated_context);
                    let console_options = LoggerOptions {
                        console: true,
                        file: None,
                        ..options
                    };
                    dolog(
                        self.log_level.clone(),
                        &self.component,
                        console_message,
                        &console_options,
                    );
                }

                // For file: use full context (but keep stack truncated for readability)
                if options.file.is_some() {
                    let file_message = format!("{} | Context: {}", message, context_str);
                    let file_options = LoggerOptions {
                        console: false,
                        ..options.clone()
                    };
                    dolog(
                        self.log_level.clone(),
                        &self.component,
                        file_message,
                        &file_options,
                    );
                }
            } else {
                // Context is small enough, include it normally
                message.push_str(&format!(" | Context: {}", context_str));
                dolog(self.log_level.clone(), &self.component, message, &options);
            }
        } else {
            // No context to worry about
            dolog(self.log_level.clone(), &self.component, message, &options);
        }
        self.clone() // Return self for chaining if needed
    }
    /// Convert this error to a formatted log message
    /// Automatically truncates context if it's too large for console output (max 2048 chars)
    /// Includes stack trace information when available
    ///
    /// # Returns
    /// A formatted string suitable for logging
    pub fn log(&self, file: impl Into<String>) -> Self {
        self.log_with_options(file, &LoggerOptions::default())
    }

    /// Check if this is a critical error
    pub fn is_critical(&self) -> bool {
        matches!(self.log_level, LogLevel::Critical)
    }

    /// Check if this is an error-level issue
    pub fn is_error(&self) -> bool {
        matches!(self.log_level, LogLevel::Error | LogLevel::Critical)
    }

    pub fn from_io(
        component: &str,
        path: &PathBuf,
        message: &str,
        err: std::io::Error,
        location: impl Into<String>,
    ) -> Self {
        Error {
            component: format!("IOError:{}", component),
            message: format!("An I/O error occurred while {}: {}", message, err),
            cause: err.to_string(),
            log_level: LogLevel::Critical,
            context: Some(json!({ "path": path })),
            location: Some(location.into()),
        }
    }
    pub fn from_json(
        component: impl Into<String>,
        path: &PathBuf,
        content: impl Into<String>,
        message: impl Into<String>,
        err: serde_json::Error,
        location: impl Into<String>,
    ) -> Self {
        let mut content_str = content.into();

        let line_info = err.line();
        let column_info = err.column();

        let position_info = format!(" at line {}, column {}", line_info, column_info);
        let detailed_cause = format!("{}{}", err, position_info);

        let error_type = match err.classify() {
            serde_json::error::Category::Io => "IO",
            serde_json::error::Category::Syntax => "Syntax",
            serde_json::error::Category::Data => "Data",
            serde_json::error::Category::Eof => "EOF",
        };

        // Highlight error
        content_str.insert_at(line_info, column_info, " <<< ERROR HERE <<< ");

        // 🔒 truncate AFTER highlighting
        let (content_str, _) = truncate_with_indicator(&content_str, 1000, None);

        Error {
            component: format!("ParseError:{}", component.into()),
            message: message.into(),
            cause: detailed_cause,
            log_level: LogLevel::Critical,
            context: Some(json!({
                "path": path.to_str(),
                "content": content_str,
                "line": line_info,
                "column": column_info,
                "error_type": error_type,
                "truncated": content_str.len() == 1000
            })),
            location: Some(location.into()),
        }
    }

    pub fn from_zip(
        component: impl Into<String>,
        file_name: impl Into<String>,
        message: impl Into<String>,
        err: zip::result::ZipError,
        location: impl Into<String>,
    ) -> Self {
        Error {
            component: format!("ZipError:{}", component.into()),
            message: message.into(),
            cause: err.to_string(),
            log_level: LogLevel::Critical,
            context: Some(json!({ "file_name": file_name.into() })),
            location: Some(location.into()),
        }
    }
    pub fn from_arc<T>(
        component: &str,
        archive_name: &str,
        message: &str,
        err: Arc<PoisonError<MutexGuard<'_, T>>>,
        location: impl Into<String>,
    ) -> Self {
        Error {
            component: format!("ArcError:{}", component),
            message: message.to_string(),
            cause: err.to_string(),
            location: Some(location.into()),
            log_level: LogLevel::Critical,
            context: Some(json!({ "archive_name": archive_name })),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (message, _) = truncate_with_indicator(&self.message, 1000, None);

        if let Some(location) = &self.location {
            let (location, _) = truncate_with_indicator(location, 200, None);

            write!(
                f,
                "{}: {} | Cause: {} | Location: {}",
                self.component, message, self.cause, location
            )
        } else {
            write!(f, "{}: {} | Cause: {}", self.component, message, self.cause)
        }
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for Error {
    fn from(err: PoisonError<MutexGuard<'_, T>>) -> Self {
        Error::new("Mutex", "Failed to lock mutex", "Unknown location")
            .with_cause(err.to_string())
            .set_log_level(LogLevel::Critical)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new("IO", "I/O operation failed", "Unknown location")
            .with_cause(err.to_string())
            .set_log_level(LogLevel::Critical)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        let line_info = err.line();
        let column_info = err.column();

        let position_info = format!(" at line {}, column {}", line_info, column_info);
        let (detailed_cause, _) =
            truncate_with_indicator(&format!("{}{}", err, position_info), 1000, None);

        Error::new("JSON", "JSON parsing failed", "Unknown location")
            .with_cause(detailed_cause)
            .set_log_level(LogLevel::Critical)
    }
}
