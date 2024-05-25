use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};
use regex::Regex;
use serde_json::json;
use service::{StockRivenMutation, TransactionMutation};
use tauri::{Manager, State};

use entity::stock::{
    self,
    riven::{attribute, create::CreateStockRiven, stock_riven},
};

use crate::{
    app::client::AppState,
    cache::{client::CacheClient, modules::riven, types::cache_riven::RivenStat},
    wfm_client::client::WFMClient,
    http_client::types::conversation::Conversation,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    APP,
};

#[post("/new_conversation")]
pub async fn new_conversation(input: web::Json<Conversation>) -> impl Responder {
    let app_handle = APP.get().expect("failed to get app handle");
    let wfm_state: State<Arc<Mutex<WFMClient>>> = app_handle.state();
    let wfm = wfm_state.lock().expect("failed to lock notify state");

    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state.lock().expect("failed to lock notify state");



    // Look up the user on the Warframe Market API
    let user = match wfm.user().user_profile(&input.user_name).await {
        Ok(user) => {
            Some(user)
        }
        Err(e) => {
            None
        }
    };

    HttpResponse::Ok().body(serde_json::to_string(&input).unwrap())
}
