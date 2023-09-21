use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    cache::CacheState,
    database::DatabaseClient,
    database2::client::DBClient,
    debug::DebugClient,
    error::{self, AppError},
    helper, logger,
    price_scraper::{self, PriceScraper},
    settings::SettingsState,
    wfm_client::client::WFMClient, structs::InvantoryCreateOrUpdate,
};

#[tauri::command]
pub async fn init(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheState>>>,
    price_scraper: tauri::State<'_, Arc<Mutex<PriceScraper>>>,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
    db2: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Value, AppError> {
    let db = db.lock()?.clone();
    let db2 = db2.lock()?.clone();
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();
    let price_scraper = price_scraper.lock()?.clone();

    let inv = db2
        .inventory()
        .get_item_by_url_name("archon_vitality")
        .await?;
    if inv.is_none() {
        println!("Item not found");
    } else {
        let mut inv = inv.unwrap();
        inv.owned = 1555;
        db2.inventory().update(InvantoryCreateOrUpdate{id:20}).await?;
    }
    // println!("items: {:?}", items.len());

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
    if !wfm.auth().validate().await? {
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
    let current_orders = wfm.orders().get_my_orders().await?;

    Ok(json!({
        "valid": true,
        "settings": &settings.clone(),
        "user": &auth.clone(),
        "inventorys": inventorys,
        "transactions": transactions,
        "orders":vec![current_orders.sell_orders, current_orders.buy_orders],
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
