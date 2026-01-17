use service::sea_orm::DatabaseConnection;
use tauri::AppHandle;
use tauri::PackageInfo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub app_id: String,
    pub conn: DatabaseConnection,
    pub is_first_install: bool,
    pub is_pre_release: bool,
    pub is_development: bool,
    pub tauri_app: AppHandle,
}

impl AppState {
    pub fn new(
        conn: DatabaseConnection,
        tauri_app: AppHandle,
        is_first_install: bool,
        is_pre_release: bool,
    ) -> AppState {
        AppState {
            app_id: "rqf6ahg*RFY3wkn4neq".to_string(),
            conn,
            tauri_app,
            is_first_install,
            is_pre_release,
            is_development: cfg!(debug_assertions),
        }
    }

    pub fn get_app_info(&self) -> PackageInfo {
        self.tauri_app.package_info().clone()
    }
}
