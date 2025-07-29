// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![allow(non_snake_case)]
#![allow(deprecated)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::client::AppState;
// use cache::client::CacheClient;
// use debug::DebugClient;
// use live_scraper::client::LiveScraperClient;
// use log_parser::client::LogParser;
use ::utils::clear_logs;
use ::utils::critical;
use ::utils::error;
use ::utils::info;
use migration::{Migrator, MigratorTrait};
// use notification::client::NotifyClient;
use ::utils::init_logger;
use ::utils::set_base_path;
use ::utils::warning;
use ::utils::Error;
use ::utils::LoggerOptions;
use service::sea_orm::{Database, DatabaseConnection};
// use settings::SettingsState;

use std::env;
use std::panic;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use tauri::{App, Emitter, Manager};

use crate::cache::client::CacheState;
use crate::notification::client::NotificationState;

mod app;
mod utils;
// mod auth;
mod cache;
mod commands;
mod macros;

// mod debug;
mod enums;
mod helper;
// mod http_client;
// mod live_scraper;
// mod log_parser;
mod notification;
// mod qf_client;
// mod settings;
// mod wfm_client;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static DATABASE: OnceLock<DatabaseConnection> = OnceLock::new();
pub static HAS_STARTED: OnceLock<bool> = OnceLock::new();

// If use_debug is true the debug database will be used and all data will be lost on restart
async fn init_database(use_debug: bool) -> Result<(), Error> {
    // Create the database connection and store it
    let storage_path = helper::get_app_storage_path();

    let mut file_name = "quantframeV2.sqlite";
    let debug_file_name = "quantframeV2_debug.sqlite";

    // Create the path to the database file
    let file_path = format!("{}/{}", storage_path.to_str().unwrap(), file_name);

    // Create the path to the database file
    let file_path_backup = format!("{}/{}_backup", storage_path.to_str().unwrap(), file_name);
    info(
        "Setup:Database",
        "Creating a backup of the database file",
        LoggerOptions::default(),
    );
    if std::path::Path::new(&file_path).exists() {
        std::fs::copy(&file_path, &file_path_backup)
            .expect("Failed to create a backup of the database file");
    }

    if use_debug {
        let db_debug_file_path_backup = file_path.replace(file_name, debug_file_name);
        file_name = debug_file_name;
        warning(
            "Setup:Database",
            "Debug mode is enabled, using the debug database file no data wil be saved",
            LoggerOptions::default(),
        );
        if std::path::Path::new(&file_path).exists() {
            std::fs::copy(&file_path, &db_debug_file_path_backup)
                .expect("Failed to create a backup of the database file");
        }
    }

    // Create the database connection URL
    let db_url = format!(
        "sqlite://{}/{}?mode=rwc",
        storage_path.to_str().unwrap(),
        file_name,
    );

    // Create the database connection and store it and run the migrations
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    DATABASE.get_or_init(|| conn);
    Ok(())
}

async fn setup_manages(app: tauri::AppHandle) -> Result<(), Error> {
    // Clear the logs older then 7 days
    clear_logs(7)?;

    let notify_state: Mutex<NotificationState> = Mutex::new(NotificationState::new());
    app.manage(notify_state);

    // Clone the fields needed for CacheState before moving app_state
    let app_state = AppState::new(app.clone()).await;
    let qf_client = app_state.qf_client.clone();
    let user = app_state.user.clone();
    app.manage(Mutex::new(app_state));

    let cache_state = Mutex::new(CacheState::new(&qf_client, &user).await?);
    app.manage(cache_state);

    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the logger for elapsed time tracking

    panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        critical(
            "Panic",
            format!("Panic: {:?}", panic_info).as_str(),
            LoggerOptions::default().set_file("panic.log"),
        );
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            // log_parser::types::trade_detection::init_detections();
            APP.get_or_init(|| app.handle().clone());
            // Clone the handle for async task
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                if let Err(e) = init_database(true).await {
                    emit_error!(e);
                    e.log(Some("init_database_error.log"));
                }
                if let Err(e) = setup_manages(app_handle.clone()).await {
                    emit_error!(e);
                    e.log(Some("setup_error.log"));
                }
                if let Err(e) = app_handle.emit("app:ready", ()) {
                    error(
                        "Emit",
                        &format!("Failed to emit app:ready event: {:?}", e),
                        LoggerOptions::default().set_file("emit_error.log"),
                    );
                }
                HAS_STARTED.set(true).unwrap();
            });
            init_logger();
            set_base_path(helper::get_app_storage_path().to_str().unwrap());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Base commands
            commands::dashboard::dashboard_summary,
            commands::app::was_initialized,
            commands::app::app_get_app_info,
            commands::app::app_get_settings,
            commands::app::app_update_settings,
            // Auth commands
            commands::auth::auth_me,
            commands::auth::auth_login,
            commands::auth::auth_logout,
            // User commands
            commands::user::user_set_status,
            // User commands
            commands::user::user_set_status,
            // Analytics commands
            commands::analytics::analytics_add_metric,
            commands::analytics::analytics_set_last_user_activity,
            // Alert commands
            commands::alert::alert_get_alerts,
            // Cache commands
            commands::cache::cache_get_tradable_items,
            // Log commands
            commands::logs::log_export,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
