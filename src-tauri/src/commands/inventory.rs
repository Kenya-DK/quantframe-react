use std::sync::{Arc, Mutex};

use crate::{
    database::DatabaseClient,
    error::{self, AppError},
    structs::Invantory,
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
    match db.create_inventory_entry(id, report, quantity, price, rank).await {
        Ok(invantory) => {
            db.send_to_window("inventorys", "CREATE_OR_UPDATE", serde_json::to_value(invantory.clone()).unwrap());                
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}

#[tauri::command]
pub async fn delete_invantory_entry(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DatabaseClient>>>,
) -> Result<Option<Invantory>, AppError> {
    let db = db.lock()?.clone();
    match db.delete_inventory_entry(id).await {
        Ok(invantory) => {
            db.send_to_window("inventorys", "DELETE", serde_json::to_value(invantory.clone()).unwrap());
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
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
    match db.sell_invantory_entry(id, report, price, quantity).await {
        Ok(invantory) => {
            if invantory.owned == 0 {
                db.send_to_window("inventorys", "DELETE", serde_json::to_value(invantory.clone()).unwrap());
            } else {
                db.send_to_window("inventorys", "CREATE_OR_UPDATE", serde_json::to_value(invantory.clone()).unwrap());                       
            }
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}
