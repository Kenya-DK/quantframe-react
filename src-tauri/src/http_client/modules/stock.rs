use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};

use serde_json::json;
use tauri::{Manager, State};

use crate::{
    app::client::AppState,
    cache::client::CacheClient,
    helper,
    http_client::types::{create_item::ItemPayload, create_riven::RivenPayload},
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    settings::SettingsState,
    utils::modules::error::{self},
    wfm_client::{client::WFMClient, enums::order_type::OrderType},
    APP,
};

#[post("/add_riven")]
pub async fn add_riven(riven: web::Json<RivenPayload>) -> impl Responder {
    let app_handle = APP.get().expect("failed to get app handle");
    let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    let app = app_state.lock().expect("failed to lock app state").clone();

    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state
        .lock()
        .expect("failed to lock notify state")
        .clone();

    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let cache = cache_state
        .lock()
        .expect("failed to lock notify state")
        .clone();

    let wfm_state: State<Arc<Mutex<WFMClient>>> = app_handle.state();
    let wfm = wfm_state.lock().expect("failed to lock wfm state").clone();

    let qf_state: State<Arc<Mutex<QFClient>>> = app_handle.state();
    let qf = qf_state.lock().expect("failed to lock qf state").clone();

    let mut entry = riven.into_inner();

    match helper::progress_stock_riven(
        &mut entry.riven_data,
        // "--weapon_by name --weapon_lang en --attribute_by upgrades --upgrade_by short_string",
        entry.by.as_str(),
        "",
        OrderType::Buy,
        "http_server",
        &app,
        &cache,
        &notify,
        &wfm,
        &qf,
    )
    .await
    {
        Ok((stock, _)) => HttpResponse::Ok().body(serde_json::to_string(&stock).unwrap()),
        Err(e) => {
            error::create_log_file("command_stock_riven_sell.log".to_string(), &e);
            HttpResponse::BadRequest().body(json!(e).to_string())
        }
    }
}

#[post("/add_item")]
pub async fn add_item(_item: web::Json<ItemPayload>) -> impl Responder {
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
