use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::{qf_client::client::QFClient, utils::modules::error::AppError};

#[tauri::command]
pub async fn item_get_syndicates_prices(
    page: i64,
    limit: i64,
    filter: Option<Value>,
    sort: Option<Value>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<Value, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();
    match qf
        .item()
        .get_syndicates_prices(page, limit, filter, sort)
        .await
    {
        Ok(list) => Ok(list),
        Err(e) => Err(e),
    }
}
