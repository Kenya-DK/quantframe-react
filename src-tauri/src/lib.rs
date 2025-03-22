// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![allow(non_snake_case)]
#![allow(deprecated)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::client::AppState;
use auth::AuthState;
use cache::client::CacheClient;
use debug::DebugClient;
use live_scraper::client::LiveScraperClient;
use log_parser::client::LogParser;
use migration::{Migrator, MigratorTrait};
use notification::client::NotifyClient;
use service::sea_orm::{Database, DatabaseConnection};
use settings::SettingsState;
use utils::modules::error::AppError;
use utils::modules::logger::{self, LoggerOptions, START_TIME};

use std::panic;
use std::sync::{Arc, OnceLock};
use std::time::Instant;
use std::{env, sync::Mutex};

use tauri::async_runtime::block_on;
use tauri::{App, Manager};

mod app;
mod auth;
mod cache;
mod commands;
mod debug;
mod enums;
mod helper;
mod http_client;
mod live_scraper;
mod log_parser;
mod notification;
mod qf_client;
mod settings;
mod utils;
mod wfm_client;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static DATABASE: OnceLock<DatabaseConnection> = OnceLock::new();

// If use_debug is true the debug database will be used and all data will be lost on restart
async fn init_database(use_debug: bool) -> Result<(), AppError> {
    // Create the database connection and store it
    let storage_path = helper::get_app_storage_path();

    let mut file_name = "quantframeV2.sqlite";
    let debug_file_name = "quantframeV2_debug.sqlite";

    // Create the path to the database file
    let file_path = format!("{}/{}", storage_path.to_str().unwrap(), file_name);

    // Create the path to the database file
    let file_path_backup = format!("{}/{}_backup", storage_path.to_str().unwrap(), file_name);
    logger::info(
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
        logger::warning(
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

async fn setup_manages(app: &mut App) -> Result<(), AppError> {
    // Clear the logs older then 7 days
    logger::info(
        "Setup:Logs",
        "Clearing logs older then 7 days",
        LoggerOptions::default(),
    );
    logger::clear_logs(7)?;
    // Get the update channel
    // let context = tauri::generate_context!();
    // let updater = context.config().tauri.updater.clone();

    // let mut is_pre_release = false;
    // if updater.active && updater.endpoints.is_some() {
    //     let endpoints = updater.endpoints.as_ref().unwrap();
    //     for endpoint in endpoints {
    //         if endpoint.to_string().contains("prerelease") {
    //             is_pre_release = true;
    //         }
    //     }
    // }

    // Check if the app is being run for the first time
    let is_first_install = !helper::dose_app_exist();

    // Create and manage Notification state
    let app_arc: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(
        app.handle().clone(),
        is_first_install,
        false,
    )));
    app.manage(app_arc.clone());

    // Create and manage Notification state
    let notify_arc: Arc<Mutex<NotifyClient>> = Arc::new(Mutex::new(NotifyClient::new()));
    app.manage(notify_arc.clone());

    // create and manage Settings state
    let settings_arc = Arc::new(Mutex::new(SettingsState::setup()?));
    app.manage(settings_arc.clone());

    // create and manage Auth state
    let auth_arc = Arc::new(Mutex::new(AuthState::setup()?));
    app.manage(auth_arc.clone());

    // create and manage Quantframe client state
    let qf_client = Arc::new(Mutex::new(qf_client::client::QFClient::new()));
    app.manage(qf_client.clone());

    // create and manage Warframe Market API client state
    let wfm_client = Arc::new(Mutex::new(wfm_client::client::WFMClient::new()));
    app.manage(wfm_client.clone());

    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheClient::new()));
    app.manage(cache_arc.clone());

    // create and manage HTTP client state
    let http_client_arc = Arc::new(Mutex::new(http_client::client::HttpClient::setup()?));
    app.manage(http_client_arc.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraperClient::new();
    app.manage(Arc::new(Mutex::new(live_scraper)));

    // create and manage WhisperScraper state
    let debug_client = DebugClient::new();
    app.manage(Arc::new(Mutex::new(debug_client)));

    let log_parser = LogParser::new();
    app.manage(Arc::new(Mutex::new(log_parser)));
    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        logger::critical(
            "Panic",
            format!("Panic: {:?}", panic_info).as_str(),
            LoggerOptions::default().set_file("panic.log"),
        );
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .setup(move |app| {
            log_parser::types::trade_detection::init_detections();
            START_TIME.set(Instant::now()).unwrap();
            APP.get_or_init(|| app.handle().clone());

            match block_on(init_database(false)) {
                Ok(_) => {}
                Err(e) => {
                    let component = e.component();
                    let cause = e.cause();
                    let backtrace = e.backtrace();
                    let log_level = e.log_level();
                    logger::dolog(
                        log_level,
                        component.as_str(),
                        format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
                        LoggerOptions::default().set_file("init_database_error.log"),
                    );
                }
            };

            // Setup Manages for the app
            match block_on(setup_manages(app)) {
                Ok(_) => {}
                Err(e) => {
                    let component = e.component();
                    let cause = e.cause();
                    let backtrace = e.backtrace();
                    let log_level = e.log_level();
                    logger::dolog(
                        log_level,
                        component.as_str(),
                        format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
                        LoggerOptions::default().set_file("setup_error.log"),
                    );
                }
            };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Base commands
            commands::app::app_init,
            commands::app::app_exit,
            commands::app::app_update_settings,
            // Auth commands
            commands::auth::auth_login,
            commands::auth::auth_set_status,
            commands::auth::auth_logout,
            // Cache commands
            commands::cache::cache_reload,
            commands::cache::cache_get_tradable_items,
            commands::cache::cache_get_riven_weapons,
            commands::cache::cache_get_riven_attributes,
            commands::cache::cache_get_tradable_item,
            // Transaction commands
            commands::transaction::transaction_reload,
            commands::transaction::transaction_get_all,
            commands::transaction::transaction_update,
            commands::transaction::transaction_delete,
            // Debug commands
            commands::analytics::analytics_set_last_user_activity,
            commands::analytics::analytics_send_metric,
            // Debug commands
            commands::debug::debug_db_reset,
            commands::debug::debug_migrate_data_base,
            commands::debug::debug_method,
            commands::debug::debug_import_algo_trader,
            // Log commands
            commands::log::log_open_folder,
            commands::log::log_export,
            commands::log::log_send,
            // Auctions commands
            commands::auctions::auction_refresh,
            commands::auctions::auction_delete,
            commands::auctions::auction_delete_all,
            commands::auctions::auction_import,
            // Orders commands
            commands::orders::order_delete,
            commands::orders::order_delete_all,
            commands::orders::order_refresh,
            // Chat commands
            commands::chat::chat_refresh,
            commands::chat::chat_delete,
            commands::chat::chat_on_message,
            commands::chat::chat_get_messages,
            commands::chat::chat_set_active,
            commands::chat::chat_delete_all,
            // Live Trading commands
            commands::live_scraper::live_scraper_set_running_state,
            // Stock Item commands
            commands::stock_item::stock_item_reload,
            commands::stock_item::stock_item_create,
            commands::stock_item::stock_item_update,
            commands::stock_item::stock_item_update_bulk,
            commands::stock_item::stock_item_sell,
            commands::stock_item::stock_item_delete,
            commands::stock_item::stock_item_delete_bulk,
            // Stock Riven commands
            commands::stock_riven::stock_riven_reload,
            commands::stock_riven::stock_riven_update,
            commands::stock_riven::stock_riven_update_bulk,
            commands::stock_riven::stock_riven_sell,
            commands::stock_riven::stock_riven_delete,
            commands::stock_riven::stock_riven_delete_bulk,
            commands::stock_riven::stock_riven_create,
            // Wish List commands
            commands::wish_list::wish_list_reload,
            commands::wish_list::wish_list_create,
            commands::wish_list::wish_list_update,
            commands::wish_list::wish_list_delete,
            commands::wish_list::wish_list_bought,
            // Notification commands
            commands::notification::send_system_notification,
            // Item commands
            commands::item::item_get_syndicates_prices,
            commands::item::item_get_prices,
            // Page Home commands
            commands::pages::home::get_statistic,
            // Log Parser commands
            commands::log_parser::get_cache_lines,
            commands::log_parser::get_last_read_date,
            commands::log_parser::clear_cache_lines,
            commands::log_parser::dump_cache_lines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
