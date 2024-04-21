use entity::transaction;
use service::TransactionQuery;
use std::{
    f32::consts::E,
    sync::{Arc, Mutex},
};

use crate::{
    app::client::AppState,
    utils::modules::{
        error::{self, AppError},
        logger::error,
    },
};

#[tauri::command]
pub async fn transaction_get_all(
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<transaction::Model>, AppError> {
    let app = app.lock()?.clone();
    match TransactionQuery::get_all(&app.conn).await {
        Ok(transactions) => {
            return Ok(transactions);
        }
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };
}
