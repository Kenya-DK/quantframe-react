use crate::{
    app::client::AppState,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
};
use entity::transaction::transaction;
use eyre::eyre;
use serde_json::json;
use service::{TransactionMutation, TransactionQuery};
use std::sync::{Arc, Mutex};

#[tauri::command]
pub async fn transaction_reload(
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();

    match TransactionQuery::get_all(&app.conn).await {
        Ok(transactions) => {
            qf.analytics().add_metric("Transaction_Reload", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::Set,
                Some(json!(transactions)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::reload", e);
            error::create_log_file("transaction_reload.log".to_string(), &error);
            return Err(error);
        }
    };
    Ok(())
}
#[tauri::command]
pub async fn transaction_get_all(
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<transaction::Model>, AppError> {
    let app = app.lock()?.clone();
    match TransactionQuery::get_all(&app.conn).await {
        Ok(transactions) => {
            return Ok(transactions);
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("transaction_get_all.log".to_string(), &error);
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn transaction_update(
    id: i64,
    price: Option<i64>,
    quantity: Option<i64>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<transaction::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();

    // Find the transaction by id
    let transaction = match TransactionQuery::find_by_id(&app.conn, id).await {
        Ok(transaction) => transaction,
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_by_id", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };

    if transaction.is_none() {
        return Err(AppError::new(
            "TransactionUpdate",
            eyre!(format!("Transaction with id {} not found", id)),
        ));
    }

    let mut new_item = transaction.unwrap();

    if let Some(price) = price {
        new_item.price = price;
    }

    if let Some(quantity) = quantity {
        new_item.quantity = quantity;
    }

    match TransactionMutation::update_by_id(&app.conn, id, new_item.clone()).await {
        Ok(updated) => {
            qf.analytics().add_metric("Transaction_Update", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("transaction_update.log".to_string(), &error);
            return Err(error);
        }
    };
    match qf.transaction().update_transaction(&new_item).await {
        Ok(_) => (),
        Err(e) => {
            error::create_log_file("transaction_update.log".to_string(), &e);
            return Err(e);
        }
    }
    Ok(new_item)
}

#[tauri::command]
pub async fn transaction_delete(
    id: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf: QFClient = qf.lock()?.clone();
    match TransactionMutation::delete_by_id(&app.conn, id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                qf.analytics().add_metric("Transaction_Delete", "manual");
                notify.gui().send_event_update(
                    UIEvent::UpdateTransaction,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionMutation::delete", e);
            error::create_log_file("transaction_delete.log".to_string(), &error);
            return Err(error);
        }
    };

    match qf.transaction().delete_transaction(id).await {
        Ok(_) => (),
        Err(e) => {
            error::create_log_file("transaction_delete.log".to_string(), &e);
            return Err(e);
        }
    }
    Ok(())
}
