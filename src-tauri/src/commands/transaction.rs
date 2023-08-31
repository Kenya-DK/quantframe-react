use std::sync::{Arc, Mutex};

use crate::{
    database::DatabaseClient,
    structs::{GlobleError, Transaction},
};

#[tauri::command]
pub async fn create_transaction_entry(
    id: String,
    ttype: String,
    quantity: i64,
    rank: i64,
    price: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Transaction, GlobleError> {
    let db = db.lock()?.clone();
    let transaction = db
        .create_transaction_entry(id, ttype, quantity, rank, price)
        .await
        .unwrap();
    Ok(transaction)
}
