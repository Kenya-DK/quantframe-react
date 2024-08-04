use std::sync::{Arc, Mutex};

use serde_json::json;
use service::{StockItemQuery, StockRivenQuery, TransactionQuery};

use crate::{
    app::client::AppState,
    auth::AuthState,
    cache::client::CacheClient,
    log_parser,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    settings::SettingsState,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::client::WFMClient,
};

pub fn save_auth_state(auth: tauri::State<'_, Arc<Mutex<AuthState>>>, auth_state: AuthState) {
    let arced_mutex = Arc::clone(&auth);
    let mut my_lock = arced_mutex.lock().expect("Could not lock auth");
    *my_lock = auth_state.clone();
}

#[tauri::command]
pub async fn app_init(
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    log_parser: tauri::State<'_, Arc<Mutex<log_parser::client::LogParser>>>,
) -> Result<bool, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let settings = settings.lock()?.clone();
    let mut wfm = wfm.lock()?.clone();
    let cache = cache.lock()?.clone();
    let qf = qf.lock()?.clone();
    let log_parser = log_parser.lock()?.clone();
    let mut auth_state = auth.lock()?.clone();

    // Send App Info to UI
    let app_info = app.get_app_info();
    notify.gui().send_event_update(
        UIEvent::UpdateAppInfo,
        UIOperationEvent::Set,
        Some(json!({
            "version": app_info.version,
            "name": app_info.name,
            "description": app_info.description,
            "authors": app_info.authors,
            "is_pre_release": app.is_pre_release,
        })),
    );

    // Send Settings to UI
    notify.gui().send_event_update(
        UIEvent::UpdateSettings,
        UIOperationEvent::Set,
        Some(json!(&settings)),
    );

    // Start Log Parser
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("log_parser")));
    match log_parser.start_loop() {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("log_parser.log".to_string(), &e);
            return Err(e);
        }
    }

    // Load Stock Items
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("stock_items")));
    match StockItemQuery::get_all(&app.conn).await {
        Ok(items) => {
            // Send Stock Items to UI
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!(&items)),
            );
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
            // Send Stock Rivens to UI
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!(&items)),
            );
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
            // Send Transactions to UI
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::Set,
                Some(json!(&transactions)),
            );
        }
        Err(e) => {
            let error = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };

    // Validate WFM Auth
    notify
        .gui()
        .send_event(UIEvent::OnInitialize, Some(json!("validation")));
    let wfm_user = match wfm.auth().validate().await {
        Ok(user) => user,
        Err(e) => {
            error::create_log_file("wfm_validation.log".to_string(), &e);
            return Err(e);
        }
    };
    auth_state.update_from_wfm_user_profile(&wfm_user, auth_state.wfm_access_token.clone());

    let mut qf_user = match qf.auth().validate().await {
        Ok(user) => user,
        Err(e) => {
            error::create_log_file("qf_validate.log".to_string(), &e);
            return Err(e);
        }
    };

    if qf_user.is_none() && !wfm_user.anonymous && wfm_user.verification {
        // Login to QuantFrame
        qf_user = match qf
            .auth()
            .login_or_register(
                &auth_state.get_username(),
                &auth_state.get_password(),
                &auth_state.ingame_name.clone(),
            )
            .await
        {
            Ok(user) => {
                auth_state.update_from_qf_user_profile(&user, user.token.clone());
                Some(user.clone())
            }
            Err(e) => {
                error::create_log_file("auth_login.log".to_string(), &e);
                return Err(e);
            }
        }
    } else {
        auth_state.update_from_qf_user_profile(&qf_user.clone().unwrap(), auth_state.qf_access_token.clone());
    }

    // Send User to UI
    notify.gui().send_event_update(
        UIEvent::UpdateUser,
        UIOperationEvent::Set,
        Some(json!(&auth_state.clone())),
    );

    // Save AuthState to Tauri State
    save_auth_state(auth.clone(), auth_state.clone());

    if !wfm_user.anonymous && wfm_user.verification && qf_user.is_some() && !qf_user.unwrap().banned
    {
        // Initialize QF Analytics
        match qf.analytics().init() {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file("analytics.log".to_string(), &e);
                return Err(e);
            }
        }

        // Load Cache
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("cache")));
        match cache.load().await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file("cache.log".to_string(), &e);
                return Err(e);
            }
        }

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
        // Send Orders to UI
        notify.gui().send_event_update(
            UIEvent::UpdateOrders,
            UIOperationEvent::Set,
            Some(json!(&orders)),
        );

        // Load User Auctions
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("user_auctions")));
        match wfm.auction().get_my_auctions().await {
            Ok(auctions) => {
                // Send Auctions to UI
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::Set,
                    Some(json!(&auctions)),
                );
            }
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };

        // Load User Chats
        notify
            .gui()
            .send_event(UIEvent::OnInitialize, Some(json!("user_chats")));
        match wfm.chat().get_chats().await {
            Ok(chats_vec) => {
                // Send Chats to UI
                notify.gui().send_event_update(
                    UIEvent::UpdateChats,
                    UIOperationEvent::Set,
                    Some(json!(&chats_vec)),
                );
            }
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
    }
    // Save AuthState
    auth_state.save_to_file()?;
    // Update The current AuthState

    // let arced_mutex = Arc::clone(&auth);
    // let mut my_lock = arced_mutex.lock().expect("Could not lock auth");
    // *my_lock = auth_state.clone();
    Ok(false)
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

    // Set Logging Settings
    my_lock.debug = settings.debug;

    // Set Live Scraper Settings
    my_lock.live_scraper = settings.live_scraper;

    // Set Whisper Scraper Settings
    my_lock.notifications = settings.notifications;

    my_lock.save_to_file().expect("Could not save settings");

    notify.gui().send_event_update(
        UIEvent::UpdateSettings,
        UIOperationEvent::Set,
        Some(json!(my_lock.clone())),
    );
    Ok(true)
}
