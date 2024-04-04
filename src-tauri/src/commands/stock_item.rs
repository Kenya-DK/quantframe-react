use std::sync::{Arc, Mutex};

use crate::{
    database::{client::DBClient, modules::stock_item::StockItemStruct},
    utils::modules::error::AppError,
};

#[tauri::command]
pub async fn stock_item_get_all(
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Vec<StockItemStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.stock_item().get_items().await {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_item_get_by_id(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Option<StockItemStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.stock_item().get_by_id(id).await {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_item_create(
    url_name: String,
    quantity: i32,
    price: f64,
    rank: i32,
    sub_type: Option<&str>,
    minium_price: Option<i32>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<StockItemStruct, AppError> {
    let db = db.lock()?.clone();
    match db
        .stock_item()
        .create(&url_name, quantity, price, minium_price, rank, sub_type)
        .await
    {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_item_update(
    id: i64,
    owned: Option<i32>,
    minium_price: Option<i32>,
    hidden: Option<bool>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<StockItemStruct, AppError> {
    let db = db.lock()?.clone();
    match db
        .stock_item()
        .update_by_id(
            id,
            owned,
            None,
            minium_price,
            None,
            None,
            hidden,
            None,
            None,
        )
        .await
    {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_item_delete(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<bool, AppError> {
    let db = db.lock()?.clone();
    match db.stock_item().delete(id).await {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(e);
        }
    }
}
