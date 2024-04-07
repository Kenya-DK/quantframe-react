use std::sync::Arc;
use std::sync::Mutex;

use service::sea_orm::DatabaseConnection;
use tauri::AppHandle;
use tauri::PackageInfo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub tauri_app: AppHandle,
}

impl AppState {
    pub fn new(conn: DatabaseConnection, tauri_app: AppHandle) -> Self {
        AppState { conn, tauri_app }
    }

    pub fn get_conn(&self) -> &DatabaseConnection {
        &self.conn
    }

    pub fn get_app_info(&self) -> PackageInfo {
        self.tauri_app.package_info().clone()
    }
}
