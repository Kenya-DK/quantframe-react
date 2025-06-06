use std::sync::{Arc, Mutex};

use serde_json::json;
use service::{StockItemMutation, StockRivenMutation, TransactionMutation};

use crate::{
    helper,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::AppError,
    },
    DATABASE,
};

#[tauri::command]
pub async fn debug_db_reset(
    target: String,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<bool, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    match target.as_str() {
        "all" => {
            StockItemMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            StockRivenMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            TransactionMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "all");
            notify.gui().send_event(UIEvent::RefreshTransactions, None);
        }
        "stock_item" => {
            StockItemMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "stock_item");
        }
        "stock_riven" => {
            StockRivenMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "stock_riven");
        }
        "transaction" => {
            TransactionMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "transaction");
            notify.gui().send_event(UIEvent::RefreshTransactions, None);
        }
        _ => {
            return Err(AppError::new("DebugDbReset", eyre::eyre!("Invalid target")));
        }
    }
    // let debug_client = debug_client.lock().unwrap();
    Ok(true)
}
