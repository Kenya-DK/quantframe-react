use std::sync::{Arc, Mutex};

use create::CreateStockRiven;
use entity::stock::riven::*;
use entity::{enums::stock_status::StockStatus, sub_type::SubType};

use eyre::eyre;
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::cache::client::CacheClient;
use crate::utils::modules::error;
use crate::utils::modules::logger::LoggerOptions;
use crate::wfm_client::enums::order_type::OrderType;
use crate::{helper, DATABASE};
use crate::{
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::{error::AppError, logger},
    },
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn get_stock_rivens(
    query: entity::stock::riven::dto::StockRivenPaginationQueryDto,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<entity::dto::pagination::PaginatedDto<stock_riven::Model>, AppError> {
    let cache = cache.lock()?.clone();
    println!("get_stock_rivens called with query: {:?}", query);
    let conn = DATABASE.get().unwrap();
    match StockRivenQuery::get_all_v2(conn, query).await {
        Ok(mut items) => {
            // Apply additional information from the cache
            for item in items.results.iter_mut() {
                for attr in item.attributes.0.iter_mut() {
                    let matched = cache.riven().find_wfm_riven_attribute_by(
                        &attr.url_name,
                        "--attribute_by url_name --attribute_lang en",
                    )?;
                    if let Some(c_attr) = matched {
                        attr.effect = Some(c_attr.effect.clone());
                    }
                }
            }
            return Ok(items);
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemQuery::reload", e);
            error::create_log_file("command_stock_item_reload.log", &error);
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn stock_riven_update(
    id: i64,
    minimum_price: Option<i64>,
    sub_type: Option<SubType>,
    is_hidden: Option<bool>,
    filter: Option<match_riven::MatchRivenStruct>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_riven::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let stock = match StockRivenMutation::find_by_id(conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockRivenUpdate", eyre!(e))),
    };

    if stock.is_none() {
        return Err(AppError::new(
            "StockRivenUpdate",
            eyre!(format!("Stock Riven not found: {}", id)),
        ));
    }

    let mut stock = stock.unwrap();

    if let Some(minimum_price) = minimum_price {
        stock.minimum_price = Some(minimum_price);
    }

    if let Some(sub_type) = sub_type {
        stock.sub_type = Some(sub_type);
    }

    if let Some(filter) = filter {
        stock.filter = filter;
    }

    if let Some(is_hidden) = is_hidden {
        stock.is_hidden = is_hidden;
    }
    stock.updated_at = chrono::Utc::now();

    if stock.wfm_order_id.is_some()
        && stock.status == StockStatus::Live
        && stock.list_price.unwrap_or(0) < stock.minimum_price.unwrap_or(0)
    {
        let post_price = stock.minimum_price.unwrap();

        match wfm
            .auction()
            .update(
                stock.wfm_order_id.clone().unwrap().as_str(),
                post_price,
                0,
                &stock.comment.clone(),
                post_price,
                true,
            )
            .await
        {
            Ok(updated) => {
                helper::add_metric("WFM_RivenUpdate", "stock_riven");
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(updated)),
                );
            }
            Err(e) => {
                error::create_log_file("command_stock_riven_update.log", &e);
                return Err(e);
            }
        }
    }

    match StockRivenMutation::update_by_id(conn, stock.id, stock.clone()).await {
        Ok(_) => helper::add_metric("Stock_RivenUpdate", "manual"),
        Err(e) => return Err(AppError::new_db("StockItemUpdate", e)),
    }

    Ok(stock)
}

#[tauri::command]
pub async fn stock_riven_update_bulk(
    ids: Vec<i64>,
    minimum_price: Option<i64>,
    is_hidden: Option<bool>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let conn = DATABASE.get().unwrap();
    let wfm = wfm.lock()?.clone();

    let mut total: i64 = 0;

    let items =
        match StockRivenMutation::update_bulk(conn, ids.clone(), minimum_price, is_hidden).await {
            Ok(updated) => {
                helper::add_metric("Stock_RivenUpdateBulk", "manual");
                updated
                    .into_iter()
                    .filter(|x| {
                        ids.contains(&x.id)
                            && x.wfm_order_id.is_some()
                            && x.status == StockStatus::Live
                            && x.list_price.unwrap_or(0) < x.minimum_price.unwrap_or(0)
                    })
                    .collect::<Vec<stock_riven::Model>>()
            }
            Err(e) => {
                let error: AppError = AppError::new_db("StockRivenUpdateBulk", e);
                return Err(error);
            }
        };

    for stock in items {
        let post_price = stock.minimum_price.unwrap();

        match wfm
            .auction()
            .update(
                stock.wfm_order_id.clone().unwrap().as_str(),
                post_price,
                0,
                &stock.comment.clone(),
                post_price,
                true,
            )
            .await
        {
            Ok(_) => helper::add_metric("WFM_RivenUpdate", "stock_riven"),
            Err(e) => {
                error::create_log_file("command_stock_riven_update_bulk.log", &e);
                return Err(e);
            }
        }
        total += 1;
    }
    Ok(total)
}
#[tauri::command]
pub async fn stock_riven_delete_bulk(
    ids: Vec<i64>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let wfm = wfm.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    helper::add_metric("Stock_RivenDeleteBulk", "manual");

    let stocks = match StockRivenQuery::find_by_ids(conn, ids.clone()).await {
        Ok(stocks) => stocks,
        Err(e) => {
            let error: AppError = AppError::new_db("StockRivenDeleteBulk", e);
            error::create_log_file("command_stock_riven_delete_bulk.log", &error);
            return Err(error);
        }
    };

    let total = stocks.clone().len() as i64;

    for stock in stocks {
        match StockRivenMutation::delete(conn, stock.id).await {
            Ok(_) => {}
            Err(e) => {
                let error: AppError = AppError::new_db("StockRivenDeleteBulk", e);
                error::create_log_file("command_stock_riven_delete_bulk.log", &error);
                return Err(error);
            }
        }

        if stock.wfm_order_id.is_some() {
            match wfm
                .auction()
                .delete(&stock.wfm_order_id.clone().unwrap())
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    if e.cause().contains("app.form.not_exist") {
                        logger::warning(
                            "StockRivenSell",
                            format!("Error deleting auction: {}", e.cause()).as_str(),
                            LoggerOptions::default(),
                        );
                    } else {
                        error::create_log_file("command_stock_riven_delete_bulk.log", &e);
                        return Err(e);
                    }
                }
            }
        }
    }
    match wfm.auction().get_my_auctions().await {
        Ok(auctions) => {
            notify.gui().send_event_update(
                UIEvent::UpdateAuction,
                UIOperationEvent::Set,
                Some(json!(auctions)),
            );
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_delete_bulk.log", &e);
            return Err(e);
        }
    }

    Ok(total)
}

#[tauri::command]
pub async fn stock_riven_create(
    wfm_url: String,
    mod_name: String,
    mastery_rank: i64,
    rank: i64,
    re_rolls: i64,
    polarity: String,
    attributes: Vec<entity::stock::riven::attribute::RivenAttribute>,
    bought: i64,
) -> Result<entity::stock::riven::stock_riven::Model, AppError> {
    let mut riven_entry = CreateStockRiven::new(
        wfm_url,
        mod_name,
        mastery_rank,
        re_rolls,
        polarity,
        attributes,
        rank,
        Some(bought),
        None, // rank
        None, // stock_id
    );

    match helper::progress_stock_riven(
        &mut riven_entry,
        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
        "",
        OrderType::Buy,
        "manual",
    )
    .await
    {
        Ok((stock, _)) => {
            return Ok(stock);
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_create.log", &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_riven_sell(id: i64, price: i64) -> Result<stock_riven::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let stock = match StockRivenMutation::find_by_id(conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockRivenSell", eyre!(e))),
    };

    if stock.is_none() {
        return Err(AppError::new(
            "StockRivenSell",
            eyre!(format!("Stock Riven not found: {}", id)),
        ));
    }
    let stock = stock.unwrap();

    let mut entry = stock.to_create(price);

    match helper::progress_stock_riven(
        &mut entry,
        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
        "",
        OrderType::Sell,
        "manual",
    )
    .await
    {
        Ok((stock, _)) => {
            return Ok(stock);
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_sell.log", &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_riven_delete(
    id: i64,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let stock_item = match StockRivenMutation::find_by_id(conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockRivenDelete", eyre!(e))),
    };

    if stock_item.is_none() {
        return Err(AppError::new(
            "StockRivenDelete",
            eyre!(format!("Stock Riven not found: {}", id)),
        ));
    }
    let stock_item = stock_item.unwrap();

    // Delete the auction from WFM
    if stock_item.wfm_order_id.is_some() {
        match wfm
            .auction()
            .delete(&stock_item.wfm_order_id.clone().unwrap())
            .await
        {
            Ok(auction) => {
                if auction.is_some() {
                    helper::add_metric("WFM_RivenDeleted", "stock_riven");
                    notify.gui().send_event_update(
                        UIEvent::UpdateAuction,
                        UIOperationEvent::Delete,
                        Some(json!({ "id": id })),
                    );
                }
            }
            Err(e) => {
                if e.cause().contains("app.form.not_exist") {
                    logger::info(
                        "StockRivenSell",
                        format!("Error deleting auction: {}", e.cause()).as_str(),
                        LoggerOptions::default(),
                    );
                } else {
                    error::create_log_file("command_stock_riven_delete.log", &e);
                    return Err(e);
                }
            }
        }
    }
    match StockRivenMutation::delete(conn, stock_item.id).await {
        Ok(_) => helper::add_metric("Stock_RivenDelete", "manual"),
        Err(e) => return Err(AppError::new("StockRivenDelete", eyre!(e))),
    }
    Ok(())
}
