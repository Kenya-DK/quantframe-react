use std::sync::{Arc, Mutex};

use crate::{
    database::{client::DBClient, modules::transaction::TransactionStruct},
    error::AppError,
};

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
    // Update UI
    // db.send_to_window(
    //     "transactions",
    //     "CREATE_OR_UPDATE",
    //     serde_json::to_value(transaction.clone()).unwrap(),
    // );
    Ok(transaction)
}
