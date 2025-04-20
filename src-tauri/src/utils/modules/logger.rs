use eyre::eyre;
use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
use tauri::PackageInfo;

use crate::{helper, utils::enums::log_level::LogLevel};

use super::error::AppError;
use std::sync::OnceLock;
use std::time::Instant;

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

#[derive(Clone)]
pub struct LoggerOptions {
    pub console: bool,
    pub file: Option<String>,
    pub show_time: bool,
    pub show_component: bool,
    pub show_elapsed_time: bool,
    pub show_level: bool,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        LoggerOptions {
            console: true,
            file: None,
            show_time: true,
            show_component: true,
            show_elapsed_time: true,
            show_level: true,
        }
    }
}
impl LoggerOptions {
    pub fn set_console(&mut self, value: bool) -> Self {
        self.console = value;
        self.clone()
    }
    pub fn set_file(&mut self, value: &str) -> Self {
        self.file = Some(value.to_string());
        self.clone()
    }
    pub fn set_show_time(&mut self, value: bool) -> Self {
        self.show_time = value;
        self.clone()
    }
    pub fn set_show_component(&mut self, value: bool) -> Self {
        self.show_component = value;
        self.clone()
    }

    pub fn set_show_level(&mut self, value: bool) -> Self {
        self.show_level = value;
        self.clone()
    }
}

pub fn format_text(text: &str, color: &str, bold: bool) -> String {
    let color_code = match color {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "orange" => "38;5;208",
        _ => "0", // default color
    };

    if bold {
        format!("\x1b[1;{}m{}\x1b[0m", color_code, text)
    } else {
        format!("\x1b[{}m{}\x1b[0m", color_code, text)
    }
}
fn remove_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    re.replace_all(s, "").to_string()
}
fn format_square_bracket(msg: &str) -> String {
    format!(
        "{}{}{}",
        format_text("[", "cyan", false),
        msg,
        format_text("]", "cyan", false)
    )
}

pub fn dolog(level: LogLevel, component: &str, msg: &str, options: LoggerOptions) {
    let time = format_square_bracket(
        chrono::Local::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str(),
    );

    let elapsed_time = START_TIME
        .get()
        .map_or(0.0, |start| start.elapsed().as_secs_f64());
    let elapsed_str = format_square_bracket(format!("{:.4}", elapsed_time).as_str()); // Formats to 4 decimal places

    let component = format_square_bracket(format_text(component, "magenta", true).as_str());
    let msg = format_text(msg, "white", false);
    let log_prefix = match level {
        LogLevel::Info => format_square_bracket(format_text("INFO", "green", true).as_str()),
        LogLevel::Warning => format_square_bracket(format_text("WARN", "yellow", true).as_str()),
        LogLevel::Error => format_square_bracket(format_text("ERROR", "red", true).as_str()),
        LogLevel::Debug => format_square_bracket(format_text("DEBUG", "blue", true).as_str()),
        LogLevel::Trace => format_square_bracket(format_text("TRACE", "cyan", true).as_str()),
        LogLevel::Critical => format_square_bracket(format_text("CRITICAL", "red", true).as_str()),
        _ => format_square_bracket(format_text("UNKNOWN", "white", true).as_str()),
    };

    let mut prefix = String::new();
    if options.show_time {
        prefix = format!("{} {}", prefix, time);
    }
    if options.show_elapsed_time {
        prefix = format!("{} {}", prefix, elapsed_str);
    }

    if options.show_level {
        prefix = format!("{} {}", prefix, log_prefix);
    }

    if options.show_component {
        prefix = format!("{} {}", prefix, component);
    }

    let message = format!("{} {}", prefix, msg);

    if options.console {
        println!("{}", message.trim());
    }

    if let Some(file) = options.file {
        let mut log_path = get_log_folder();
        log_path.push(file);
        if !log_path.exists() {
            fs::File::create(&log_path).unwrap();
        }
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_path)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", remove_ansi_codes(&message)) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn get_log_folder() -> PathBuf {
    let app_path = helper::get_app_storage_path();
    let log_path = app_path.join("logs");
    // Create the directory if it does not exist
    if !log_path.exists() {
        fs::create_dir_all(&log_path).unwrap();
    }
    //create a folder for the current local date
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();
    let log_path = log_path.join(date);
    // Create the directory if it does not exist
    if !log_path.exists() {
        fs::create_dir_all(&log_path).unwrap();
    }
    log_path
}

pub fn debug(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Debug, component, msg, options);
}

pub fn error(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Error, component, msg, options);
}

pub fn info(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Info, component, msg, options);
}

pub fn trace(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Trace, component, msg, options);
}
pub fn critical(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Critical, component, msg, options);
}

pub fn warning(component: &str, msg: &str, options: LoggerOptions) {
    dolog(LogLevel::Warning, component, msg, options);
}

#[allow(dead_code)]
pub fn clear_log_file(file_path: &str) -> Result<(), AppError> {
    let path = get_log_folder().join(file_path);
    if path.exists() {
        fs::write(&path, "").map_err(|e| AppError::new("clear_log_file", eyre!(e.to_string())))?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn log_json(file_path: &str, data: &Value) -> Result<(), AppError> {
    let path = get_log_folder().join(file_path);
    let file =
        std::fs::File::create(path).map_err(|e| AppError::new("log_json", eyre!(e.to_string())))?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)
        .map_err(|e| AppError::new("log_json", eyre!(e.to_string())))?;
    Ok(())
}
pub fn clear_logs(days: i64) -> Result<(), AppError> {
    // Get the logs folder there is older then the days
    let app_path = helper::get_app_storage_path();
    let log_path = app_path.join("logs");
    if !log_path.is_dir() {
        return Ok(());
    }
    for path in fs::read_dir(log_path).unwrap() {
        let path = path.unwrap().path();
        // Check if path is auth.json
        if !path.is_dir() {
            continue;
        }
        let folder_name = path.file_name().unwrap().to_str().unwrap();

        match chrono::NaiveDate::parse_from_str(folder_name, "%Y-%m-%d") {
            Ok(date) => {
                if date >= chrono::Local::now().naive_utc().date() - chrono::Duration::days(days) {
                    continue;
                }
                fs::remove_dir_all(path)
                    .map_err(|e| AppError::new("clear_logs", eyre!(e.to_string())))?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

pub fn export_logs(info: PackageInfo) -> String {
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();

    let version = info.version.to_string();
    let app_path = helper::get_app_storage_path();

    let zip_path =
        helper::get_desktop_path().join(format!("{} v{} {} Logs.zip", info.name, version, date));
    let mut files_to_compress: Vec<helper::ZipEntry> = vec![];

    let mut logs_path = get_log_folder();
    logs_path.pop();

    files_to_compress.push(helper::ZipEntry {
        file_path: logs_path,
        sub_path: Some("logs".to_string()),
        content: None,
        include_dir: true,
    });

    // Cache path
    let cache_path = app_path.join("cache");
    if cache_path.exists() {
        files_to_compress.push(helper::ZipEntry {
            file_path: cache_path,
            sub_path: Some("cache".to_string()),
            content: None,
            include_dir: true,
        });
    }

    for path in fs::read_dir(app_path).unwrap() {
        let path = path.unwrap().path();
        // Check if path is auth.json
        if path.ends_with("auth.json") || path.ends_with("settings.json") {
            let json = helper::open_json_and_replace(
                &path.to_str().unwrap(),
                vec![
                    "check_code".to_string(),
                    "qf_access_token".to_string(),
                    "wfm_access_token".to_string(),
                    "webhook".to_string(),
                ],
            )
            .expect("Could not open auth.json");

            files_to_compress.push(helper::ZipEntry {
                file_path: path.to_owned(),
                sub_path: None,
                content: Some(serde_json::to_string_pretty(&json).unwrap()),
                include_dir: false,
            });
        } else {
            files_to_compress.push(helper::ZipEntry {
                file_path: path.to_owned(),
                sub_path: None,
                content: None,
                include_dir: false,
            });
        }
    }

    match helper::create_zip_file(files_to_compress, zip_path.to_str().unwrap_or_default()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    zip_path.to_str().unwrap_or_default().to_string()
}
