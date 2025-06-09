use crate::{
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::UIEvent,
        modules::error::{self, AppError},
    },
    DATABASE,
};
use entity::transaction::transaction::{self};
use eyre::eyre;
use service::{TransactionMutation, TransactionQuery};
use std::sync::{Arc, Mutex};

#[tauri::command]
pub async fn transaction_get_all(
    query: entity::transaction::dto::TransactionPaginationQueryDto,
) -> Result<entity::dto::pagination::PaginatedDto<transaction::Model>, AppError> {
    let conn = DATABASE.get().unwrap();

    match TransactionQuery::get_all(conn, query).await {
        Ok(transactions) => {
            return Ok(transactions);
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("transaction_get_all.log", &error);
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn transaction_update(
    id: i64,
    price: Option<i64>,
    quantity: Option<i64>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<transaction::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();

    // Find the transaction by id
    let transaction = match TransactionQuery::find_by_id(conn, id).await {
        Ok(transaction) => transaction,
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_by_id", e);
            error::create_log_file("command.log", &error);
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

    match TransactionMutation::update_by_id(conn, id, new_item.clone()).await {
        Ok(_) => qf.analytics().add_metric("Transaction_Update", "manual"),
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("transaction_update.log", &error);
            return Err(error);
        }
    };
    notify.gui().send_event(UIEvent::RefreshTransactions, None);
    Ok(new_item)
}

#[tauri::command]
pub async fn transaction_delete(
    id: i64,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let qf: QFClient = qf.lock()?.clone();
    match TransactionMutation::delete_by_id(conn, id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                qf.analytics().add_metric("Transaction_Delete", "manual");
            }
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionMutation::delete", e);
            error::create_log_file("transaction_delete.log", &error);
            return Err(error);
        }
    };
    notify.gui().send_event(UIEvent::RefreshTransactions, None);
    Ok(())
}
