use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::{
    error::{self, AppError},
    helper,
    wfm_client::client::WFMClient, database::client::DBClient,
};
use std::sync::{Arc, Mutex};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_orders.log".to_string()));

#[tauri::command]
pub async fn get_orders(_wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    Ok(())
}
#[tauri::command]
pub async fn delete_order(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.orders().delete(id.as_str(), "Any", "", "").await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
    Ok(())
}
#[tauri::command]
pub async fn create_order(_wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    Ok(())
}
#[tauri::command]
pub async fn update_order(_wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    Ok(())
}

#[tauri::command]
pub async fn refresh_orders(wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let current_orders = match wfm.orders().get_my_orders().await {
        Ok(mut auctions) => {
            let mut orders = auctions.buy_orders;
            orders.append(&mut auctions.sell_orders);
            orders
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    helper::emit_update("orders", "SET", Some(serde_json::to_value(current_orders).unwrap()));
    Ok(())
}
#[tauri::command]
pub async fn delete_all_orders(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<serde_json::Value, AppError> {
    let wfm = wfm.lock()?.clone();
    let db = db.lock()?.clone();
    match db.stock_item().reset_listed_price().await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    let current_orders = match wfm.orders().get_my_orders().await {
        Ok(mut auctions) => {
            let mut orders = auctions.buy_orders;
            orders.append(&mut auctions.sell_orders);
            orders
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    let count = current_orders.len();
    for order in current_orders {
        match wfm.orders().delete(&order.id, "None", "None", "Any").await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e);
            }
        };
    }
    Ok(json!({
        "count": count
    }))
}
