use std::sync::{Arc, Mutex};

use crate::{error::AppError, database::{client::DBClient, modules::transaction::TransactionStruct}, structs::RivenAttribute};

#[tauri::command]
pub async fn create_transaction_entry(
    id: String,
    ttype: String,
    item_type: String,
    quantity: i32,
    rank: i32,
    price: i32,
    sub_type: Option<&str>,
    attributes: Option<Vec<RivenAttribute>>,
    mastery_rank: Option<i32>,
    re_rolls: Option<i32>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<TransactionStruct, AppError> {
    let db = db.lock()?.clone();
    let transaction = db.transaction().create(&id, &item_type, &ttype, quantity, price, rank, sub_type, attributes, mastery_rank, re_rolls)
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
