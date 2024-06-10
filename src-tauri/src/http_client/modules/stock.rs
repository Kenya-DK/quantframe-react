use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};

use serde_json::json;
use service::{StockRivenMutation, TransactionMutation};
use tauri::{Manager, State};

use entity::stock::{
    item::{create::CreateStockItem},
    riven::{create::CreateStockRiven},
};

use crate::{
    app::client::AppState,
    cache::{client::CacheClient},
    notification::client::NotifyClient,
    settings::SettingsState,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    APP,
};

#[post("/add_riven")]
pub async fn add_riven(riven: web::Json<CreateStockRiven>) -> impl Responder {
    let component = "HTTPAddRiven";
    let app_handle = APP.get().expect("failed to get app handle");
    let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    let settings_state: State<Arc<Mutex<SettingsState>>> = app_handle.state();
    let _settings = settings_state
        .lock()
        .expect("failed to lock settings state");
    let app = app_state.lock().expect("failed to lock app state");
    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state.lock().expect("failed to lock notify state");
    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let cache = cache_state.lock().expect("failed to lock notify state");

    let mut riven = riven.into_inner();

    // Validate the riven
    match cache.riven().validate_create_riven(
        &mut riven,
        "--weapon_by name --weapon_lang en --attribute_by upgrades",
    ) {
        Ok(_) => (),
        Err(e) => {
            error::create_log_file("http_client.log".to_string(), &e);
            notify.gui().send_event(
                UIEvent::OnNotificationError,
                Some(json!({
                    "i18n_key_title": "add_riven.error.title",
                    "i18n_key_message": "add_riven.error.message",
                    "values": json!(e)
                })),
            );
            return HttpResponse::BadRequest().body(serde_json::to_string(&e).unwrap());
        }
    }

    let stock = riven.to_stock();
    match StockRivenMutation::create(&app.conn, stock.clone()).await {
        Ok(stock) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(stock)),
            );
            notify.gui().send_event(
                UIEvent::OnNotificationSuccess,
                Some(json!({
                    "i18n_key_title": "add_riven.success.title",
                    "i18n_key_message": "add_riven.success.message",
                    "values": {
                        "name": format!("{} {}", riven.weapon_name, riven.mod_name),
                    }
                })),
            );
        }
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(serde_json::to_string(&AppError::new(component, eyre::eyre!(e))).unwrap());
        }
    }

    if stock.bought == 0 {
        return HttpResponse::Ok().body(serde_json::to_string(&stock).unwrap());
    }

    let transaction = stock.to_transaction(
        "",
        stock.bought,
        entity::transaction::transaction::TransactionType::Purchase,
    );

    match TransactionMutation::create(&app.conn, transaction).await {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
        }
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(serde_json::to_string(&AppError::new(component, eyre::eyre!(e))).unwrap());
        }
    }
    HttpResponse::Ok().body(serde_json::to_string(&riven).unwrap())
}

#[post("/add_item")]
pub async fn add_item(_riven: web::Json<CreateStockItem>) -> impl Responder {
    let _component = "HTTPAddItem";
    let app_handle = APP.get().expect("failed to get app handle");
    let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    let settings_state: State<Arc<Mutex<SettingsState>>> = app_handle.state();
    let _settings = settings_state
        .lock()
        .expect("failed to lock settings state");
    let _app = app_state.lock().expect("failed to lock app state");
    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let _notify = notify_state.lock().expect("failed to lock notify state");
    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let _cache = cache_state.lock().expect("failed to lock notify state");

    HttpResponse::Ok().body("Not implemented")
}
