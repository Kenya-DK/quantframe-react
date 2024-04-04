use std::sync::{Arc, Mutex};

use crate::{
    database::{
        client::DBClient,
        modules::stock_riven::{MatchRivenStruct, StockRivenStruct},
    },
    utils::modules::error::AppError,
    wfm_client::types::riven_attribute::RivenAttribute,
};

#[tauri::command]
pub async fn stock_riven_get_all(
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Vec<StockRivenStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.stock_riven().get_rivens().await {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_riven_get_by_id(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<Option<StockRivenStruct>, AppError> {
    let db = db.lock()?.clone();
    match db.stock_riven().get_by_id(id).await {
        Ok(items) => Ok(items),
        Err(e) => {
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn stock_riven_create(
    id: String,
    price: f64,
    rank: i32,
    attributes: Vec<RivenAttribute>,
    match_riven: Option<MatchRivenStruct>,
    mastery_rank: i32,
    re_rolls: i32,
    polarity: &str,
    mod_name: &str,
    minium_price: Option<i32>,
    comment: Option<String>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<StockRivenStruct, AppError> {
    let db = db.lock()?.clone();
    match db
        .stock_riven()
        .create(
            None,
            &id,
            mod_name,
            price.clone(),
            rank,
            attributes,
            match_riven,
            mastery_rank,
            re_rolls,
            polarity,
            minium_price,
            comment,
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
pub async fn stock_riven_update(
    id: i64,
    attributes: Option<Vec<RivenAttribute>>,
    match_riven: Option<MatchRivenStruct>,
    minium_price: Option<i32>,
    private: Option<bool>,
    comment: Option<String>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<StockRivenStruct, AppError> {
    let db = db.lock()?.clone();
    match db
        .stock_riven()
        .update_by_id(
          id,
          None,
          None,
          None,
          None,
          attributes,
          match_riven,
          minium_price,
          None,
          private,
          comment,
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
pub async fn stock_riven_delete(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<bool, AppError> {
    let db = db.lock()?.clone();
    match db.stock_riven().delete(id).await {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(e);
        }
    }
}
