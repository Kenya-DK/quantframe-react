use once_cell::sync::Lazy;
use serde_json::json;

use crate::{error::{AppError, self}, helper, wfm_client::client::WFMClient};
use std::sync::{Arc, Mutex};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("commands.log".to_string()));

#[tauri::command]
pub async fn get_orders(wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}
#[tauri::command]
pub async fn delete_order(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.orders().delete(id.as_str(), "Any", "", "").await {
        Ok(_) => {
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
        }
    }
    Ok(())
}
#[tauri::command]
pub async fn create_order(
    id: String,
    order_type: String,
    quantity: i64,
    price: i64,
    rank: i64,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}
#[tauri::command]
pub async fn update_order(
    id: String,
    order_type: String,
    quantity: i64,
    price: i64,
    rank: i64,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}

#[tauri::command]
pub async fn refresh_orders(wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let mut ordres_vec = wfm.orders().get_my_orders().await?;
    let mut ordres = ordres_vec.buy_orders;
    ordres.append(&mut ordres_vec.sell_orders);
    helper::emit_update("orders", "SET", Some(serde_json::to_value(ordres).unwrap()));
    Ok(())
}
#[tauri::command]
pub async fn delete_all_orders(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let wfm = wfm.lock()?.clone();

    let current_orders = wfm.orders().get_my_orders().await?;

    let count = current_orders.buy_orders.len() + current_orders.sell_orders.len();

    for order in current_orders.sell_orders {
        wfm.orders()
            .delete(&order.id, "None", "None", "Any")
            .await?;
    }
    for order in current_orders.buy_orders {
        wfm.orders()
            .delete(&order.id, "None", "None", "Any")
            .await?;
    }
    Ok(json!({
        "count": count
    }))
}