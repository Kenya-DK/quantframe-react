use tauri::AppHandle;
use tauri::PackageInfo;

use crate::utils::modules::logger;
use crate::utils::modules::logger::LoggerOptions;

#[derive(Clone, Debug)]
pub struct AppState {
    pub app_id: String,
    pub is_first_install: bool,
    pub is_pre_release: bool,
    pub is_development: bool,
    pub tauri_app: AppHandle,
}

impl AppState {
    pub fn new(tauri_app: AppHandle, is_first_install: bool, is_pre_release: bool) -> AppState {
        AppState {
            app_id: if cfg!(dev) {
                "DEV".to_string()
            } else {
                "rqf6ahg*RFY3wkn4neq".to_string()
            },
            tauri_app,
            is_first_install,
            is_pre_release,
            is_development: cfg!(dev),
        }
    }

    pub fn get_app_info(&self) -> PackageInfo {
        self.tauri_app.package_info().clone()
    }

    pub fn initialize(&self) {
        logger::info(
            "Initializing AppState...",
            "client",
            LoggerOptions::default(),
        );
    }
}
