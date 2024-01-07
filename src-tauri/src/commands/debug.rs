use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{
    debug::DebugClient,
    error::{self, AppError},
};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_debug.log".to_string()));

#[tauri::command]
pub async fn import_warframe_algo_trader_data(
    db_path: String,
    import_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    let debug = debug.lock()?.clone();
    match debug
        .import_warframe_algo_trader_data(db_path, import_type)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
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
