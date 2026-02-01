use std::sync::Mutex;

use entity::{
    dto::*,
    enums::{FieldChange, TransactionItemType},
    transaction::{dto::TransactionPaginationQueryDto, *},
};
use serde_json::json;
use service::{TransactionMutation, TransactionQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, sorting::SortDirection, warning, Error, LoggerOptions};

use crate::{add_metric, app::client::AppState, types::PermissionsFlags, APP, DATABASE};

#[tauri::command]
pub async fn get_transaction_pagination(
    query: TransactionPaginationQueryDto,
) -> Result<PaginatedResult<transaction::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match TransactionQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn get_transaction_financial_report(
    query: TransactionPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_transaction_pagination(query.clone()).await?.results;

    let mut trading_partners = group_by(&items, |item| {
        if item.user_name == "" {
            "Unknown".to_string()
        } else {
            item.user_name.clone()
        }
    });
    // Remove Unknown trading partners
    trading_partners.remove("Unknown");
    let mut trading_partners = trading_partners
        .iter()
        .map(|(name, items)| {
            FinancialReport::from(items).with_properties(json!({
                "user": name,
            }))
        })
        .collect::<Vec<FinancialReport>>();
    trading_partners.sort_by(|a, b| b.total_transactions.cmp(&a.total_transactions));

    let mut report = FinancialReport::from(&items);
    if let Some(properties) = &mut report.properties {
        properties["trading_partners"] =
            serde_json::to_value(trading_partners.into_iter().take(10).collect::<Vec<_>>())
                .unwrap();
    }
    Ok(report)
}

#[tauri::command]
pub async fn transaction_delete(id: i64) -> Result<transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = TransactionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| e.with_location(get_location!()))?;
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
        Err(e) => return Err(e.with_location(get_location!())),
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
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }

    Ok(deleted_count)
}

#[tauri::command]
pub async fn transaction_update(input: UpdateTransaction) -> Result<transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match TransactionMutation::update_by_id(conn, input).await {
        Ok(transaction) => Ok(transaction),
        Err(e) => return Err(e.with_location(get_location!())),
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
                add_metric!("export_transaction_json", "success");
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => return Err(e.with_location(get_location!())),
    }
}

#[tauri::command]
pub async fn transaction_calculate_tax(
    cache: tauri::State<'_, Mutex<crate::cache::CacheState>>,
) -> Result<(), Error> {
    let conn = DATABASE.get().unwrap();
    let cache = cache.lock()?.clone();
    let items = get_transaction_pagination(TransactionPaginationQueryDto::new(1, -1))
        .await?
        .results;
    for item in items {
        let mut update_data = UpdateTransaction::new(item.id);
        match item.transaction_type {
            entity::enums::TransactionType::Sale => {
                update_data.credits = entity::enums::FieldChange::Value(
                    item.price * crate::enums::TradeItemType::Platinum.to_tax(),
                );
            }
            entity::enums::TransactionType::Purchase => {
                if item.item_type == TransactionItemType::Riven {
                    update_data.credits = entity::enums::FieldChange::Value(
                        item.quantity * crate::enums::TradeItemType::RivenVeiled.to_tax(),
                    );
                } else {
                    match cache.tradable_item().get_by(&item.wfm_id) {
                        Ok(tradable_item) => {
                            update_data.credits =
                                entity::enums::FieldChange::Value(tradable_item.trade_tax);
                        }
                        Err(e) => {
                            warning(
                                "Command::TransactionCalculateTax",
                                format!(
                                    "Failed to get tradable item for WFM ID {}: {}",
                                    item.wfm_id, e
                                ),
                                &LoggerOptions::default(),
                            );
                        }
                    }
                }
            }
        }
        if let Err(e) = TransactionMutation::update_by_id(conn, update_data).await {
            warning(
                "Command::TransactionCalculateTax",
                format!("Failed to update transaction {}: {}", item.item_name, e),
                &LoggerOptions::default(),
            );
        }
    }
    Ok(())
}
