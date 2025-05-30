use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde_json::{json, Value};
use service::{sea_orm::Database, StockItemMutation, StockRivenMutation, TransactionMutation};

use crate::{
    debug::DebugClient,
    helper,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::UIOperationEvent,
        modules::{error::AppError, trading_helper::combine_and_detect_match},
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
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
                UIOperationEvent::Set,
                Some(json!([])),
            );
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
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
                UIOperationEvent::Set,
                Some(json!([])),
            );
        }
        _ => {
            return Err(AppError::new("DebugDbReset", eyre::eyre!("Invalid target")));
        }
    }
    // let debug_client = debug_client.lock().unwrap();
    Ok(true)
}
