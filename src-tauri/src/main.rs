// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::client::AppState;
use auth::AuthState;
use cache::client::CacheClient;
use debug::DebugClient;
use live_scraper::client::LiveScraperClient;
use log_parser::client::LogParser;
use migration::{Migrator, MigratorTrait};
use notification::client::NotifyClient;
use service::sea_orm::Database;
use settings::SettingsState;
use utils::modules::error::AppError;
use utils::modules::logger;

use std::panic;
use std::sync::{Arc, OnceLock};
use std::{env, sync::Mutex};

use tauri::async_runtime::block_on;
use tauri::SystemTray;
use tauri::{App, Manager, SystemTrayEvent};

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
mod system_tray;
mod utils;
mod wfm_client;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

async fn setup_manages(app: &mut App) -> Result<(), AppError> {
    // Get the update channel
    let context = tauri::generate_context!();
    let updater = context.config_mut().tauri.updater;

    let mut is_pre_release = false;
    for arg in updater.endpoints {
        if arg.contains("prerelease") {
            is_pre_release = true;
        }
    }


    // Check if the app is being run for the first time
    let is_first_install = !helper::dose_app_exist();

    // Create the database connection and store it
    let storage_path = helper::get_app_storage_path();

    let mut db_file_name = "quantframeV2.sqlite";
    let db_debug_file_name = "quantframeV2_debug.sqlite";
    let debug_db = true;

    // Create the path to the database file
    let db_file_path = format!("{}/{}", storage_path.to_str().unwrap(), db_file_name);

    // Create a backup of the database file
    let db_file_path_backup = format!("{}/{}_backup", storage_path.to_str().unwrap(), db_file_name);
    logger::info_con("Setup:Database", "Creating a backup of the database file");
    if std::path::Path::new(&db_file_path).exists() {
        std::fs::copy(&db_file_path, &db_file_path_backup)
            .expect("Failed to create a backup of the database file");
    }

    // Create the path to the debug database file
    let db_debug_file_path_backup = db_file_path.replace(db_file_name, db_debug_file_name);
    if debug_db {
        db_file_name = db_debug_file_name;
        logger::warning_con(
            "Setup:Database",
            "Debug mode is enabled, using the debug database file no data wil be saved",
        );
        if std::path::Path::new(&db_file_path).exists() {
            std::fs::copy(&db_file_path, &db_debug_file_path_backup)
                .expect("Failed to create a backup of the database file");
        }
    }

    // Create the database connection URL
    let db_url = format!(
        "sqlite://{}/{}?mode=rwc",
        storage_path.to_str().unwrap(),
        db_file_name,
    );

    // Create the database connection and store it and run the migrations
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // Create and manage Notification state
    let app_arc: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(
        conn,
        app.handle(),
        is_first_install,
    )));
    app.manage(app_arc.clone());

    // Create and manage Notification state
    let notify_arc: Arc<Mutex<NotifyClient>> = Arc::new(Mutex::new(NotifyClient::new(
        Arc::clone(&app_arc),
        app.handle().clone(),
    )));
    app.manage(notify_arc.clone());

    // create and manage Settings state
    let settings_arc = Arc::new(Mutex::new(SettingsState::setup()?));
    app.manage(settings_arc.clone());

    // create and manage Auth state
    let auth_arc = Arc::new(Mutex::new(AuthState::setup()?));
    app.manage(auth_arc.clone());
    
    // create and manage Quantframe client state
    let qf_client = Arc::new(Mutex::new(qf_client::client::QFClient::new(
        Arc::clone(&auth_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&app_arc),
    )));
    app.manage(qf_client.clone());

    // create and manage Warframe Market API client state
    let wfm_client = Arc::new(Mutex::new(wfm_client::client::WFMClient::new(
        Arc::clone(&auth_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&app_arc),
        Arc::clone(&qf_client),
    )));
    app.manage(wfm_client.clone());


    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheClient::new(
        Arc::clone(&qf_client),
        Arc::clone(&settings_arc),
    )));
    app.manage(cache_arc.clone());

    // create and manage HTTP client state
    let http_client_arc = Arc::new(Mutex::new(http_client::client::HttpClient::setup(
        Arc::clone(&settings_arc),
    )?));
    app.manage(http_client_arc.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraperClient::new(
        Arc::clone(&app_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&cache_arc),
        Arc::clone(&notify_arc),
        Arc::clone(&qf_client),
    );
    app.manage(Arc::new(Mutex::new(live_scraper)));

    // create and manage WhisperScraper state
    let debug_client = DebugClient::new(Arc::clone(&cache_arc), Arc::clone(&notify_arc));
    app.manage(Arc::new(Mutex::new(debug_client)));

    let log_parser = LogParser::new(
        Arc::clone(&app_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&cache_arc),
        Arc::clone(&notify_arc),
        Arc::clone(&qf_client),
    );
    app.manage(Arc::new(Mutex::new(log_parser)));
    Ok(())
}
fn main() {
    panic::set_hook(Box::new(|panic_info| {
        logger::critical(
            "Panic",
            format!("Panic: {:?}", panic_info).as_str(),
            true,
            Some("panic.log"),
        );
    }));

    // Delete Folder
    // helper::delete_app_folder();

    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .system_tray(SystemTray::new().with_menu(system_tray::client::get_tray_menu()))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                system_tray::client::get_tray_event(id);
            }
            _ => {}
        })
        .setup(move |app| {
            _ = APP.get_or_init(|| app.app_handle());
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
                        true,
                        Some("setup_error.log"),
                    );
                }
            };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Base commands
            commands::app::app_init,
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
            commands::debug::debug_import_algo_trader,
            // Log commands
            commands::log::log_open_folder,
            commands::log::log_export,
            // Auctions commands
            commands::auctions::auction_refresh,
            commands::auctions::auction_delete,
            commands::auctions::auction_delete_all,
            // Orders commands
            commands::orders::order_delete,
            commands::orders::order_delete_all,
            commands::orders::order_refresh,
            // Chat commands
            commands::chat::chat_refresh,
            // Live Trading commands
            commands::live_scraper::live_scraper_set_running_state,
            // Stock Item commands
            commands::stock_item::stock_item_reload,
            commands::stock_item::stock_item_create,
            commands::stock_item::stock_item_update,
            commands::stock_item::stock_item_update_bulk,
            commands::stock_item::stock_item_sell,
            commands::stock_item::stock_item_sell_by_wfm_order,
            commands::stock_item::stock_item_delete,
            commands::stock_item::stock_item_delete_bulk,
            // Stock Riven commands
            commands::stock_riven::stock_riven_reload,
            commands::stock_riven::stock_riven_update,
            commands::stock_riven::stock_riven_update_bulk,
            commands::stock_riven::stock_riven_sell,
            commands::stock_riven::stock_riven_delete,
            commands::stock_riven::stock_riven_delete_bulk,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
