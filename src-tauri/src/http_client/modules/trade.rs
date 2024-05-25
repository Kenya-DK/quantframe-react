use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};
use tauri::{Manager, State};
use regex::Regex;

use  entity::stock::riven::create::CreateStockRiven;

use crate::{
    cache::{client::CacheClient, types::cache_riven::RivenStat}, http_client::{
    types::trade::PlayerTrade}, notification::client::NotifyClient, APP
};

#[post("/progress")]
pub async fn progress(mut riven: web::Json<PlayerTrade>) -> impl Responder {
    let re = Regex::new(r"<.*?>").unwrap();
    let app_handle = APP.get().expect("failed to get app handle");
    // let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    // let app = app_state.lock().expect("failed to lock app state");
    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state.lock().expect("failed to lock notify state");
    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let cache = cache_state.lock().expect("failed to lock notify state");
    
    HttpResponse::Ok().body(format!("Hello {}!", riven.user_name))
}
