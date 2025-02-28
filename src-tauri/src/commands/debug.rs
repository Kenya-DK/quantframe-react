use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde_json::{json, Value};
use service::{sea_orm::Database, StockItemMutation, StockRivenMutation, TransactionMutation};

use crate::{
    app::client::AppState,
    debug::DebugClient,
    helper,
    log_parser::types::trade_detection::{TradeDetection, DETECTIONS},
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{enums::ui_events::UIOperationEvent, modules::error::AppError},
    DATABASE,
};

#[tauri::command]
pub async fn debug_import_algo_trader(
    db_path: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<bool, AppError> {
    let conn = DATABASE.get().unwrap();
    let debug = debug.lock()?.clone();
    let app = app.lock()?.clone();
    let qf = qf.lock()?.clone();

    // Check if the old database exists
    let old_db_path = PathBuf::from(db_path);
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

    match debug.import_algo_trader(&old_con, conn).await {
        Ok(_) => {
            qf.analytics()
                .add_metric("Debug_ImportAlgoTrader", "manual");
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(true)
}
#[tauri::command]
pub async fn debug_migrate_data_base(
    target: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<bool, AppError> {
    let conn = DATABASE.get().unwrap();
    let debug = debug.lock()?.clone();
    let app = app.lock()?.clone();
    let qf = qf.lock()?.clone();

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
            debug.migrate_data_all(&old_con, conn).await?;
            helper::add_metric("Debug_MigrateDataBase", "all");
        }
        "stock_item" => {
            debug.migrate_data_stock_item(&old_con, conn).await?;
            qf.analytics()
                .add_metric("Debug_MigrateDataBase", "stock_item");
        }
        "stock_riven" => {
            debug.migrate_data_stock_riven(&old_con, conn).await?;
            qf.analytics()
                .add_metric("Debug_MigrateDataBase", "stock_riven");
        }
        "transaction" => {
            debug.migrate_data_transactions(&old_con, conn).await?;
            qf.analytics()
                .add_metric("Debug_MigrateDataBase", "transaction");
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
            StockItemMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "stock_item");
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!([])),
            );
        }
        "stock_riven" => {
            StockRivenMutation::delete_all(conn)
                .await
                .map_err(|e| AppError::new("DebugDbReset", eyre::eyre!(e)))?;
            helper::add_metric("Debug_DbReset", "stock_riven");
            notify.gui().send_event_update(
                crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!([])),
            );
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

#[tauri::command]
pub async fn debug_method(name: String, payload: Value) -> Result<Value, AppError> {
    match name.as_str() {
        "is_irrelevant_trade_line" => {
            let detections = DETECTIONS.get().unwrap();
            let detection = detections.get("en").unwrap();
            let line = payload["line"].as_str().unwrap();
            let next_line = payload["next_line"].as_str().unwrap();
            let (is_irrelevant, msg, status) = detection.is_irrelevant_trade_line(line, next_line);
            return Ok(json!({
                "is_irrelevant": is_irrelevant,
                "msg": msg,
                "status": format!("{:?}", status),
            }));
        }
        "is_beginning_of_trade" => {
            let detections = DETECTIONS.get().unwrap();
            let detection = detections.get("en").unwrap();
            let line = payload["line"].as_str().unwrap();
            let next_line = payload["next_line"].as_str().unwrap();
            let is_previous = payload["is_previous"].as_str().unwrap();
            let ignore_combined = payload["ignore_combined"].as_str().unwrap();
            return Ok(json!({
                "status": format!("{:?}", detection.is_beginning_of_trade(line,next_line, is_previous== "true", ignore_combined== "true")),
            }));
        }
        "stock_riven" => {}
        "transaction" => {
            // do something
        }
        _ => {
            return Err(AppError::new("DebugDbReset", eyre::eyre!("Invalid target")));
        }
    }

    Ok(json!({}))
}
