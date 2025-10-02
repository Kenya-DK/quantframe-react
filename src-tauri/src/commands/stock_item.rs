use std::{collections::HashMap, str::FromStr, sync::Mutex};

use chrono::Utc;
use entity::{dto::*, stock_item::*, transaction::dto::TransactionPaginationQueryDto};
use serde_json::{json, Value};
use service::{StockItemMutation, StockItemQuery, TransactionQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, Error, GroupByDate, LoggerOptions};
use wf_market::enums::OrderType;

use crate::{
    app::client::AppState,
    cache::client::CacheState,
    enums::{FindBy, FindByType},
    handlers::{handle_item_by_entity, handle_wfm_item, stock_item::handle_item},
    helper::generate_transaction_summary,
    utils::{ErrorFromExt, OrderExt, SubTypeExt},
    APP, DATABASE,
};

#[tauri::command]
pub async fn get_stock_item_pagination(
    query: StockItemPaginationQueryDto,
) -> Result<PaginatedResult<stock_item::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match StockItemQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_db(
                "StockItemQuery::get_stock_items",
                "Failed to get stock items: {}",
                e,
                get_location!(),
            );
            return Err(error);
        }
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

    let item = StockItemQuery::find_by_id(conn, id).await.map_err(|e| {
        Error::from_db(
            "Command::StockItemDelete",
            "Failed to get stock item by ID: {}",
            e,
            get_location!(),
        )
    })?;
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
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockItemDelete",
                "Failed to delete stock item by ID: {}",
                e,
                get_location!(),
            ));
        }
    }

    Ok(item)
}

#[tauri::command]
pub async fn stock_item_update(input: UpdateStockItem) -> Result<stock_item::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match StockItemMutation::update_by_id(conn, input).await {
        Ok(stock_item) => Ok(stock_item),
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockItemUpdate",
                "Failed to get stock item by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}

#[tauri::command]
pub async fn stock_item_get_by_id(
    id: i64,
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();
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
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockItemGetById",
                "Failed to get stock item by ID: {}",
                e,
                get_location!(),
            ))
        }
    };

    let transaction_paginate = TransactionQuery::get_all(
        conn,
        TransactionPaginationQueryDto::new(1, -1)
            .set_wfm_id(&item.wfm_id)
            .set_sub_type(item.sub_type.clone()),
    )
    .await
    .map_err(|e| {
        Error::from_db(
            "Command::StockItemGetById",
            "Failed to get transactions: {}",
            e,
            get_location!(),
        )
    })?;

    let order = app.wfm_client.order().cache_orders().find_order(
        &item.wfm_id,
        &SubTypeExt::from_entity(item.sub_type.clone()),
        OrderType::Sell,
    );

    let mut payload = json!(FinancialReport::from(&transaction_paginate.results));
    payload["item_info"] = json!(cache
        .tradable_item()
        .get_by(FindBy::new(FindByType::Url, &item.wfm_url))?);
    payload["stock"] = json!(item);
    if let Some(mut order_info) = order {
        let mut details = order_info.get_details();
        let mut orders = details.orders;
        if !orders.is_empty() {
            for ord in orders.iter_mut() {
                ord.order.apply_item_info(&cache)?;
            }
            details.orders = orders;
            order_info.update_details(details);
        }
        payload["order_info"] = json!(order_info);
        payload["stock_profit"] = json!(order_info.platinum - item.bought as u32);
    }
    payload["last_transactions"] = json!(transaction_paginate.take_top(5));

    Ok(payload)
}
#[tauri::command]
pub async fn export_stock_item_json(
    mut query: StockItemPaginationQueryDto,
) -> Result<String, Error> {
    let app = APP.get().unwrap();
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
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockItemUpdate",
                "Failed to get stock item by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}
