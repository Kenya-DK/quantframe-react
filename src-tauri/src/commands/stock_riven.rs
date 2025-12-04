use std::{collections::HashMap, sync::Mutex};

use entity::{dto::*, stock_riven::*, transaction::TransactionPaginationQueryDto};
use serde_json::{json, Value};
use service::{StockRivenMutation, StockRivenQuery, TransactionQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, Error, LoggerOptions};
use wf_market::enums::OrderType;

use crate::{
    add_metric,
    app::client::AppState,
    cache::types::RivenSummary,
    enums::FindByType,
    handlers::{handle_riven, handle_riven_by_entity},
    types::PermissionsFlags,
    utils::ErrorFromExt,
    APP, DATABASE,
};

#[tauri::command]
pub async fn get_stock_riven_pagination(
    query: StockRivenPaginationQueryDto,
) -> Result<PaginatedResult<stock_riven::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match StockRivenQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn get_stock_riven_financial_report(
    query: StockRivenPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_stock_riven_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn get_stock_riven_status_counts(
    query: StockRivenPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_stock_riven_pagination(query).await?;
    Ok(group_by(&items.results, |item| item.status.to_string())
        .iter()
        .map(|(status, items)| (status.clone(), items.len()))
        .collect::<HashMap<_, _>>())
}

#[tauri::command]
pub async fn stock_riven_create(input: CreateStockRiven) -> Result<stock_riven::Model, Error> {
    match handle_riven_by_entity(
        input,
        "",
        OrderType::Buy,
        FindByType::Url,
        FindByType::Url,
        &[],
    )
    .await
    {
        Ok((operations, updated_item)) => {
            info(
                "Command::StockRivenCreate",
                &format!("Operations: {:?}", operations.operations),
                &utils::LoggerOptions::default(),
            );
            add_metric!("stock_riven_create", "success");
            return Ok(updated_item);
        }
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log("stock_riven_create.log"));
        }
    }
}

#[tauri::command]
pub async fn stock_riven_sell(
    wfm_url: String,
    mod_name: String,
    mastery_rank: i64,
    rank: i64,
    re_rolls: i64,
    polarity: String,
    attributes: Vec<RivenAttribute>,
    bought: i64,
) -> Result<stock_riven::Model, Error> {
    match handle_riven(
        wfm_url,
        mod_name,
        mastery_rank,
        rank,
        re_rolls,
        polarity,
        attributes,
        bought,
        "",
        OrderType::Sell,
        FindByType::Url,
        FindByType::Url,
        &[],
    )
    .await
    {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log("stock_riven_create.log"));
        }
    }
}

#[tauri::command]
pub async fn stock_riven_delete(
    id: i64,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<stock_riven::Model, Error> {
    let app = app.lock()?.clone();
    let conn = DATABASE.get().unwrap();

    let item = StockRivenQuery::get_by_id(conn, id)
        .await
        .map_err(|e| e.with_location(get_location!()))?;
    if item.is_none() {
        return Err(Error::new(
            "Command::StockRivenDelete",
            format!("Stock riven with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    match app
        .wfm_client
        .auction()
        .cache_auctions()
        .get_by_uuid(&item.uuid)
    {
        Some(auction) => {
            app.wfm_client
                .auction()
                .delete(&auction.id)
                .await
                .map_err(|e| {
                    Error::from_wfm(
                        "Command::StockRivenDelete",
                        "Failed to delete auction associated with stock riven",
                        e,
                        get_location!(),
                    )
                })?;
        }
        None => {}
    }
    add_metric!("stock_riven_delete", "manual");
    match StockRivenMutation::delete(conn, id).await {
        Ok(_) => {}
        Err(e) => return Err(e.with_location(get_location!())),
    }

    Ok(item)
}
#[tauri::command]
pub async fn stock_riven_update(input: UpdateStockRiven) -> Result<stock_riven::Model, Error> {
    let conn = DATABASE.get().unwrap();

    match StockRivenMutation::update_by_id(conn, input).await {
        Ok(stock_riven) => Ok(stock_riven),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
#[tauri::command]
pub async fn stock_riven_delete_multiple(ids: Vec<i64>) -> Result<i64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut deleted_count = 0;
    for id in ids {
        match StockRivenMutation::delete(conn, id).await {
            Ok(_) => deleted_count += 1,
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    Ok(deleted_count)
}
#[tauri::command]
pub async fn stock_riven_update_multiple(
    ids: Vec<i64>,
    input: UpdateStockRiven,
) -> Result<Vec<stock_riven::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    let mut updated_items = Vec::new();

    for id in ids {
        let mut update_input = input.clone();
        update_input.id = id;
        match StockRivenMutation::update_by_id(conn, update_input).await {
            Ok(stock_riven) => updated_items.push(stock_riven),
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    Ok(updated_items)
}
#[tauri::command]
pub async fn stock_riven_get_by_id(
    id: i64,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let item = match StockRivenQuery::get_by_id(conn, id).await {
        Ok(stock_riven) => {
            if let Some(item) = stock_riven {
                item
            } else {
                return Err(Error::new(
                    "Command::StockRivenGetById",
                    "Stock riven not found",
                    get_location!(),
                ));
            }
        }
        Err(e) => return Err(e.with_location(get_location!())),
    };

    let transaction_paginate = TransactionQuery::get_all(
        conn,
        TransactionPaginationQueryDto::new(1, -1).set_wfm_id(&item.wfm_weapon_id),
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    let auction = app
        .wfm_client
        .auction()
        .cache_auctions()
        .get_by_uuid(&item.uuid);

    let mut payload = json!(FinancialReport::from(&transaction_paginate.results));
    payload["stock"] = json!(item);
    payload["auction_info"] = json!(auction);
    payload["last_transactions"] = json!(transaction_paginate.take_top(5));

    if auction.is_some() {
        let order_info = auction.unwrap();
        payload["stock_profit"] = json!(order_info.starting_price - item.bought as i32);
    }

    let attributes = item
        .attributes
        .0
        .iter()
        .map(|a| (a.url_name.clone(), a.value, a.positive))
        .collect::<Vec<_>>();

    // let summary = cache
    //     .riven_parser()
    //     .create_summary(
    //         FindBy::new(FindByType::UniqueName, item.weapon_unique_name.clone()),
    //         item.mastery_rank,
    //         item.re_rolls,
    //         item.sub_type.unwrap().rank.unwrap_or(0),
    //         item.polarity.clone(),
    //         attributes,
    //     )
    //     .map_err(|e| e.with_location(get_location!()))?;
    // payload["riven_summary"] = json!(summary);
    payload["riven_summary"] = json!(RivenSummary::new(
        item.weapon_unique_name,
        item.mastery_rank,
        item.re_rolls,
        item.sub_type.unwrap().rank.unwrap_or(0),
        item.polarity.clone(),
        attributes
    )?);
    Ok(payload)
}

#[tauri::command]
pub async fn export_stock_riven_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: StockRivenPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        e.log("export_stock_riven_json.log");
        return Err(e);
    }
    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match StockRivenQuery::get_all(conn, query).await {
        Ok(stock_riven) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_Stock_Riven", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&stock_riven.results).map_err(|e| {
                    Error::new(
                        "Command::ExportStockRivenJson",
                        format!("Failed to serialize stock riven to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportStockRivenJson",
                        format!("Failed to write stock riven to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportStockRivenJson",
                    format!("Exported stock riven to JSON file: {}", file_path),
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
