use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use service::{StockItemQuery, StockRivenQuery, TransactionQuery};

use crate::{
    app::client::AppState,
    auth::AuthState,
    cache::client::CacheClient,
    debug::DebugClient,
    notification::client::NotifyClient,
    settings::SettingsState,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn app_init(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Value, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let settings = settings.lock()?.clone();
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();

    let mut response = json!({
        "settings": &settings.clone(),
        "user": &auth.clone(),
    });

    // Load Cache
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("cache")));
    match cache.load().await {
        Ok(_) => {
            response["cache"] = json!({
                "riven_items": cache.riven().get_wfm_riven_types()?,
                "riven_attributes": cache.riven().get_wfm_riven_attributes()?,
                "tradable_items": cache.tradable_items().get_items()?,
            });
        }
        Err(e) => {
            error::create_log_file("command.log".to_string(), &e);
            return Err(e);
        }
    }

    // Validate Auth
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("validation")));
    let is_validate = match wfm.auth().validate().await {
        Ok(is_validate) => {
            response["valid"] = json!(is_validate);
            is_validate
        }
        Err(e) => {
            error::create_log_file("command.log".to_string(), &e);
            return Err(e);
        }
    };

    // Load Stock Items
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("stock_items")));
    match StockItemQuery::get_all(&app.conn).await {
        Ok(items) => {
            response["stock_items"] = json!(items);
        }
        Err(e) => {
            let error = AppError::new_db("StockItemQuery::get_all", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };
    // Load Stock Rivens
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("stock_rivens")));
    match StockRivenQuery::get_all(&app.conn).await {
        Ok(items) => {
            response["stock_rivens"] = json!(items);
        }
        Err(e) => {
            let error = AppError::new_db("StockRivenQuery::get_all", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };

    // Load Transactions
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("transactions")));
    match TransactionQuery::get_all(&app.conn).await {
        Ok(transactions) => {
            response["transactions"] = json!(transactions);
        }
        Err(e) => {
            let error = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };

    if is_validate {
        // Load User Orders
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("user_orders")));
        let mut orders_vec = match wfm.orders().get_my_orders().await {
            Ok(orders_vec) => orders_vec,
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
        let mut orders = orders_vec.buy_orders;
        orders.append(&mut orders_vec.sell_orders);
        response["orders"] = json!(orders);

        // Load User Auctions
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("user_auctions")));
        let auctions_vec = match wfm.auction().get_my_auctions().await {
            Ok(auctions_vec) => auctions_vec,
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
        response["auctions"] = json!(auctions_vec);

        // Load User Chats
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("user_chats")));
        let chats_vec = match wfm.chat().get_chats().await {
            Ok(chats_vec) => chats_vec,
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
        response["chats"] = json!(chats_vec);
    }

    // Check for updates
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("check_updates")));
    let app_info = app.get_app_info();
    response["app_info"] = json!({
        "version": app_info.version,
        "name": app_info.name,
        "description": app_info.description,
        "authors": app_info.authors
    });

    Ok(response)
}

#[tauri::command]
pub async fn app_update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<bool, AppError> {
    let notify = notify.lock()?.clone();
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;

    // Set Loggin Settings
    my_lock.debug = settings.debug;

    // Set Live Scraper Settings
    my_lock.live_scraper = settings.live_scraper;

    // Set Whisper Scraper Settings
    my_lock.notifications = settings.notifications;

    my_lock.save_to_file().expect("Could not save settings");

    notify
        .gui()
        .send_event_update(UIEvent::UpdateSettings, UIOperationEvent::Set, Some(json!(my_lock.clone())));
    Ok(true)
}

