use crate::{
    database::{client::DBClient, modules::transaction::TransactionStruct}, utils::modules::error::{self, AppError}
};
use eyre::eyre;
use std::sync::{Arc, Mutex};


#[tauri::command]
pub async fn tra_get_all(
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Vec<TransactionStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.transaction().get_items().await {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn tra_get_by_id(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Option<TransactionStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.transaction().get_by_id(id).await {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(e)
    }
}

#[tauri::command]
pub async fn tra_update_by_id(
    id: i64,
    price: Option<i64>,
    transaction_type: Option<String>,
    quantity: Option<i64>,
    rank: Option<i64>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<TransactionStruct, AppError> {
    let db = db.lock()?.clone();
    match db
        .transaction()
        .update_by_id(id, price, transaction_type, quantity, rank)
        .await
    {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(e),        
    }
}

#[tauri::command]
pub async fn tra_delete_by_id(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<bool, AppError> {
    let db = db.lock()?.clone();
    match db.transaction().delete(id).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

