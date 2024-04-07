// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::client::AppState;
use auth::AuthState;
use cache::client::CacheClient;
use debug::DebugClient;
use live_scraper::client::LiveScraperClient;
use migration::{Migrator, MigratorTrait};
use notification::client::NotifyClient;
use service::sea_orm::Database;
use settings::SettingsState;
use utils::modules::error::AppError;
use utils::modules::logger;

use std::panic;
use std::sync::Arc;
use std::{env, sync::Mutex};

use tauri::async_runtime::block_on;
use tauri::SystemTray;
use tauri::{App, Manager, SystemTrayEvent};

mod app;
mod auth;
mod cache;
mod enums;
mod helper;
mod notification;
mod qf_client;
mod settings;
mod utils;
mod wfm_client;
mod commands;
mod debug;
mod live_scraper;
mod system_tray;

async fn setup_manages(app: &mut App) -> Result<(), AppError> {
    // Create the database connection and store it
    let db_path = helper::get_app_storage_path();

    let db_url = format!(
        "sqlite://{}/{}",
        db_path.to_str().unwrap(),
        "quantframeV2.sqlite?mode=rwc"
    );

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // Create and manage Notification state
    let app_arc: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(conn, app.handle())));
    app.manage(app_arc.clone());

    // Create and manage Notification state
    let notify_arc: Arc<Mutex<NotifyClient>> =
        Arc::new(Mutex::new(NotifyClient::new(app.handle().clone())));
    app.manage(notify_arc.clone());

    // create and manage Settings state
    let settings_arc = Arc::new(Mutex::new(SettingsState::setup()?));
    app.manage(settings_arc.clone());

    // create and manage Auth state
    let auth_arc = Arc::new(Mutex::new(AuthState::setup()?));
    app.manage(auth_arc.clone());

    // create and manage Warframe Market API client state
    let wfm_client = Arc::new(Mutex::new(wfm_client::client::WFMClient::new(
        Arc::clone(&auth_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&app_arc),
    )));
    app.manage(wfm_client.clone());

    // create and manage Quantframe client state
    let qf_client = Arc::new(Mutex::new(qf_client::client::QFClient::new(
        Arc::clone(&auth_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&app_arc),
    )));
    app.manage(qf_client.clone());

    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheClient::new(
        Arc::clone(&wfm_client),
        Arc::clone(&qf_client),
    )));
    app.manage(cache_arc.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraperClient::new(
        Arc::clone(&app_arc),
        Arc::clone(&settings_arc),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&cache_arc),
        Arc::clone(&notify_arc),
    );
    app.manage(Arc::new(Mutex::new(live_scraper)));

    // create and manage WhisperScraper state
    let debug_client = DebugClient::new(Arc::clone(&cache_arc));
    app.manage(Arc::new(Mutex::new(debug_client)));

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

    tauri::Builder::default()
        // .plugin(tauri_plugin_websocket::init())
        .system_tray(SystemTray::new().with_menu(system_tray::client::get_tray_menu()))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                system_tray::client::get_tray_event(id);
            }
            _ => {}
        })
        .setup(move |app| {
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
            commands::base::init,
            // commands::base::log,
            // commands::base::update_settings,
            // commands::base::open_logs_folder,
            // commands::base::export_logs,
            // commands::base::show_notification,
            // commands::base::on_new_wfm_message,
            // // Auth commands
            // commands::auth::login,
            // commands::auth::logout,
            // commands::auth::update_user_status,
            // // Transaction commands
            // commands::transaction::tra_get_all,
            // commands::transaction::tra_get_by_id,
            // commands::transaction::tra_update_by_id,
            // commands::transaction::tra_delete_by_id,
            // // Stock Item commands
            // commands::stock_item::stock_item_get_all,
            // commands::stock_item::stock_item_get_by_id,
            // commands::stock_item::stock_item_create,
            // commands::stock_item::stock_item_update,
            // commands::stock_item::stock_item_delete,
            // // Stock Riven commands
            // commands::stock_riven::stock_riven_get_all,
            // commands::stock_riven::stock_riven_get_by_id,
            // commands::stock_riven::stock_riven_create,
            // commands::stock_riven::stock_riven_update,
            // commands::stock_riven::stock_riven_delete,
            // // Live Scraper commands
            // commands::live_scraper::toggle_live_scraper,
            // // Auctions commands
            // commands::auctions::refresh_auctions,
            // // Orders commands
            // commands::orders::refresh_orders,
            // commands::orders::get_orders,
            // commands::orders::delete_order,
            // commands::orders::create_order,
            // commands::orders::update_order,
            // commands::orders::delete_all_orders,
            // // Chat commands
            // commands::chat::get_chat,
            // commands::chat::delete_chat,
            // commands::chat::refresh_chats,
            // // Stock commands
            // commands::stock::create_item_stock,
            // commands::stock::delete_item_stock,
            // commands::stock::update_item_stock,
            // commands::stock::sell_item_stock,
            // commands::stock::sell_item_stock_by_url,
            // commands::stock::create_riven_stock,
            // commands::stock::import_auction,
            // commands::stock::delete_riven_stock,
            // commands::stock::update_riven_stock,
            // commands::stock::sell_riven_stock,
            // // Warframe Market Commands
            // wfm_client::modules::auction::auction_search,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
