use std::{collections::HashMap, sync::Mutex};

use entity::{dto::*, stock_item::*};
use serde_json::{json, Value};
use service::{StockItemMutation, StockItemQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, Error, LoggerOptions};
use wf_market::enums::OrderType;

use crate::{
    app::client::AppState,
    enums::FindByType,
    handlers::{handle_item_by_entity, handle_wfm_item, stock_item::handle_item},
    helper::{self},
    types::PermissionsFlags,
    APP, DATABASE,
};

#[tauri::command]
pub async fn get_stock_item_pagination(
    query: StockItemPaginationQueryDto,
) -> Result<PaginatedResult<stock_item::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match StockItemQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn get_stock_item_financial_report(
    query: StockItemPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_stock_item_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn get_stock_item_status_counts(
    query: StockItemPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_stock_item_pagination(query).await?;
    Ok(group_by(&items.results, |item| item.status.to_string())
        .iter()
        .map(|(status, items)| (status.clone(), items.len()))
        .collect::<HashMap<_, _>>())
}

#[tauri::command]
pub async fn stock_item_create(
    input: CreateStockItem,
    by: Option<String>,
) -> Result<stock_item::Model, Error> {
    let find_by_type = if by.is_some() {
        FindByType::from_str(by.unwrap().as_str())?
    } else {
        FindByType::Url
    };
    match handle_item_by_entity(input, "", OrderType::Buy, find_by_type, &[]).await {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log("stock_item_create.log"));
        }
    }
}

#[tauri::command]
pub async fn stock_item_sell(
    wfm_url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    by: Option<String>,
) -> Result<stock_item::Model, Error> {
    let find_by_type = if by.is_some() {
        FindByType::from_str(by.unwrap().as_str())?
    } else {
        FindByType::Url
    };
    match handle_item(
        wfm_url,
        sub_type,
        quantity,
        price,
        "",
        OrderType::Sell,
        find_by_type,
        &[],
    )
    .await
    {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e.with_location(get_location!()).log("stock_item_sell.log"));
        }
    }
}

#[tauri::command]
pub async fn stock_item_delete(id: i64) -> Result<stock_item::Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = StockItemQuery::find_by_id(conn, id)
        .await
        .map_err(|e| e.with_location(get_location!()))?;
    if item.is_none() {
        return Err(Error::new(
            "Command::StockItemDelete",
            format!("Stock item with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    handle_wfm_item(&item.wfm_id, &item.sub_type, 1, OrderType::Sell, true)
        .await
        .map_err(|e| {
            e.with_location(get_location!())
                .log("stock_item_delete.log")
        })?;

    match StockItemMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => return Err(e.with_location(get_location!())),
    }

    Ok(item)
}

#[tauri::command]
pub async fn stock_item_update(input: UpdateStockItem) -> Result<stock_item::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match StockItemMutation::update_by_id(conn, input).await {
        Ok(stock_item) => Ok(stock_item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}

#[tauri::command]
pub async fn stock_item_get_by_id(id: i64) -> Result<Value, Error> {
    let conn = DATABASE.get().unwrap();
    let item = match StockItemQuery::find_by_id(conn, id).await {
        Ok(stock_item) => {
            if let Some(item) = stock_item {
                item
            } else {
                return Err(Error::new(
                    "Command::StockItemGetById",
                    "Stock item not found",
                    get_location!(),
                ));
            }
        }
        Err(e) => return Err(e.with_location(get_location!())),
    };

    let (mut payload, _, order) = helper::get_item_details(
        FindByType::Id,
        &item.wfm_id,
        item.sub_type.clone(),
        OrderType::Sell,
    )
    .await?;

    if let Some(order) = order {
        payload["potential_profit"] = json!(order.platinum - item.bought as u32);
    }

    payload["stock"] = json!(item);

    Ok(payload)
}
#[tauri::command]
pub async fn export_stock_item_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: StockItemPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        e.log("export_stock_item_json.log");
        return Err(e);
    }

    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match StockItemQuery::get_all(conn, query).await {
        Ok(stock_items) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_Stock_Item", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&stock_items.results).map_err(|e| {
                    Error::new(
                        "Command::ExportStockItemJson",
                        format!("Failed to serialize stock item to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportStockItemJson",
                        format!("Failed to write stock item to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportStockItemJson",
                    format!("Exported stock item to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
