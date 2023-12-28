use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use eyre::eyre;
use once_cell::sync::Lazy;
use reqwest::{Client, Method, Url};
use serde_json::{json, Value};
use tokio::process::Command;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("commands.log".to_string()));

use crate::{
    auth::AuthState,
    cache::client::CacheClient,
    database::client::DBClient,
    enums::LogLevel,
    error::{self, AppError},
    handler::MonitorHandler,
    helper, logger,
    price_scraper::PriceScraper,
    settings::SettingsState,
    wf_ee_log_parser::client::EELogParser,
    wfm_client::client::WFMClient,
    PACKAGEINFO,
};

#[tauri::command]
pub async fn init(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    price_scraper: tauri::State<'_, Arc<Mutex<PriceScraper>>>,
    ee_log: tauri::State<'_, Arc<std::sync::Mutex<EELogParser>>>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Value, AppError> {

    let db = db.lock()?.clone();
    let mut ee_log = ee_log.lock()?.clone();
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();
    let price_scraper = price_scraper.lock()?.clone();
    let mut response = json!({
        "settings": &settings.clone(),
        "user": &auth.clone(),
        "price_scraper_last_run": price_scraper.get_status(),
    });

    helper::emit_undate_initializ_status("Loading Database...", None);
    match db.initialize().await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }

    // Load Cache
    helper::emit_undate_initializ_status("Loading Cache...", None);
    match cache.load().await {
        Ok(_) => {
            response["items"] = json!(cache.items().get_types()?);
            response["riven_items"] = json!(cache.riven().get_types()?);
            response["riven_attributes"] = json!(cache.riven().get_attributes()?);
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }

    // Validate Auth
    helper::emit_undate_initializ_status("Validating Credentials...", None);
    let is_validate = match wfm.auth().validate().await {
        Ok(is_validate) => {
            response["valid"] = json!(is_validate);
            is_validate
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // Load Stock Items, Rivens
    helper::emit_undate_initializ_status("Loading Stock...", None);
    // Load Stock Items
    match db.stock_item().get_items().await {
        Ok(items) => {
            response["stock_items"] = json!(items);
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    // Load Stock Rivens
    match db.stock_riven().get_rivens().await {
        Ok(items) => {
            response["stock_rivens"] = json!(items);
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // Load Transactions
    helper::emit_undate_initializ_status("Loading Transactions...", None);
    match db.transaction().get_items().await {
        Ok(transactions) => {
            response["transactions"] = json!(transactions);
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    if is_validate {
        helper::emit_undate_initializ_status("Loading Your Orders...", None);
        let mut ordres_vec = wfm.orders().get_my_orders().await?;
        let mut ordres = ordres_vec.buy_orders;
        ordres.append(&mut ordres_vec.sell_orders);
        response["orders"] = json!(ordres);

        helper::emit_undate_initializ_status("Loading Your Auctions...", None);
        response["auctions"] = json!(wfm.auction().get_my_auctions().await?);

        helper::emit_undate_initializ_status("Loading Your Chats...", None);
        response["chats"] = json!(wfm.chat().get_chats().await?);
    }

    // Check for updates
    helper::emit_undate_initializ_status("Checking for updates...", None);
    response["app_info"] = helper::get_app_info().await?;

    // Start EE Log Parser
    if !ee_log.is_running() {
        ee_log.start_loop();
    }

    Ok(response)
}

#[tauri::command]
pub async fn update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;

    // Set Live Scraper Settings
    my_lock.live_scraper = settings.live_scraper;

    // Set Whisper Scraper Settings
    my_lock.whisper_scraper = settings.whisper_scraper;

    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}

#[tauri::command]
pub async fn open_logs_folder() {
    Command::new("explorer")
        .args(["/select,", &logger::get_log_forlder().to_str().unwrap()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}



#[tauri::command]
pub fn show_notification(
    title: String,
    message: String,
    icon: Option<String>,
    sound: Option<String>,
    mh: tauri::State<'_, Arc<std::sync::Mutex<MonitorHandler>>>,
) {
    let mh = mh.lock().unwrap();
    mh.show_notification(
        &title,
        &message,
        Some("https://i.imgur.com/UggEVVI.jpeg"),
        sound.as_deref(),
    );
}

#[tauri::command]
pub fn log(
    component: String,
    msg: String,
    level: LogLevel,
    console: Option<bool>,
    file: Option<&str>,
) {
    let (console, file) = match (console, file) {
        (Some(console), Some(file)) => (console, Some(file)),
        (Some(console), None) => (console, None),
        (None, Some(file)) => (false, Some(file)),
        (None, None) => (false, None),
    };
    logger::dolog(level, &component, &msg, console, file);
}

#[tauri::command]
pub fn export_logs(mh: tauri::State<'_, Arc<std::sync::Mutex<MonitorHandler>>>) {
    logger::export_logs();
    show_notification(
        "Logs Exported".to_string(),
        "Logs exported to desktop".to_string(),
        None,
        None,
        mh
    );
}
