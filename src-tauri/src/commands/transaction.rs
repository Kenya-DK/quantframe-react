use crate::{
    database::{client::DBClient, modules::transaction::TransactionStruct},
    error::{self, AppError},
};
use eyre::eyre;
use std::sync::{Arc, Mutex};

#[tauri::command]
pub async fn create_transaction_entry(
    id: String,
    ttype: String,
    item_type: String,
    quantity: i32,
    rank: i32,
    price: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<TransactionStruct, AppError> {
    let db = db.lock()?.clone();
    let transaction = db
        .transaction()
        .create(&id, &item_type, &ttype, quantity, price, rank, None)
        .await
        .unwrap();
    db.transaction().emit(
        "CREATE_OR_UPDATE",
        serde_json::to_value(transaction.clone()).unwrap(),
    );
    Ok(transaction)
}
#[tauri::command]
pub async fn update_transaction_entry(
    id: i64,
    price: Option<i64>,
    transaction_type: Option<String>,
    quantity: Option<i64>,
    rank: Option<i64>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<TransactionStruct, AppError> {
    let db = db.lock()?.clone();
    // Find Riven in Stock
    let transaction = db.transaction().get_by_id(id).await?;
    if transaction.is_none() {
        return Err(AppError::new(
            "Command:Transaction",
            eyre!("Transaction not found"),
        ));
    }

    // Update Riven in Stock
    match db
        .transaction()
        .update_by_id(id, price, transaction_type, quantity, rank)
        .await
    {
        Ok(transaction) => {
            return Ok(transaction);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}

#[tauri::command]
pub async fn delete_transaction_entry(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<TransactionStruct, AppError> {
    let db = db.lock()?.clone();

    // Find Transaction
    let transaction = db.transaction().get_by_id(id).await?;
    if transaction.is_none() {
        return Err(AppError::new(
            "Command:Transaction",
            eyre!("Transaction not found"),
        ));
    }

    let transaction = transaction.unwrap().clone();
    // Delete Transaction
    db.transaction().delete(id).await?;
    db.transaction()
        .emit("DELETE", serde_json::to_value(transaction.clone()).unwrap());
    Ok(transaction)
}
