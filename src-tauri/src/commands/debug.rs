use std::sync::{Arc, Mutex};

use crate::{auth::AuthState, debug::DebugClient, error::{AppError, self}, wfm_client::client::WFMClient};

#[tauri::command]
pub async fn import_warframe_algo_trader_data(
    db_path: String,
    import_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    let debug = debug.lock()?.clone();
    match     debug
    .import_warframe_algo_trader_data(db_path, import_type)
    .await {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("debug".to_string(), &e);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn reset_data(
    reset_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    let debug = debug.lock()?.clone();
    debug.reset_data(reset_type).await?;
    Ok(())
}
