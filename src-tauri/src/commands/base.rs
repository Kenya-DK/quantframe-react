use entity::transaction;
use serde_json::{json, Value};
use service::{StockItemQuery, StockRivenQuery, TransactionMutation, TransactionQuery};
use std::sync::{Arc, Mutex};
use tokio::process::Command;

use crate::{
    app::client::AppState,
    auth::AuthState,
    cache::client::CacheClient,
    helper, logger,
    notification::client::NotifyClient,
    settings::SettingsState,
    utils::{
        enums::log_level::LogLevel,
        modules::error::{self, AppError},
    },
    wfm_client::{client::WFMClient, types::chat_message::ChatMessage},
};

#[tauri::command]
pub async fn init(
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


    TransactionMutation::create(&app.conn, transaction::Model {
        id:0,
        wfm_id: "123".to_string(),
        wfm_url: "https://warframe.market".to_string(),
        item_name: "Test".to_string(),
        item_type: "Test".to_string(),
        item_unique_name: "Test".to_string(),
        sub_type: None,
        tags: "Test".to_string(),
        transaction_type: transaction::TransactionType::Buy,
        quantity: 1,
        user_name: "Test".to_string(),
        price: 1,
        created_at: "2021-09-01T00:00:00Z".parse().unwrap(),
        updated_at: "2021-09-01T00:00:00Z".parse().unwrap(),
        properties: None,

    }).await.unwrap();


    // Load Cache
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

        let auctions_vec = match wfm.auction().get_my_auctions().await {
            Ok(auctions_vec) => auctions_vec,
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
        response["auctions"] = json!(auctions_vec);

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
pub async fn update_settings(
    settings: SettingsState,
    settings_state: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&settings_state);
    let mut my_lock = arced_mutex.lock()?;

    // Set Log in Settings
    my_lock.debug = settings.debug;

    // Set Live Scraper Settings
    my_lock.live_scraper = settings.live_scraper;

    // Set Whisper Scraper Settings
    my_lock.notifications = settings.notifications;

    my_lock.save_to_file().expect("Could not save settings");
    Ok(())
}

#[tauri::command]
pub async fn open_logs_folder() {
    Command::new("explorer")
        .args(["/select,", &logger::get_log_folder().to_str().unwrap()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn show_notification(
    title: String,
    message: String,
    _icon: Option<String>,
    sound: Option<String>,
    notify: tauri::State<'_, Arc<std::sync::Mutex<NotifyClient>>>,
) {
    let notify = notify.lock().unwrap();
    notify
        .system()
        .send_notification(&title, &message, None, sound.as_deref());
}

#[tauri::command]
pub fn on_new_wfm_message(
    message: ChatMessage,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    settings: tauri::State<'_, Arc<std::sync::Mutex<SettingsState>>>,
    mh: tauri::State<'_, Arc<std::sync::Mutex<NotifyClient>>>,
) {
    let mh = mh.lock().unwrap();
    let auth = auth.lock().unwrap().clone();
    let settings = settings
        .lock()
        .unwrap()
        .clone()
        .notifications
        .on_wfm_chat_message;

    if auth.id == message.message_from {
        return;
    }

    let content = settings.content.replace(
        "<WFM_MESSAGE>",
        &message.raw_message.unwrap_or("".to_string()),
    );
    if settings.system_notify {
        mh.system().send_notification(
            &settings.title,
            &content,
            Some("https://i.imgur.com/UggEVVI.jpeg"),
            Some("Default"),
        );
    }

    if settings.discord_notify && settings.webhook.is_some() {
        crate::helper::send_message_to_discord(
            settings.webhook.unwrap_or("".to_string()),
            settings.title,
            content,
            settings.user_ids,
        );
    }
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
pub fn export_logs(notify: tauri::State<'_, Arc<std::sync::Mutex<NotifyClient>>>) {
    let notify = notify.lock().unwrap().clone();
    logger::export_logs();

    notify
        .system()
        .send_notification("Logs Exported", "Logs exported to desktop", None, None);
}
