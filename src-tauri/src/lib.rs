// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![allow(non_snake_case)]
#![allow(deprecated)]
use ::utils::clear_logs;
use ::utils::critical;
use ::utils::error;
use ::utils::get_location;
use ::utils::info;
use ::utils::init_logger;
use ::utils::set_base_path;
use ::utils::warning;
use ::utils::Error;
use ::utils::LoggerOptions;
use app::client::AppState;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::{Database, DatabaseConnection};

use std::env;
use std::panic;
use std::sync::{Mutex, OnceLock};
use tauri::{Emitter, Manager};

use crate::cache::client::CacheState;
use crate::live_scraper::LiveScraperState;
use crate::log_parser::init_detections;
use crate::log_parser::LogParserState;

mod app;
mod cache;
mod commands;
mod handlers;
mod http_server;
mod macros;
mod utils;

mod enums;
mod helper;
mod live_scraper;
mod log_parser;
mod types;

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
        &LoggerOptions::default(),
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
            &LoggerOptions::default(),
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
    match Migrator::up(&conn, None).await {
        Ok(_) => info(
            "Setup:Database",
            "Database migrations applied successfully",
            &LoggerOptions::default(),
        ),
        Err(e) => {
            return Err(Error::new(
                "InitDatabase",
                format!("Failed to apply database migrations: {}", e),
                get_location!(),
            ));
        }
    }
    DATABASE.get_or_init(|| conn);
    Ok(())
}

async fn setup_manages(app: tauri::AppHandle, use_temp_db: bool) -> Result<(), Error> {
    init_detections();
    // Clear the logs older then 7 days
    clear_logs(7)?;

    // Clone the fields needed for CacheState before moving app_state
    let app_state = AppState::new(app.clone(), use_temp_db, false).await;
    let qf_client = app_state.qf_client.clone();
    let settings = app_state.settings.clone();
    let user = app_state.user.clone();
    app.manage(Mutex::new(app_state));

    let cache_state = Mutex::new(CacheState::new(&qf_client, &user, &settings.lang).await?);
    app.manage(cache_state);

    let live_scraper_state = LiveScraperState::new();
    app.manage(live_scraper_state);

    app.manage(Mutex::new(LogParserState::new(&settings.advanced_settings)));
    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let use_temp_db = false;

    // Initialize the logger for elapsed time tracking

    panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        critical(
            "Panic",
            format!("Panic: {:?}", panic_info).as_str(),
            &LoggerOptions::default().set_file("panic.log"),
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
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            // log_parser::types::trade_detection::init_detections();
            APP.get_or_init(|| app.handle().clone());
            // Clone the handle for async task
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let mut err = None;
                if let Err(e) = init_database(use_temp_db).await {
                    err = Some(e.clone());
                    e.log("init_database_error.log");
                }
                if let Err(e) = setup_manages(app_handle.clone(), use_temp_db).await {
                    err = Some(e.clone());
                    e.log("setup_error.log");
                }
                if let Err(e) = app_handle.emit("app:ready", ()) {
                    error(
                        "Emit",
                        &format!("Failed to emit app:ready event: {:?}", e),
                        &LoggerOptions::default().set_file("emit_error.log"),
                    );
                }
                HAS_STARTED.set(true).unwrap();
                if let Some(e) = err {
                    emit_error!(e);
                }
            });
            init_logger();
            set_base_path(helper::get_app_storage_path().to_str().unwrap());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Base commands
            commands::dashboard::dashboard_summary,
            commands::app::initialized,
            commands::app::app_get_app_info,
            commands::app::app_get_settings,
            commands::app::app_update_settings,
            commands::app::app_exit,
            commands::app::app_accept_tos,
            commands::app::app_notify_reset,
            // Auth commands
            commands::auth::auth_me,
            commands::auth::auth_login,
            commands::auth::auth_logout,
            commands::auth::auth_has_permission,
            // User commands
            commands::user::user_set_status,
            // Analytics commands
            commands::analytics::analytics_add_metric,
            commands::analytics::analytics_set_last_user_activity,
            // Alert commands
            commands::alert::alert_get_alerts,
            // Cache commands
            commands::cache::cache_get_tradable_items,
            commands::cache::cache_get_riven_attributes,
            commands::cache::cache_get_riven_weapons,
            commands::cache::cache_get_theme_presets,
            commands::cache::cache_create_theme,
            commands::cache::cache_open_theme_folder,
            commands::cache::cache_get_chat_link,
            commands::cache::cache_get_chat_icons,
            // Log commands
            commands::logs::log_export,
            // Live Scraper commands
            commands::live_scraper::live_scraper_get_state,
            commands::live_scraper::live_scraper_toggle,
            commands::live_scraper::live_scraper_get_interesting_wtb_items,
            // Stock Item commands
            commands::stock_item::get_stock_item_pagination,
            commands::stock_item::get_stock_item_financial_report,
            commands::stock_item::get_stock_item_status_counts,
            commands::stock_item::stock_item_create,
            commands::stock_item::stock_item_delete,
            commands::stock_item::stock_item_sell,
            commands::stock_item::stock_item_update,
            commands::stock_item::stock_item_get_by_id,
            commands::stock_item::stock_item_update_multiple,
            commands::stock_item::stock_item_delete_multiple,
            commands::stock_item::export_stock_item_json,
            // Stock Riven commands
            commands::stock_riven::get_stock_riven_pagination,
            commands::stock_riven::get_stock_riven_financial_report,
            commands::stock_riven::get_stock_riven_status_counts,
            commands::stock_riven::stock_riven_create,
            commands::stock_riven::stock_riven_delete,
            commands::stock_riven::stock_riven_sell,
            commands::stock_riven::stock_riven_update,
            commands::stock_riven::stock_riven_get_by_id,
            commands::stock_riven::export_stock_riven_json,
            commands::stock_riven::stock_riven_update_multiple,
            commands::stock_riven::stock_riven_delete_multiple,
            // Debug commands
            commands::debug::debug_get_wfm_state,
            // Order commands
            commands::order::get_wfm_orders_pagination,
            commands::order::get_wfm_orders_status_counts,
            commands::order::order_refresh,
            commands::order::order_delete_all,
            commands::order::order_delete_by_id,
            commands::order::get_wfm_order_by_id,
            // Auction commands
            commands::auction::auction_refresh,
            commands::auction::get_wfm_auctions_pagination,
            commands::auction::get_wfm_auctions_overview,
            commands::auction::auction_delete_all,
            commands::auction::auction_delete_by_id,
            commands::auction::auction_import_by_id,
            // Wish List commands
            commands::wish_list::get_wish_list_pagination,
            commands::wish_list::get_wish_list_financial_report,
            commands::wish_list::get_wish_list_status_counts,
            commands::wish_list::wish_list_create,
            commands::wish_list::wish_list_bought,
            commands::wish_list::wish_list_delete,
            commands::wish_list::wish_list_update,
            commands::wish_list::wish_list_get_by_id,
            commands::wish_list::export_wish_list_json,
            commands::wish_list::wish_list_update_multiple,
            commands::wish_list::wish_list_delete_multiple,
            // Chat commands
            commands::chat::chat_refresh,
            commands::chat::get_chat_pagination,
            commands::chat::chat_get_messages_by_id,
            commands::chat::chat_delete,
            commands::chat::chat_set_active,
            commands::chat::chat_send_message,
            // Transaction commands
            commands::transaction::get_transaction_pagination,
            commands::transaction::get_transaction_financial_report,
            commands::transaction::transaction_update,
            commands::transaction::transaction_delete,
            commands::transaction::transaction_delete_bulk,
            commands::transaction::export_transaction_json,
            // Item commands
            commands::item::item_prices_lookup,
            commands::item::export_item_price_data,
            // Riven commands
            commands::riven::riven_prices_lookup,
            commands::riven::export_riven_price_data,
            // Market commands
            commands::market::get_user_activity,
            // Trade Entry commands
            // Trade Entry commands
            commands::trade_entry::get_trade_entry_pagination,
            commands::trade_entry::trade_entry_get_by_id,
            commands::trade_entry::trade_entry_create,
            commands::trade_entry::trade_entry_create_multiple,
            commands::trade_entry::trade_entry_delete,
            commands::trade_entry::trade_entry_delete_multiple,
            commands::trade_entry::trade_entry_update,
            commands::trade_entry::trade_entry_update_multiple,
            commands::trade_entry::export_trade_entry_json,
            // Warframe GDPR commands
            commands::warframe_gdpr::wfgdpr_get_state,
            commands::warframe_gdpr::wfgdpr_load,
            commands::warframe_gdpr::wfgdpr_get_trades_pagination,
            commands::warframe_gdpr::wfgdpr_get_trades_financial_report,
            commands::warframe_gdpr::wfgdpr_get_purchases_pagination,
            commands::warframe_gdpr::wfgdpr_get_logins_pagination,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
