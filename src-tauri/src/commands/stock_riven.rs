use std::{collections::HashMap, sync::Mutex};

use entity::{dto::*, stock_riven::*};
use serde_json::{json, Value};
use service::{StockItemMutation, StockItemQuery, StockRivenMutation, StockRivenQuery};
use utils::{get_location, group_by, info, Error};
use wf_market::enums::OrderType;

use crate::{
    app::client::AppState,
    handlers::{handle_riven, handle_riven_by_entity, stock_item::handle_item},
    utils::{ErrorFromExt, SubTypeExt},
    DATABASE,
};

#[tauri::command]
pub async fn get_stock_riven_pagination(
    query: StockRivenPaginationQueryDto,
) -> Result<PaginatedDto<stock_riven::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match StockRivenQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_db(
                "StockRivenQuery::get_stock_riven",
                "Failed to get stock riven: {}",
                e,
                get_location!(),
            );
            return Err(error);
        }
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
    match handle_riven_by_entity(input, "", OrderType::Buy).await {
        Ok((operations, updated_item)) => {
            info(
                "Command::StockRivenCreate",
                &format!("Operations: {:?}", operations),
                &utils::LoggerOptions::default(),
            );
            return Ok(updated_item);
        }
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log(Some("stock_riven_create.log")));
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
    )
    .await
    {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log(Some("stock_riven_create.log")));
        }
    }
}

#[tauri::command]
pub async fn stock_riven_delete(
    id: i64,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    let conn = DATABASE.get().unwrap();

    let item = StockRivenQuery::get_by_id(conn, id).await.map_err(|e| {
        Error::from_db(
            "Command::StockRivenDelete",
            "Failed to get stock riven by ID: {}",
            e,
            get_location!(),
        )
    })?;
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

    match StockRivenMutation::delete(conn, id).await {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockRivenDelete",
                "Failed to delete stock riven by ID: {}",
                e,
                get_location!(),
            ));
        }
    }

    Ok(())
}
#[tauri::command]
pub async fn stock_riven_update(input: UpdateStockRiven) -> Result<stock_riven::Model, Error> {
    let conn = DATABASE.get().unwrap();

    match StockRivenMutation::update_by_id(conn, input).await {
        Ok(stock_riven) => Ok(stock_riven),
        Err(e) => {
            return Err(Error::from_db(
                "Command::StockRivenUpdate",
                "Failed to get stock riven by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}

#[tauri::command]
pub async fn stock_riven_get_by_id(id: i64) -> Result<Value, Error> {
    Ok(json!({}))
}
