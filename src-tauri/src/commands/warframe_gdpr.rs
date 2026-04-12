use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use utils::Error;

use crate::log_parser::LogParserState;

#[tauri::command]
pub async fn wfgdpr_get_state(
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    Ok(json!({
        "was_initialized": log_parser.warframe_gdpr().was_initialized(),
    }))
}

#[tauri::command]
pub async fn wfgdpr_load(
    file_path: String,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<(), Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    log_parser.warframe_gdpr().load(&file_path)?;
    Ok(())
}

#[tauri::command]
pub async fn wfgdpr_get_accounts(
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    let log_parser = log_parser.lock()?;
    Ok(json!(log_parser.warframe_gdpr().accounts()))
}
