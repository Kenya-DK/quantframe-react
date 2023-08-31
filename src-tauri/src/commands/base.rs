use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{
    auth::AuthState, cache::CacheState, database::DatabaseClient, settings::SettingsState,
    structs::GlobleError, wfm_client::WFMClientState,
};

#[tauri::command]
pub async fn setup(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheState>>>,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Value, GlobleError> {
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();
    cache.update_cache().await?;
    let db = db.lock()?.clone();
    db.initialize().await?;

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
         "transactions": &db.get_transactions("SELECT * FROM transactions").await?,

    }))
}

#[tauri::command]
pub async fn update_settings(
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
    my_lock.ping_on_notif = settings.ping_on_notif;
    my_lock.webhook = settings.webhook;
    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}
