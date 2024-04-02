// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use auth::AuthState;
use cache::client::CacheClient;
use database::client::DBClient;
use debug::DebugClient;
use live_scraper::client::LiveScraperClient;
use notification::client::NotifyClient;
use once_cell::sync::Lazy;
use settings::SettingsState;
use utils::modules::error::AppError;
use utils::modules::logger;

use std::panic;
use std::sync::Arc;
use std::{env, sync::Mutex};

use tauri::async_runtime::block_on;
use tauri::{App, Manager, PackageInfo, SystemTrayEvent};
mod utils;
use tauri::SystemTray;

mod auth;
mod cache;
mod commands;
mod database;
mod debug;
mod helper;
mod live_scraper;
mod notification;
mod qf_client;
mod settings;
mod system_tray;
mod wfm_client;
// mod utils;

use helper::WINDOW as HE_WINDOW;

pub static PACKAGEINFO: Lazy<Mutex<Option<PackageInfo>>> = Lazy::new(|| Mutex::new(None));

async fn setup_async(app: &mut App) -> Result<(), AppError> {

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
    )));
    app.manage(wfm_client.clone());

    // create and manage Quantframe client state
    let qf_client = Arc::new(Mutex::new(qf_client::client::QFClient::new(
        Arc::clone(&auth_arc),
        Arc::clone(&settings_arc),
    )));
    app.manage(qf_client.clone());

    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheClient::new(
        Arc::clone(&wfm_client),
        Arc::clone(&qf_client),
    )));
    app.manage(cache_arc.clone());

    // create and manage DatabaseClient state
    let database_client = Arc::new(Mutex::new(
        DBClient::new(cache_arc.clone(), wfm_client.clone())
            .await
            .unwrap(),
    ));
    app.manage(database_client.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraperClient::new(
        Arc::clone(&settings_arc),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&database_client),
        Arc::clone(&cache_arc),
        Arc::clone(&notify_arc),
    );
    app.manage(Arc::new(Mutex::new(live_scraper)));

    // create and manage WhisperScraper state
    let debug_client = DebugClient::new(Arc::clone(&cache_arc), Arc::clone(&database_client));
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
        .plugin(tauri_plugin_websocket::init())
        .system_tray(SystemTray::new().with_menu(system_tray::client::get_tray_menu()))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                system_tray::client::get_tray_event(id);
            }
            _ => {}
        })
        .setup(move |app| {
            // Get the main window
            let window = app.get_window("main").unwrap().clone();

            // Get the 'main' window and store it
            *HE_WINDOW.lock().unwrap() = Some(window.clone());

            // Get the package info and store it
            *PACKAGEINFO.lock().unwrap() = Some(app.package_info().clone());

            // create and manage DatabaseClient state
            match block_on(setup_async(app)) {
                Ok(_) => {}
                Err(e) => {
                    let component = e.component();
                    let cause = e.cause();
                    let backtrace = e.backtrace();
                    let log_level = e.log_level();
                    crate::logger::dolog(
                        log_level,
                        component.as_str(),
                        format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
                        true,
                        Some("setup_error.log"),
                    );
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Base commands
            commands::base::init,
            commands::base::log,
            commands::base::update_settings,
            commands::base::open_logs_folder,
            commands::base::export_logs,
            commands::base::show_notification,
            commands::base::on_new_wfm_message,
            // Auth commands
            commands::auth::login,
            commands::auth::logout,
            commands::auth::update_user_status,
            // Transaction commands
            commands::transaction::create_transaction_entry,
            commands::transaction::delete_transaction_entry,
            commands::transaction::update_transaction_entry,
            // Live Scraper commands
            commands::live_scraper::toggle_live_scraper,
            // Auctions commands
            commands::auctions::refresh_auctions,
            // Orders commands
            commands::orders::refresh_orders,
            commands::orders::get_orders,
            commands::orders::delete_order,
            commands::orders::create_order,
            commands::orders::update_order,
            commands::orders::delete_all_orders,
            // Chat commands
            commands::chat::get_chat,
            commands::chat::delete_chat,
            commands::chat::refresh_chats,
            // Stock commands
            commands::stock::create_item_stock,
            commands::stock::delete_item_stock,
            commands::stock::update_item_stock,
            commands::stock::sell_item_stock,
            commands::stock::sell_item_stock_by_url,
            commands::stock::create_riven_stock,
            commands::stock::import_auction,
            commands::stock::delete_riven_stock,
            commands::stock::update_riven_stock,
            commands::stock::sell_riven_stock,
            // Warframe Market Commands
            wfm_client::modules::auction::auction_search,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
