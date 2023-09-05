use std::sync::{Arc, Mutex};

use crate::{
    database::DatabaseClient,
    structs::Invantory,
    error::AppError,
};

#[tauri::command]
pub async fn create_invantory_entry(
    id: String,
    report: bool,
    quantity: i64,
    price: i64,
    rank: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Invantory, AppError> {
    let db = db.lock()?.clone();
    let invantory = db
        .create_inventory_entry(id, report, quantity, price, rank)
        .await
        .unwrap();
    Ok(invantory)
}

#[tauri::command]
pub async fn delete_invantory_entry(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Option<Invantory>, AppError> {
    let db = db.lock()?.clone();
    Ok(db.delete_inventory_entry(id).await?)
}
#[tauri::command]
pub async fn sell_invantory_entry(
    id: i64,
    report: bool,
    price: i64,
    quantity: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Invantory, AppError> {
    let db = db.lock()?.clone();
    let invantory = db.sell_invantory_entry(id, report, price, quantity).await?;
    Ok(invantory)
}
