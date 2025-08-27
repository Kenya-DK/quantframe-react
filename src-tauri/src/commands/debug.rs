use std::sync::Mutex;

use serde_json::{json, Value};
use utils::Error;

use crate::app::client::AppState;

#[tauri::command]
pub fn debug_get_wfm_state(app: tauri::State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let orders = app.wfm_client.order().cache_orders();
    let user_auctions = app.wfm_client.auction().cache_auctions();
    Ok(json!({
      "user_orders": json!(orders),
      "user_auctions": json!(user_auctions),
      "order_limit": app.wfm_client.order().get_order_limit()
    }))
}
