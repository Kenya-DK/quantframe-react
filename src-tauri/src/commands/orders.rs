use crate::{error::AppError, helper, wfm_client::client::WFMClient};
use std::sync::{Arc, Mutex};

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
    helper::emit_update("orders", "SET",Some(serde_json::to_value(ordres).unwrap()));
    Ok(())
}
