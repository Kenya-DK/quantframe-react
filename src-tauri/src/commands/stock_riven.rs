use std::sync::{Arc, Mutex};

use create::CreateStockRiven;
use entity::stock::riven::*;
use entity::{enums::stock_status::StockStatus, sub_type::SubType};

use eyre::eyre;
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::cache::client::CacheClient;
use crate::helper;
use crate::qf_client::client::QFClient;
use crate::utils::modules::error;
use crate::wfm_client::enums::order_type::OrderType;
use crate::{
    app::client::AppState,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::{error::AppError, logger},
    },
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn stock_riven_reload(
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    match StockRivenQuery::get_all(&app.conn).await {
        Ok(rivens) => {
            helper::add_metric("Stock_RivenReload", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!(rivens)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockRivenQuery::reload", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };
    Ok(())
}

#[tauri::command]
pub async fn stock_riven_update(
    id: i64,
    minimum_price: Option<i64>,
    sub_type: Option<SubType>,
    is_hidden: Option<bool>,
    filter: Option<match_riven::MatchRivenStruct>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_riven::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let stock = match StockRivenMutation::find_by_id(&app.conn, id).await {
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
                error::create_log_file("command_stock_riven_update.log".to_string(), &e);
                return Err(e);
            }
        }
    }

    match StockRivenMutation::update_by_id(&app.conn, stock.id, stock.clone()).await {
        Ok(updated) => {
            helper::add_metric("Stock_RivenUpdate", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => return Err(AppError::new_db("StockItemUpdate", e)),
    }

    Ok(stock)
}

#[tauri::command]
pub async fn stock_riven_update_bulk(
    ids: Vec<i64>,
    minimum_price: Option<i64>,
    is_hidden: Option<bool>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let mut total: i64 = 0;

    let items =
        match StockRivenMutation::update_bulk(&app.conn, ids.clone(), minimum_price, is_hidden)
            .await
        {
            Ok(updated) => {
                helper::add_metric("Stock_RivenUpdateBulk", "manual");
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::Set,
                    Some(json!(updated)),
                );
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
            Ok(updated) => {
                helper::add_metric("WFM_RivenUpdate", "stock_riven");
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(updated)),
                );
            }
            Err(e) => {
                error::create_log_file("command_stock_riven_update_bulk.log".to_string(), &e);
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
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let wfm = wfm.lock()?.clone();
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    helper::add_metric("Stock_RivenDeleteBulk", "manual");

    let stocks = match StockRivenQuery::find_by_ids(&app.conn, ids.clone()).await {
        Ok(stocks) => stocks,
        Err(e) => {
            let error: AppError = AppError::new_db("StockRivenDeleteBulk", e);
            error::create_log_file("command_stock_riven_delete_bulk.log".to_string(), &error);
            return Err(error);
        }
    };

    let total = stocks.clone().len() as i64;

    for stock in stocks {
        match StockRivenMutation::delete(&app.conn, stock.id).await {
            Ok(deleted) => {
                if deleted.rows_affected > 0 {
                    notify.gui().send_event_update(
                        UIEvent::UpdateStockRivens,
                        UIOperationEvent::Delete,
                        Some(json!({ "id": stock.id })),
                    );
                }
            }
            Err(e) => {
                let error: AppError = AppError::new_db("StockRivenDeleteBulk", e);
                error::create_log_file("command_stock_riven_delete_bulk.log".to_string(), &error);
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
                        logger::warning_con(
                            "StockRivenSell",
                            format!("Error deleting auction: {}", e.cause()).as_str(),
                        );
                    } else {
                        error::create_log_file(
                            "command_stock_riven_delete_bulk.log".to_string(),
                            &e,
                        );
                        return Err(e);
                    }
                }
            }
        }
    }

    match StockRivenQuery::get_all(&app.conn).await {
        Ok(rivens) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!(rivens)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockRivenQuery::reload", e);
            error::create_log_file("command_stock_riven_delete_bulk.log".to_string(), &error);
            return Err(error);
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
            error::create_log_file("command_stock_riven_delete_bulk.log".to_string(), &e);
            return Err(e);
        }
    }

    Ok(total)
}

#[tauri::command]
pub async fn stock_riven_create(
    mut riven_entry: CreateStockRiven,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<entity::stock::riven::stock_riven::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let cache = cache.lock()?.clone();
    let qf = qf.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    match helper::progress_stock_riven(
        &mut riven_entry,
        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
        "",
        OrderType::Buy,
        "manual",
        &app,
        &cache,
        &notify,
        &wfm,
        &qf,
    )
    .await
    {
        Ok((stock, _)) => {
            return Ok(stock);
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_create.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_riven_sell(
    id: i64,
    price: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<stock_riven::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let qf = qf.lock()?.clone();
    let cache = cache.lock()?.clone();
    let stock = match StockRivenMutation::find_by_id(&app.conn, id).await {
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
        &app,
        &cache,
        &notify,
        &wfm,
        &qf,
    )
    .await
    {
        Ok((stock, _)) => {
            return Ok(stock);
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_sell.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_riven_delete(
    id: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let stock_item = match StockRivenMutation::find_by_id(&app.conn, id).await {
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
                    logger::info_con(
                        "StockRivenSell",
                        format!("Error deleting auction: {}", e.cause()).as_str(),
                    );
                } else {
                    error::create_log_file("command_stock_riven_delete.log".to_string(), &e);
                    return Err(e);
                }
            }
        }
    }
    match StockRivenMutation::delete(&app.conn, stock_item.id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                helper::add_metric("Stock_RivenDelete", "manual");
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
        }
        Err(e) => return Err(AppError::new("StockRivenDelete", eyre!(e))),
    }
    Ok(())
}
