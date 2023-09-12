use crate::{wfm_client::WFMClientState, error::AppError};
use std::sync::{Arc, Mutex};


#[tauri::command]
pub async fn get_orders(wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}
#[tauri::command]
pub async fn delete_order(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<(),AppError> {
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
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
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
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}
