// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use auth::AuthState;
use cache::CacheState;
use database::DatabaseClient;
use price_scraper::PriceScraper;
use serde_json::{json, Value};
use settings::SettingsState;
use std::sync::Arc;
use std::{env, sync::Mutex};
use structs::{GlobleError, Invantory, Transaction};
use tauri::async_runtime::block_on;
use tauri::{App, Manager};
use wfm_client::WFMClientState;
mod structs;
mod whisper_scraper;
use whisper_scraper::WhisperScraper; // add this line
mod live_scraper;
use live_scraper::LiveScraper;

mod auth;
mod cache;
mod database;
mod helper;
mod logger;
mod price_scraper;
mod settings;
mod wfm_client;

use helper::WINDOW as HE_WINDOW;

#[tauri::command]
async fn setup(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheState>>>,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Value, GlobleError> {
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let db = db.lock()?.clone();
    db.initialize().await?;
    let cache = cache.lock()?.clone();
    cache.update_cache().await?;

    // Check if the user access token is valid
    // let valid = wfm.validate().await?;
    // if !valid {
    //     return Ok(json!({"valid": false}));
    // }
    Ok(json!({
        "valid": true,
        "settings": &settings.clone(),
        "user": &auth.clone(),
         "inventorys": &db.get_inventorys().await?,
         "transactions": &db.get_transactions().await?,

    }))
}

#[tauri::command]
async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<AuthState, GlobleError> {
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    match wfm.login(email, password).await {
        Ok(user) => {
            user.save_to_file()?;
            return Ok(user.clone());
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
    Ok(auth.clone())
}

#[tauri::command]
async fn create_invantory_entry(
    id: String,
    quantity: i64,
    price: i64,
    rank: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Invantory, GlobleError> {
    let db = db.lock()?.clone();
    let invantory = db
        .create_inventory_entry(id, quantity, price, rank)
        .await
        .unwrap();
    Ok(invantory)
}

#[tauri::command]
async fn create_transaction_entry(
    id: String,
    ttype: String,
    quantity: i64,
    rank: i64,
    price: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Transaction, GlobleError> {
    let db = db.lock()?.clone();
    let transaction = db
        .create_transaction_entry(id, ttype, quantity, rank, price)
        .await
        .unwrap();
    Ok(transaction)
}

#[tauri::command]
async fn delete_invantory_entry(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Option<Invantory>, GlobleError> {
    let db = db.lock()?.clone();
    Ok(db.delete_inventory_entry(id).await?)
}
#[tauri::command]
async fn sell_invantory_entry(
    id: i64,
    price: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Invantory, GlobleError> {
    println!("{:?}", id);
    println!("{:?}", price);
    let db = db.lock()?.clone();
    Ok(db.sell_invantory_entry(id, price).await?)
}

#[tauri::command]
fn toggle_whisper_scraper(
    whisper_scraper: tauri::State<'_, Arc<std::sync::Mutex<WhisperScraper>>>,
) {
    let mut whisper_scraper = whisper_scraper.lock().unwrap();

    if whisper_scraper.is_running() {
        whisper_scraper.stop_loop();
    } else {
        whisper_scraper.start_loop();
    }
}

#[tauri::command]
fn toggle_live_scraper(live_scraper: tauri::State<'_, Arc<std::sync::Mutex<LiveScraper>>>) {
    let mut live_scraper = live_scraper.lock().unwrap();
    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        match live_scraper.start_loop() {
            Ok(_) => {}
            Err(_e) => {
                live_scraper.stop_loop();
            }
        }
    }
}

#[tauri::command]
async fn update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
) -> Result<(), GlobleError> {
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;
    my_lock.volume_threshold = settings.volume_threshold;
    my_lock.range_threshold = settings.range_threshold;
    my_lock.avg_price_cap = settings.avg_price_cap;
    my_lock.max_total_price_cap = settings.max_total_price_cap;
    my_lock.price_shift_threshold = settings.price_shift_threshold;
    my_lock.blacklist = settings.blacklist;
    my_lock.whitelist = settings.whitelist;
    my_lock.strict_whitelist = settings.strict_whitelist;
    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}

#[tauri::command(async)]
async fn generate_price_history(
    days: i64,
    price_scraper: tauri::State<'_, Arc<std::sync::Mutex<PriceScraper>>>,
) -> Result<i64, GlobleError> {
    let price_scraper = price_scraper.lock().unwrap().clone();
    Ok(price_scraper.generate(days).await?)
}
async fn setup_async(
    app: &mut App
) -> Result<Arc<Mutex<DatabaseClient>>, Box<dyn std::error::Error>> {
    // create and manage Settings state
    let settings_arc = Arc::new(Mutex::new(
        SettingsState::setup().expect("Could not setup settings"),
    ));
    app.manage(settings_arc.clone());

    // create and manage Auth state
    let auth_arc = Arc::new(Mutex::new(
        AuthState::setup().expect("Could not setup auth"),
    ));
    app.manage(auth_arc.clone());

    // create and manage Warframe Market API client state
    let wfm_client = Arc::new(Mutex::new(WFMClientState::new(Arc::clone(&auth_arc))));
    app.manage(wfm_client.clone());

    // create and manage Cache state
    let cache_arc = Arc::new(Mutex::new(CacheState::new(Arc::clone(&wfm_client))));
    app.manage(cache_arc.clone());

    // create and manage DatabaseClient state
    let database_client = Arc::new(Mutex::new(DatabaseClient::new(cache_arc).await.unwrap()));
    app.manage(database_client.clone());

    // create and manage PriceScraper state
    let price_scraper: Arc<Mutex<PriceScraper>> = Arc::new(Mutex::new(PriceScraper::new(Arc::clone(&wfm_client), Arc::clone(&auth_arc))));
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
    let whisper_scraper = WhisperScraper::new();
    app.manage(Arc::new(Mutex::new(whisper_scraper)));

    Ok(database_client)
}
fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            // Get the 'main' window and store it
            let window = app.get_window("main").unwrap().clone();
            *HE_WINDOW.lock().unwrap() = Some(window.clone());

            // create and manage DatabaseClient state
            block_on(setup_async(app)).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            setup,
            login,
            update_settings,
            create_invantory_entry,
            create_transaction_entry,
            delete_invantory_entry,
            sell_invantory_entry,
            toggle_whisper_scraper,
            toggle_live_scraper,
            generate_price_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
