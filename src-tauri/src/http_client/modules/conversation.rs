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
    HttpResponse::Ok().body(serde_json::to_string(&input).unwrap())
}
