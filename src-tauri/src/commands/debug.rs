use std::sync::{Arc, Mutex};

use serde_json::json;
use service::{sea_orm::Database, StockItemMutation, StockRivenMutation, TransactionMutation};

use crate::{
    app::client::AppState,
    debug::DebugClient,
    helper,
    notification::client::NotifyClient,
    utils::{enums::ui_events::UIOperationEvent, modules::error::AppError},
};

#[tauri::command]
pub async fn debug_migrate_data_base(
    target: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, AppError> {
    let debug = debug.lock()?.clone();
    let app = app.lock()?.clone();

    // Check if the old database exists
    let old_db_path = helper::get_app_storage_path().join("quantframe.sqlite");
    if !old_db_path.exists() {
        return Err(AppError::new(
            "DebugDbReset",
            eyre::eyre!("Old database not found"),
        ));
    }

    let db_url = format!("sqlite://{}?mode=rwc", old_db_path.to_str().unwrap());
    let old_con = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    match target.as_str() {
        "all" => {
            debug.migrate_data_all(&old_con, &app.conn).await?;
        }
        "stock_item" => {
            debug.migrate_data_stock_item(&old_con, &app.conn).await?;
        }
        "stock_riven" => {
            debug.migrate_data_stock_riven(&old_con, &app.conn).await?;
        }
        "transaction" => {
            debug.migrate_data_transactions(&old_con, &app.conn).await?;
        }
        _ => {
            return Err(AppError::new("DebugDbReset", eyre::eyre!("Invalid target")));
        }
    }
    Ok(true)
}

#[tauri::command]
pub async fn debug_db_reset(
    target: String,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, AppError> {
    let notify = notify.lock()?.clone();
    let app = app.lock()?.clone();
    match target.as_str() {
        "all" => {
            StockItemMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            StockRivenMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            TransactionMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!([])),
            );
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!([])),
            );
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
                UIOperationEvent::Set,
                Some(json!([])),
            );
        }
        "stock_item" => {
            StockItemMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!([])),
            );
        }
        "stock_riven" => {
            StockRivenMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!([])),
            );
        }
        "transaction" => {
            TransactionMutation::delete_all(&app.conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
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
