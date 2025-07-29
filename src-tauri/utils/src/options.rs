use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Instant;

#[derive(Clone)]
pub struct LoggerOptions {
    pub console: bool,
    pub file: Option<String>,
    pub show_time: bool,
    pub show_component: bool,
    pub show_elapsed_time: bool,
    pub show_level: bool,
    pub color: bool,
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
            color: true,
        }
    }
}
impl LoggerOptions {
    #[allow(dead_code)]
    pub fn set_console(&mut self, value: bool) -> Self {
        self.console = value;
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_file(&mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_show_time(&mut self, value: bool) -> Self {
        self.show_time = value;
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_show_component(&mut self, value: bool) -> Self {
        self.show_component = value;
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_show_elapsed_time(&mut self, value: bool) -> Self {
        self.show_elapsed_time = value;
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_show_level(&mut self, value: bool) -> Self {
        self.show_level = value;
        self.clone()
    }
    #[allow(dead_code)]
    pub fn set_color(&mut self, value: bool) -> Self {
        self.color = value;
        self.clone()
    }
}
pub static START_TIME: OnceLock<Instant> = OnceLock::new();
pub static BASE_PATH: OnceLock<String> = OnceLock::new();

pub fn init_logger() {
    START_TIME.get_or_init(Instant::now);
}

/// Set the base path for all log files
///
/// # Arguments
/// * `path` - The base directory path where logs will be saved (e.g., "C:\\Users\\Kenya\\Desktop\\Andet")
///
/// # Example
/// ```
/// set_base_path("C:\\Users\\Kenya\\Desktop\\Andet");
/// // All logs will now be saved to the specified directory with date subdirectories
/// // e.g., C:\Users\Kenya\Desktop\Andet\2025-07-26\app.log
/// ```
pub fn set_base_path(path: impl Into<String>) {
    BASE_PATH.set(path.into()).ok();
}

/// Get the current base path for logs, defaults to "logs" if not set
pub fn get_base_path() -> String {
    BASE_PATH.get().cloned().unwrap_or_else(|| "".to_string())
}

/// Get the full save folder path with date subdirectory created
///
/// This function replaces the complex path building logic by:
/// 1. Getting the base path (defaults to "logs")
/// 2. Adding current date subdirectory (e.g., "2025-07-26")
/// 3. Creating the directory if it doesn't exist
/// 4. Returning the full PathBuf
///
/// # Returns
/// PathBuf to the date-based save folder, ready to use
///
/// # Example
/// ```
/// let folder_path = get_folder();
/// let file_path = folder_path.join("app.log");
/// ```
pub fn get_folder() -> PathBuf {
    let mut path = PathBuf::from(get_base_path());

    // Add current date subdirectory
    path.push("logs");

    // Create directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }

    // Add current date subdirectory
    let date_folder = Local::now().format("%Y-%m-%d").to_string();
    path.push(date_folder);

    // Create directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }

    path
}
