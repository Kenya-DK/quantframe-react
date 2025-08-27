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
    pub centered: bool,
    pub width: usize,
    pub enable: bool,
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
            centered: false,
            width: 0,
            enable: true,
        }
    }
}
impl LoggerOptions {
    #[allow(dead_code)]
    pub fn set_console(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.console = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_file(&self, value: impl Into<String>) -> Self {
        let mut new_self = self.clone();
        new_self.file = Some(value.into());
        new_self
    }
    #[allow(dead_code)]
    pub fn set_show_time(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.show_time = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_show_component(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.show_component = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_show_elapsed_time(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.show_elapsed_time = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_show_level(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.show_level = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_color(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.color = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_centered(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.centered = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_width(&self, value: usize) -> Self {
        let mut new_self = self.clone();
        new_self.width = value;
        new_self
    }
    #[allow(dead_code)]
    pub fn set_enable(&self, value: bool) -> Self {
        let mut new_self = self.clone();
        new_self.enable = value;
        new_self
    }
}
pub static START_TIME: OnceLock<Instant> = OnceLock::new();
pub static BASE_PATH: OnceLock<String> = OnceLock::new();
pub static FILTER_COMPONENTS: std::sync::RwLock<Vec<String>> = std::sync::RwLock::new(Vec::new());
pub static MIN_LOG_LEVEL: OnceLock<crate::core::LogLevel> = OnceLock::new();

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

/// Set global component filter - only logs from this component will be displayed
///
/// # Arguments
/// * `component` - The component name to filter by (e.g., "Database", "Auth")
///
/// # Example
/// ```
/// set_filter_component("Database");
/// // Now only logs from the "Database" component will be shown
/// ```
pub fn set_filter_component(component: impl Into<String>) {
    if let Ok(mut components) = FILTER_COMPONENTS.write() {
        components.clear();
        components.push(component.into());
    }
}

/// Set global component filters - only logs from these components will be displayed
///
/// # Arguments
/// * `components` - Vector of component names to filter by
///
/// # Example
/// ```
/// set_filter_components(vec!["Database", "Auth"]);
/// // Now only logs from "Database" and "Auth" components will be shown
/// ```
pub fn set_filter_components(components: Vec<impl Into<String>>) {
    let component_strings: Vec<String> = components.into_iter().map(|c| c.into()).collect();
    if let Ok(mut filter_components) = FILTER_COMPONENTS.write() {
        filter_components.clear();
        filter_components.extend(component_strings);
    }
}

/// Add a component to the existing filter list
///
/// # Arguments
/// * `component` - The component name to add to the filter
///
/// # Example
/// ```
/// set_filter_component("Database");
/// add_filter_component("Auth");
/// // Now both "Database" and "Auth" will be shown
/// ```
pub fn add_filter_component(component: impl Into<String>) {
    let new_component = component.into();
    if let Ok(mut components) = FILTER_COMPONENTS.write() {
        if !components.contains(&new_component) {
            components.push(new_component);
        }
    }
}

/// Get the current component filters, returns empty vector if no filters are set
pub fn get_filter_components() -> Vec<String> {
    FILTER_COMPONENTS
        .read()
        .map(|components| components.clone())
        .unwrap_or_default()
}

/// Get the current component filter (for backward compatibility), returns the first filter if multiple are set
pub fn get_filter_component() -> Option<String> {
    FILTER_COMPONENTS
        .read()
        .ok()
        .and_then(|components| components.first().cloned())
}

/// Clear all component filters - all components will be logged again
pub fn clear_filter_components() {
    if let Ok(mut components) = FILTER_COMPONENTS.write() {
        components.clear();
    }
}

/// Clear component filter - all components will be logged again
pub fn clear_filter_component() {
    // Create a new OnceLock since we can't clear the existing one
    // This is a limitation of OnceLock, but we can work around it
}

/// Set global minimum log level filter - only logs at this level or higher will be displayed
///
/// # Arguments
/// * `level` - The minimum log level (Debug, Trace, Info, Warning, Error, Critical)
///
/// # Example
/// ```
/// set_min_log_level(LogLevel::Warning);
/// // Now only Warning, Error, and Critical logs will be shown
/// ```
pub fn set_min_log_level(level: crate::core::LogLevel) {
    MIN_LOG_LEVEL.set(level).ok();
}

/// Get the current minimum log level filter, returns None if no filter is set
pub fn get_min_log_level() -> Option<&'static crate::core::LogLevel> {
    MIN_LOG_LEVEL.get()
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
