use std::sync::{Arc, Mutex};

use crate::{log_parser::client::LogParser, utils::modules::error::AppError};

#[tauri::command]
pub async fn get_cache_lines(
    log_parser: tauri::State<'_, Arc<Mutex<LogParser>>>,
) -> Result<Vec<String>, AppError> {
    let log_parser = log_parser.lock()?.clone();
    let lines = log_parser.get_cache_lines();
    Ok(lines)
}

#[tauri::command]
pub async fn get_last_read_date(
    log_parser: tauri::State<'_, Arc<Mutex<LogParser>>>,
) -> Result<String, AppError> {
    let log_parser = log_parser.lock()?.clone();
    let date = log_parser.get_last_read_date();
    Ok(date)
}

#[tauri::command]
pub async fn clear_cache_lines(
    log_parser: tauri::State<'_, Arc<Mutex<LogParser>>>,
) -> Result<(), AppError> {
    let log_parser = log_parser.lock()?.clone();
    log_parser.clear_cache();
    Ok(())
}

#[tauri::command]
pub async fn dump_cache_lines(
    log_parser: tauri::State<'_, Arc<Mutex<LogParser>>>,
) -> Result<String, AppError> {
    let log_parser = log_parser.lock()?.clone();
    Ok(log_parser.dump_cache())
}
