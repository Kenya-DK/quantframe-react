// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use auth::AuthState;
use cache::CacheState;
use database::DatabaseClient;
use database2::client::DBClient;
use debug::DebugClient;
use error::AppError;
use price_scraper::PriceScraper;
use settings::SettingsState;
use std::panic;
use std::sync::Arc;
use std::{env, sync::Mutex};
use tauri::async_runtime::block_on;
use tauri::{App, Manager};
mod structs;
mod whisper_scraper;
use whisper_scraper::WhisperScraper; // add this line
mod live_scraper;
use live_scraper::LiveScraper;

mod auth;
mod cache;
mod commands;
mod database;
mod database2;
mod debug;
mod error;
mod helper;
mod logger;
mod price_scraper;
mod settings;
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
    let cache_arc = Arc::new(Mutex::new(CacheState::new(Arc::clone(&wfm_client))));
    app.manage(cache_arc.clone());

    // create and manage DatabaseClient state
    let database_client = Arc::new(Mutex::new(
        DatabaseClient::new(cache_arc.clone(), wfm_client.clone())
            .await
            .unwrap(),
    ));
    app.manage(database_client.clone());

    let database_client2 = Arc::new(Mutex::new(
        DBClient::new(cache_arc.clone(), wfm_client.clone())
            .await
            .unwrap(),
    ));
    app.manage(database_client2.clone());

    // create and manage PriceScraper state
    let price_scraper: Arc<Mutex<PriceScraper>> = Arc::new(Mutex::new(PriceScraper::new(
        Arc::clone(&wfm_client),
        Arc::clone(&auth_arc),
    )));
    app.manage(price_scraper.clone());

    // create and manage LiveScraper state
    let live_scraper = LiveScraper::new(
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
        println!("Panic: {:?}", panic_info);
        //  Do something with backtrace and panic_info.
    }));
    tauri::Builder::default()
        .setup(move |app| {
            // Get the 'main' window and store it
            let window = app.get_window("main").unwrap().clone();
            *HE_WINDOW.lock().unwrap() = Some(window.clone());

            // create and manage DatabaseClient state
            match block_on(setup_async(app)) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
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
            commands::auth::login,
            commands::base::update_settings,
            commands::inventory::create_invantory_entry,
            commands::transaction::create_transaction_entry,
            commands::inventory::delete_invantory_entry,
            commands::inventory::sell_invantory_entry,
            commands::whisper_scraper::toggle_whisper_scraper,
            commands::live_scraper::toggle_live_scraper,
            commands::price_scraper::generate_price_history,
            commands::debug::import_warframe_algo_trader_data,
            commands::debug::reset_data,
            commands::orders::get_orders,
            commands::orders::delete_order,
            commands::orders::create_order,
            commands::orders::update_order
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
