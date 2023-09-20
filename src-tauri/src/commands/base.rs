use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    cache::CacheState,
    database::DatabaseClient,
    debug::DebugClient,
    error::{self, AppError},
    helper, logger,
    price_scraper::{self, PriceScraper},
    settings::SettingsState,
    wfm_client::WFMClientState,
    wfm_client2::client::ClientState,
};

#[tauri::command]
pub async fn init(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
    wfm2: tauri::State<'_, Arc<Mutex<ClientState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheState>>>,
    price_scraper: tauri::State<'_, Arc<Mutex<PriceScraper>>>,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Value, AppError> {
    let db = db.lock()?.clone();
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let wfm2 = wfm2.lock()?.clone();
    let cache = cache.lock()?.clone();
    let price_scraper = price_scraper.lock()?.clone();

    let items = wfm2.orders().get_my_auctions().await?;
    println!("items: {:?}", items.len());

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Items..."})),
    );
    let items = cache.update_items().await?;
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
        Some(json!({"status": "Validating Credentials..."})),
    );
    if !wfm.validate().await? {
        return Ok(json!({"valid": false, "settings": &settings.clone()}));
    }

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Inventory..."})),
    );
    let inventorys = db.get_inventorys().await?;

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Transactions..."})),
    );
    let transactions = db.get_transactions("SELECT * FROM transactions").await?;

    helper::send_message_to_window(
        "set_initializstatus",
        Some(json!({"status": "Loading Your Orders..."})),
    );
    let orders = wfm.get_user_ordres_as_list().await?;

    Ok(json!({
        "valid": true,
        "settings": &settings.clone(),
        "user": &auth.clone(),
        "inventorys": inventorys,
        "transactions": transactions,
        "orders":orders,
        "price_scraper_last_run":price_scraper.get_status(),
        "items": items,


    }))
}

#[tauri::command]
pub async fn update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;
    // Set Live Scraper Settings
    my_lock.live_scraper.volume_threshold = settings.live_scraper.volume_threshold;
    my_lock.live_scraper.range_threshold = settings.live_scraper.range_threshold;
    my_lock.live_scraper.avg_price_cap = settings.live_scraper.avg_price_cap;
    my_lock.live_scraper.max_total_price_cap = settings.live_scraper.max_total_price_cap;
    my_lock.live_scraper.price_shift_threshold = settings.live_scraper.price_shift_threshold;
    my_lock.live_scraper.blacklist = settings.live_scraper.blacklist;
    my_lock.live_scraper.whitelist = settings.live_scraper.whitelist;
    my_lock.live_scraper.strict_whitelist = settings.live_scraper.strict_whitelist;
    my_lock.live_scraper.webhook = settings.live_scraper.webhook;
    // Set Whisper Scraper Settings
    my_lock.whisper_scraper.ping_on_notif = settings.whisper_scraper.ping_on_notif;
    my_lock.whisper_scraper.webhook = settings.whisper_scraper.webhook;
    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}
