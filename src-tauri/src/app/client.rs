use service::sea_orm::DatabaseConnection;
use tauri::AppHandle;
use tauri::PackageInfo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub is_first_install: bool,
    pub tauri_app: AppHandle,
}

impl AppState {
    pub fn new(conn: DatabaseConnection, tauri_app: AppHandle, is_first_install: bool) -> AppState {
        AppState {
            conn,
            tauri_app,
            is_first_install,
        }
    }

    pub fn get_app_info(&self) -> PackageInfo {
        self.tauri_app.package_info().clone()
    }
}
