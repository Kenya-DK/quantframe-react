use std::sync::{Arc, Mutex};

use entity::stock::riven::*;
use entity::{
    enums::stock_status::StockStatus, sub_type::SubType,
};

use eyre::eyre;
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery, TransactionMutation};

use crate::utils::modules::error;
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
                post_price as i32,
                0,
                &stock.comment.clone(),
                post_price as i32,
                true,
            )
            .await
        {
            Ok(updated) => {
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(updated)),
                );
            }
            Err(e) => return Err(e),
        }
    }

    match StockRivenMutation::update_by_id(&app.conn, stock.id, stock.clone()).await {
        Ok(updated) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemUpdate", eyre!(e))),
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
    let mut total: i64 = 0;
    for id in ids {
        match stock_riven_update(
            id,
            minimum_price,
            None,
            is_hidden,
            None,
            app.clone(),
            notify.clone(),
            wfm.clone(),
        )
        .await
        {
            Ok(_) => {
                total += 1;
            }
            Err(e) => {
                return Err(e);
            }
        }
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
    let mut total: i64 = 0;
    for id in ids {
        match stock_riven_delete(id, app.clone(), notify.clone(), wfm.clone()).await {
            Ok(_) => {
                total += 1;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(total)
}

#[tauri::command]
pub async fn stock_riven_sell(
    id: i64,
    price: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_riven::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();
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

    // Delete the auction from WFM
    if stock.wfm_order_id.is_some() {
        match wfm
            .auction()
            .delete(&stock.wfm_order_id.clone().unwrap())
            .await
        {
            Ok(auction) => {
                if auction.is_some() {
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
                    return Err(e);
                }
            }
        }
    }

    // Add Transaction to the database
    let transaction = stock.to_transaction(
        "",
        price,
        entity::transaction::transaction::TransactionType::Sale,
    );

    match TransactionMutation::create(&app.conn, transaction).await {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    }

    // Delete the stock from the database
    match StockRivenMutation::delete(&app.conn, stock.id).await {
        Ok(_) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::Delete,
                Some(json!({ "id": stock.id })),
            );
        }
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    }

    Ok(stock)
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
                    return Err(e);
                }
            }
        }
    }
    match StockRivenMutation::delete(&app.conn, stock_item.id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
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
