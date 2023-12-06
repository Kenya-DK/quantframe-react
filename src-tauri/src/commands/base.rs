use std::sync::{Arc, Mutex};

use eyre::eyre;
use once_cell::sync::Lazy;
use reqwest::{Client, Method, Url};
use serde_json::{json, Value};
use tokio::process::Command;

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("commands.log".to_string()));

use crate::{
    auth::AuthState,
    cache::client::CacheClient,
    database::client::DBClient,
    error::{self, AppError},
    helper, logger,
    price_scraper::PriceScraper,
    settings::SettingsState,
    wf_ee_log_parser::client::EELogParser,
    wfm_client::client::WFMClient,
    PACKAGEINFO, handler::MonitorHandler,
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
    response["app_info"] = get_app_info().await?;

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
    my_lock.live_scraper.webhook = settings.live_scraper.webhook;

    // Stock Item
    my_lock.live_scraper.stock_item.volume_threshold =
        settings.live_scraper.stock_item.volume_threshold;
    my_lock.live_scraper.stock_item.range_threshold =
        settings.live_scraper.stock_item.range_threshold;
    my_lock.live_scraper.stock_item.avg_price_cap = settings.live_scraper.stock_item.avg_price_cap;
    my_lock.live_scraper.stock_item.max_total_price_cap =
        settings.live_scraper.stock_item.max_total_price_cap;
    my_lock.live_scraper.stock_item.price_shift_threshold =
        settings.live_scraper.stock_item.price_shift_threshold;
    my_lock.live_scraper.stock_item.blacklist = settings.live_scraper.stock_item.blacklist;
    my_lock.live_scraper.stock_item.whitelist = settings.live_scraper.stock_item.whitelist;
    my_lock.live_scraper.stock_item.strict_whitelist =
        settings.live_scraper.stock_item.strict_whitelist;
    my_lock.live_scraper.stock_item.order_mode = settings.live_scraper.stock_item.order_mode;
    my_lock.live_scraper.stock_item.report_to_wfm = settings.live_scraper.stock_item.report_to_wfm;
    my_lock.live_scraper.stock_item.auto_trade = settings.live_scraper.stock_item.auto_trade;

    // Stock Riven
    my_lock.live_scraper.stock_riven.range_threshold =
        settings.live_scraper.stock_riven.range_threshold;

    // Set Whisper Scraper Settings
    my_lock.whisper_scraper.enable = settings.whisper_scraper.enable;
    my_lock.whisper_scraper.ping_on_notif = settings.whisper_scraper.ping_on_notif;
    my_lock.whisper_scraper.webhook = settings.whisper_scraper.webhook;
    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}

#[tauri::command]
pub async fn get_weekly_rivens() -> Result<serde_json::Value, AppError> {
    let url = "https://n9e5v4d8.ssl.hwcdn.net/repos/weeklyRivensPC.json";
    let client = Client::new();
    let request = client.request(Method::GET, Url::parse(&url).unwrap());
    let response = request.send().await;
    if let Err(e) = response {
        return Err(AppError::new("WeeklyRivens", eyre!(e.to_string())));
    }
    let response_data = response.unwrap();
    let status = response_data.status();

    if status != 200 {
        return Err(AppError::new(
            "WeeklyRivens",
            eyre!(
                "Could not get weekly rivens. Status: {}",
                status.to_string()
            ),
        ));
    }
    let response = response_data.json::<Value>().await.unwrap();
    Ok(response)
}

#[tauri::command]
pub async fn open_logs_folder() {
    Command::new("explorer")
        .args(["/select,", &logger::get_log_forlder().to_str().unwrap()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

pub async fn get_app_info() -> Result<serde_json::Value, AppError> {
    let packageinfo = PACKAGEINFO
        .lock()
        .unwrap()
        .clone()
        .expect("Could not get package info");
    let version = packageinfo.version.to_string();
    let url = "https://raw.githubusercontent.com/Kenya-DK/quantframe-react/main/src-tauri/tauri.conf.json";
    let client = Client::new();
    let request = client.request(Method::GET, Url::parse(&url).unwrap());
    let response = request.send().await;
    if let Err(e) = response {
        return Err(AppError::new("CHECKFORUPDATES", eyre!(e.to_string())));
    }
    let response_data = response.unwrap();
    let status = response_data.status();

    if status != 200 {
        return Err(AppError::new(
            "CHECKFORUPDATES",
            eyre!("Could not get package.json. Status: {}", status.to_string()),
        ));
    }
    let response = response_data.json::<Value>().await.unwrap();

    let current_version_str = response["package"]["version"].as_str().unwrap();
    let current_version = current_version_str.replace(".", "");
    let current_version = current_version.parse::<i32>().unwrap();

    let version_str = version;
    let version = version_str.replace(".", "").parse::<i32>().unwrap();

    let update_state = json!({
        "update_available": current_version > version,
        "version": current_version_str,
        "current_version": version_str,
        "release_notes": "New version available",
        "download_url": "https://github.com/Kenya-DK/quantframe-react/releases",
    });

    let rep = json!({
        "app_name": packageinfo.name,
        "app_description": packageinfo.description,
        "app_author": packageinfo.authors,
        "app_version": update_state,
    });

    Ok(rep)
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
    mh.show_notification(&title, &message,Some("https://i.imgur.com/UggEVVI.jpeg"), sound.as_deref());
}