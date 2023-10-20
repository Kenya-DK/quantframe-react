use std::sync::{Arc, Mutex};

use eyre::eyre;
use reqwest::{Client, Method, Url};
use serde_json::{json, Value};
use tokio::process::Command;

use crate::{
    auth::AuthState,
    cache::client::CacheClient,
    database::client::DBClient,
    debug::DebugClient,
    error::{self, AppError},
    helper, logger,
    price_scraper::{self, PriceScraper},
    settings::SettingsState,
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn init(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    price_scraper: tauri::State<'_, Arc<Mutex<PriceScraper>>>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Value, AppError> {
    let db = db.lock()?.clone();
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();
    let price_scraper = price_scraper.lock()?.clone();

    let mut response = json!({
        "settings": &settings.clone(),
        "user": &auth.clone(),
        "price_scraper_last_run":price_scraper.get_status(),
    });

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Database..."})),
    );
    match db.initialize().await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("db.log".to_string(), &e);
            return Err(e);
        }
    }

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Cache..."})),
    );
    cache.refresh().await?;
    response["items"] = json!(cache.items().get_types()?);
    response["riven_items"] = json!(cache.riven().get_types()?);
    response["riven_attributes"] = json!(cache.riven().get_attributes()?);

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Validating Credentials..."})),
    );
    let is_validate = wfm.auth().validate().await?;
    response["valid"] = json!(is_validate);

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Stock..."})),
    );
    response["stock_items"] = json!(db.stock_item().get_items().await?);
    response["stock_rivens"] = json!(db.stock_riven().get_rivens().await?);

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Transactions..."})),
    );
    response["transactions"] = json!(db.transaction().get_items().await?);

    if is_validate {
        helper::send_message_to_window(
            "set_initializstatus",
            Some(json!({"status": "Loading Your Orders..."})),
        );
        let mut ordres_vec = wfm.orders().get_my_orders().await?;
        let mut ordres = ordres_vec.buy_orders;
        ordres.append(&mut ordres_vec.sell_orders);
        response["orders"] = json!(ordres);

        helper::send_message_to_window(
            "set_initializstatus",
            Some(json!({"status": "Loading Your Auctions..."})),
        );
        response["auctions"] = json!(wfm.auction().get_my_auctions().await?);
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
    my_lock.live_scraper.stock_item.volume_threshold = settings.live_scraper.stock_item.volume_threshold;
    my_lock.live_scraper.stock_item.range_threshold = settings.live_scraper.stock_item.range_threshold;
    my_lock.live_scraper.stock_item.avg_price_cap = settings.live_scraper.stock_item.avg_price_cap;
    my_lock.live_scraper.stock_item.max_total_price_cap = settings.live_scraper.stock_item.max_total_price_cap;
    my_lock.live_scraper.stock_item.price_shift_threshold = settings.live_scraper.stock_item.price_shift_threshold;
    my_lock.live_scraper.stock_item.blacklist = settings.live_scraper.stock_item.blacklist;
    my_lock.live_scraper.stock_item.whitelist = settings.live_scraper.stock_item.whitelist;
    my_lock.live_scraper.stock_item.strict_whitelist = settings.live_scraper.stock_item.strict_whitelist;

    // Stock Riven
    my_lock.live_scraper.stock_riven.range_threshold = settings.live_scraper.stock_riven.range_threshold;

    // Set Whisper Scraper Settings
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
