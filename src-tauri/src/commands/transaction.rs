use std::sync::Mutex;

use entity::{dto::*, transaction::dto::TransactionPaginationQueryDto, transaction::*};
use service::{TransactionMutation, TransactionQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{app::client::AppState, types::PermissionsFlags, utils::ErrorFromExt, APP, DATABASE};

#[tauri::command]
pub async fn get_transaction_pagination(
    query: TransactionPaginationQueryDto,
) -> Result<PaginatedResult<transaction::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match TransactionQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_db(
                "Command::GetTransactionPagination",
                "Failed to get transactions: {}",
                e,
                get_location!(),
            );
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn get_transaction_financial_report(
    query: TransactionPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_transaction_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn transaction_delete(id: i64) -> Result<transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = TransactionQuery::find_by_id(conn, id).await.map_err(|e| {
        Error::from_db(
            "Command::TransactionDelete",
            "Failed to get transaction by ID: {}",
            e,
            get_location!(),
        )
    })?;
    if item.is_none() {
        return Err(Error::new(
            "Command::TransactionDelete",
            format!("Transaction with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    match TransactionMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::from_db(
                "Command::TransactionDelete",
                "Failed to delete transaction by ID: {}",
                e,
                get_location!(),
            ));
        }
    }

    Ok(item)
}
#[tauri::command]
pub async fn transaction_delete_bulk(ids: Vec<i64>) -> Result<u64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut deleted_count = 0;
    for id in ids {
        match TransactionMutation::delete_by_id(conn, id).await {
            Ok(e) => {
                info(
                    "Command::TransactionDeleteBulk",
                    format!("Deleted transaction with ID: {}", id),
                    &LoggerOptions::default(),
                );
                deleted_count += e.rows_affected;
            }
            Err(e) => {
                return Err(Error::from_db(
                    "Command::TransactionDelete",
                    "Failed to delete transaction by ID: {}",
                    e,
                    get_location!(),
                ));
            }
        }
    }

    Ok(deleted_count)
}

#[tauri::command]
pub async fn transaction_update(input: UpdateTransaction) -> Result<transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match TransactionMutation::update_by_id(conn, input).await {
        Ok(transaction) => Ok(transaction),
        Err(e) => {
            return Err(Error::from_db(
                "Command::TransactionUpdate",
                "Failed to get transaction by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}
#[tauri::command]
pub async fn export_transaction_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: TransactionPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        e.log("export_transaction_json.log");
        return Err(e);
    }
    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match TransactionQuery::get_all(conn, query).await {
        Ok(transaction) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_Transactions", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&transaction.results).map_err(|e| {
                    Error::new(
                        "Command::ExportTransactionJson",
                        format!("Failed to serialize transactions to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportTransactionJson",
                        format!("Failed to write transactions to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportTransactionJson",
                    format!("Exported transactions to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => {
            return Err(Error::from_db(
                "Command::TransactionUpdate",
                "Failed to get transaction by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}
