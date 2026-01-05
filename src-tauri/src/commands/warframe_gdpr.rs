use std::sync::{Arc, Mutex};

use entity::dto::FinancialReport;
use serde_json::{json, Value};
use utils::Error;

use crate::log_parser::{
    LogParserState, LoginPaginationQueryDto, PurchasePaginationQueryDto, TradePaginationQueryDto,
    TransactionPaginationQueryDto,
};

#[tauri::command]
pub async fn wfgdpr_get_state(
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    Ok(json!({
        "was_initialized": log_parser.warframe_gdpr().was_initialized(),
        "trade_years": log_parser.warframe_gdpr().get_trade_years(),
    }))
}
#[tauri::command]
pub async fn wfgdpr_get_trades_pagination(
    query: TradePaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    let trades = log_parser.warframe_gdpr().trades(query.clone());
    Ok(json!(trades))
}
#[tauri::command]
pub async fn wfgdpr_get_trades_financial_report(
    query: TradePaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<FinancialReport, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    Ok(log_parser.warframe_gdpr().trade_financial_report(query))
}

#[tauri::command]
pub async fn wfgdpr_get_purchases_pagination(
    query: PurchasePaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    let purchases = log_parser.warframe_gdpr().purchases(query);
    Ok(json!(purchases))
}

#[tauri::command]
pub async fn wfgdpr_get_logins_pagination(
    query: LoginPaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    let logins = log_parser.warframe_gdpr().logins(query);
    Ok(json!(logins))
}

#[tauri::command]
pub async fn wfgdpr_get_transactions_pagination(
    query: TransactionPaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<Value, Error> {
    // Read the file content
    let log_parser = log_parser.lock()?;
    let transactions = log_parser.warframe_gdpr().transactions(query);
    Ok(json!(transactions))
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
