// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use auth::AuthState;
use cache::client::CacheClient;
use database::client::DBClient;
use debug::DebugClient;
use error::AppError;
use live_scraper::client::LiveScraperClient;
use price_scraper::PriceScraper;
use settings::SettingsState;
use std::panic;
use std::sync::Arc;
use std::{env, sync::Mutex};
use tauri::async_runtime::block_on;
use tauri::{App, CustomMenuItem, Manager, SystemTrayEvent};
mod structs;
mod whisper_scraper;
use tauri::{SystemTray, SystemTrayMenu};
use whisper_scraper::WhisperScraper; // add this line

mod auth;
mod cache;
mod commands;
mod database;
mod debug;
mod error;
mod helper;
mod live_scraper;
mod logger;
mod price_scraper;
mod rate_limiter;
mod settings;
mod system_tray;
mod wfm_client;

use helper::WINDOW as HE_WINDOW;

async fn setup_async(app: &mut App) -> Result<(), AppError> {
    // create and manage Settings state
    // let se=SettingsState::setup()?;
    let settings_arc = Arc::new(Mutex::new(SettingsState::setup()?));
    app.manage(settings_arc.clone());

    // create and manage Auth state
    let auth_arc = Arc::new(Mutex::new(AuthState::setup()?));
    app.manage(auth_arc.clone());

    // create and manage Warframe Market API client state
    let wfm_client = Arc::new(Mutex::new(wfm_client::client::WFMClient::new(Arc::clone(
        &auth_arc,
    ))));
    app.manage(wfm_client.clone());

    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheClient::new(Arc::clone(&wfm_client))));
    app.manage(cache_arc.clone());

    // create and manage DatabaseClient state
    let database_client = Arc::new(Mutex::new(
        DBClient::new(cache_arc.clone(), wfm_client.clone())
            .await
            .unwrap(),
    ));
    app.manage(database_client.clone());

    // create and manage PriceScraper state
    let price_scraper: Arc<Mutex<PriceScraper>> = Arc::new(Mutex::new(PriceScraper::new(
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
    )));
    app.manage(price_scraper.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraperClient::new(
        Arc::clone(&settings_arc),
        Arc::clone(&price_scraper),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&database_client),
    );
    app.manage(Arc::new(Mutex::new(live_scraper)));

    // create and manage WhisperScraper state
    let whisper_scraper = WhisperScraper::new(Arc::clone(&settings_arc));
    app.manage(Arc::new(Mutex::new(whisper_scraper)));

    // create and manage WhisperScraper state
    let debug_client = DebugClient::new(
        Arc::clone(&cache_arc),
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
        Arc::clone(&database_client),
        Arc::clone(&settings_arc),
    );
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
        .system_tray(SystemTray::new().with_menu(system_tray::client::get_tray_menu()))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                system_tray::client::get_tray_event(id);
            }
            _ => {}
        })
        .setup(move |app| {
            // Get the 'main' window and store it
            let window = app.get_window("main").unwrap().clone();
            *HE_WINDOW.lock().unwrap() = Some(window.clone());

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
            commands::base::init,
            commands::base::update_settings,
            commands::base::get_weekly_rivens,
            commands::auth::login,
            commands::transaction::create_transaction_entry,
            commands::transaction::delete_transaction_entry,
            commands::transaction::update_transaction_entry,
            commands::whisper_scraper::toggle_whisper_scraper,
            commands::live_scraper::toggle_live_scraper,
            commands::price_scraper::generate_price_history,
            commands::debug::import_warframe_algo_trader_data,
            commands::debug::reset_data,
            commands::auctions::refresh_auctions,
            commands::orders::refresh_orders,
            commands::orders::get_orders,
            commands::orders::delete_order,
            commands::orders::create_order,
            commands::orders::update_order,
            // Stock commands
            commands::stock::create_item_stock,
            commands::stock::delete_item_stock,
            commands::stock::sell_item_stock,
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
