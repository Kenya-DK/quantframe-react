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
pub async fn debug_import_algo_trader(
    db_path: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<bool, AppError> {
    let conn = DATABASE.get().unwrap();
    let debug = debug.lock()?.clone();
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
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<bool, AppError> {
    let conn = DATABASE.get().unwrap();
    let debug = debug.lock()?.clone();
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
        "combine_and_detect_match" => {
            let line = payload["line"].as_str().unwrap();
            let next_line = payload["next_line"].as_str().unwrap();
            let mach = payload["match"].as_str().unwrap();
            let use_previous_line = payload["use_previous_line"].as_str().unwrap();
            let is_exact_match = payload["is_exact_match"].as_str().unwrap();
            let (combine, status) = combine_and_detect_match(
                line,
                next_line,
                mach,
                use_previous_line == "true",
                is_exact_match == "true",
            );
            return Ok(json!({
                "combine": combine,
                "status": format!("{:?}", status),
            }));
        }
        _ => {
            return Err(AppError::new("DebugDbReset", eyre::eyre!("Invalid target")));
        }
    }
}
