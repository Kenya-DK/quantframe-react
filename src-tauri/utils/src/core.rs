use crate::helper::{format_square_bracket, format_text, remove_ansi_codes};
use crate::options::{LoggerOptions, START_TIME};
use crate::{Error, get_location};
use chrono::{Duration, Local};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
    Critical,
}

impl LogLevel {
    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
            LogLevel::Critical => "CRITICAL",
        }
    }

    /// Get the priority level for filtering (higher number = higher priority)
    /// Debug(0) < Trace(1) < Info(2) < Warning(3) < Error(4) < Critical(5)
    pub fn priority(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Trace => 1,
            LogLevel::Info => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Critical => 5,
        }
    }
}

pub fn dolog(
    level: LogLevel,
    component: impl Into<String>,
    msg: impl Into<String>,
    options: &LoggerOptions,
) {
    if !options.enable {
        return;
    }
    let component_str = component.into();

    // Filter by global component filters if set
    let filter_components = crate::options::get_filter_components();
    if !filter_components.is_empty() && !filter_components.contains(&component_str) {
        return; // Skip logging if component is not in the filter list
    }

    // Filter by global minimum log level if set
    if let Some(min_level) = crate::options::get_min_log_level() {
        if level.priority() < min_level.priority() {
            return; // Skip logging if level is below minimum
        }
    }

    // UTC time format: %Y-%m-%d %H:%M:%S
    let now = Local::now()
        .to_utc()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let time = format_square_bracket(&now, options.color);

    let elapsed = START_TIME
        .get()
        .map_or(0.0, |start| start.elapsed().as_secs_f64());
    let elapsed_str = format_square_bracket(format!("{:.4}", elapsed), options.color);

    let component_formatted = format_square_bracket(
        format_text(component_str.clone(), "magenta", true, options.color),
        options.color,
    );
    let mut message = msg.into();
    if options.centered {
        let width = options.width.min(80);
        let message_length = message.len();
        if message_length >= width {
            return;
        }
        let padding = width - message_length;
        let left_padding: usize = padding / 2;
        let right_padding = padding - left_padding;
        let line = format!(
            "{}{}{}",
            "-".repeat(left_padding),
            message,
            "-".repeat(right_padding)
        );
        message = line;
    }
    let msg = format_text(message, "white", false, options.color);
    let log_level_str = match level {
        LogLevel::Info => format_square_bracket(
            format_text(level.prefix(), "green", true, options.color),
            options.color,
        ),
        LogLevel::Warning => format_square_bracket(
            format_text(level.prefix(), "yellow", true, options.color),
            options.color,
        ),
        LogLevel::Error => format_square_bracket(
            format_text(level.prefix(), "red", true, options.color),
            options.color,
        ),
        LogLevel::Debug => format_square_bracket(
            format_text(level.prefix(), "blue", true, options.color),
            options.color,
        ),
        LogLevel::Trace => format_square_bracket(
            format_text(level.prefix(), "cyan", true, options.color),
            options.color,
        ),
        LogLevel::Critical => format_square_bracket(
            format_text(level.prefix(), "red", true, options.color),
            options.color,
        ),
    };

    let mut prefix = String::new();
    if options.show_time {
        prefix += &format!("{} ", time);
    }
    if options.show_elapsed_time {
        prefix += &format!("{} ", elapsed_str);
    }
    if options.show_level {
        prefix += &format!("{} ", log_level_str);
    }
    if options.show_component {
        prefix += &format!("{} ", component_formatted);
    }

    let message = format!("{}{}", prefix, msg);

    if options.console {
        println!("{}", message.trim());
    }

    if let Some(file_name) = &options.file {
        let folder_path = crate::options::get_folder();
        let file_path = folder_path.join(file_name);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_path)
            .unwrap();
        writeln!(file, "{}", remove_ansi_codes(message)).ok();
    }
}

macro_rules! make_level_fn {
    ($func:ident, $level:ident) => {
        pub fn $func(
            component: impl Into<String>,
            msg: impl Into<String>,
            options: &LoggerOptions,
        ) {
            dolog(LogLevel::$level, component, msg, options)
        }
    };
}

make_level_fn!(info, Info);
make_level_fn!(warning, Warning);
make_level_fn!(error, Error);
make_level_fn!(debug, Debug);
make_level_fn!(trace, Trace);
make_level_fn!(critical, Critical);

/// Log a JSON entry to a file
///
/// # Arguments
/// * `context` - A JSON-serializable context object containing the log data
/// * `file` - The filename to log to (will be created in the logs/ directory)
///
/// # Example
/// ```
/// use serde_json::json;
/// log_json(json!({
///     "level": "INFO",
///     "component": "App",
///     "message": "Application started",
///     "timestamp": chrono::Local::now().to_rfc3339(),
///     "user_id": 12345
/// }), "app.json");
/// ```
pub fn log_json(context: serde_json::Value, file: impl Into<String>) -> Result<(), Error> {
    // Use the formatted version with pretty printing enabled
    log_json_formatted(context, file, true)
}

/// Log a JSON entry to a file with pretty formatting
///
/// # Arguments
/// * `context` - A JSON-serializable context object containing the log data
/// * `file` - The filename to log to (will be created in the logs/ directory)
/// * `pretty` - Whether to format the JSON with indentation and newlines
///
/// # Example
/// ```
/// use serde_json::json;
/// log_json_formatted(json!({
///     "level": "INFO",
///     "component": "App",
///     "message": "Application started",
///     "timestamp": chrono::Local::now().to_rfc3339(),
///     "user_id": 12345
/// }), "app.json", true); // true = pretty formatted
/// ```
pub fn log_json_formatted(
    context: serde_json::Value,
    file: impl Into<String>,
    pretty: bool,
) -> Result<(), Error> {
    let component = "Utility:LoggerJsonFormatted";
    let file_name = file.into();
    let folder_path = crate::options::get_folder();
    let file_path = folder_path.join(file_name);

    // Open file in write mode (overwrites existing content)
    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|e| {
            Error::from_io(
                component,
                &file_path,
                "opening log file",
                e,
                get_location!(),
            )
        })?;

    // Write JSON entry with or without formatting
    let json_string = if pretty {
        serde_json::to_string_pretty(&context).map_err(|e| {
            Error::from_json(
                component,
                &file_path,
                &context.to_string(),
                "Failed to serialize JSON",
                e,
                get_location!(),
            )
        })?
    } else {
        serde_json::to_string(&context).map_err(|e| {
            Error::from_json(
                component,
                &file_path,
                &context.to_string(),
                "Failed to serialize JSON",
                e,
                get_location!(),
            )
        })?
    };

    writeln!(log_file, "{}", json_string).map_err(|e| {
        Error::from_io(
            component,
            &file_path,
            "writing to log file",
            e,
            get_location!(),
        )
    })?;

    Ok(())
}

/// Clear logs older than the specified number of days
///
/// # Arguments
/// * `days` - Number of days to keep (logs older than this will be deleted)
///
/// # Example
/// ```
/// // Delete all logs older than 7 days
/// clear_logs(7)?;
/// ```
pub fn clear_logs(days: i64) -> Result<(), Error> {
    let component = "Utility:ClearLogs";
    let base_path = crate::options::get_base_path();
    let logs_path = Path::new(&base_path).join("logs");

    if !logs_path.exists() {
        return Ok(()); // No logs directory exists, nothing to clear
    }

    let cutoff_date = Local::now() - Duration::days(days);
    let cutoff_date_str = cutoff_date.format("%Y-%m-%d").to_string();

    let entries = fs::read_dir(&logs_path).map_err(|e| {
        Error::from_io(
            component,
            &logs_path,
            "reading logs directory",
            e,
            get_location!(),
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            Error::from_io(
                component,
                &logs_path,
                "reading logs directory entry",
                e,
                get_location!(),
            )
        })?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
                // Check if directory name is a date in YYYY-MM-DD format
                if dir_name.len() == 10
                    && dir_name.chars().nth(4) == Some('-')
                    && dir_name.chars().nth(7) == Some('-')
                {
                    if dir_name < &cutoff_date_str {
                        println!("Removing old log directory: {}", dir_name);
                        fs::remove_dir_all(&path).map_err(|e| {
                            Error::from_io(
                                component,
                                &path,
                                "removing old log directory",
                                e,
                                get_location!(),
                            )
                        })?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Delete a specific log file
/// # Arguments
/// * `file` - The filename to delete (will be looked for in the logs/
/// directory)
/// # Example
/// ```
/// delete_log("error.log")?;
/// ```
pub fn delete_log(file: impl AsRef<Path>) -> Result<(), Error> {
    let component = "Utility:DeleteLog";
    let folder_path = crate::options::get_folder();
    let file_path = folder_path.join(file.as_ref());

    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| {
            Error::from_io(
                component,
                &file_path,
                "deleting log file",
                e,
                get_location!(),
            )
        })?;
    }

    Ok(())
}
