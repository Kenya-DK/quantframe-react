use std::sync::{Arc, Mutex};

use create::CreateStockItem;
use entity::stock::item::*;
use entity::sub_type::SubType;
use eyre::eyre;
use serde_json::json;
use service::{StockItemMutation, StockItemQuery};

use crate::qf_client::client::QFClient;
use crate::utils::modules::error;
use crate::{
    app::client::AppState,
    cache::client::CacheClient,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::AppError,
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType},
};
use crate::{helper, DATABASE};

#[tauri::command]
pub async fn stock_item_reload(
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();

    match StockItemQuery::get_all(conn).await {
        Ok(rivens) => {
            helper::add_metric("Stock_ItemReload", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!(rivens)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemQuery::reload", e);
            error::create_log_file("command_stock_item_reload.log".to_string(), &error);
            return Err(error);
        }
    };
    Ok(())
}
#[tauri::command]
pub async fn stock_item_create(
    wfm_url: String,
    bought: i64,
    minimum_price: Option<i64>,
    sub_type: Option<SubType>,
    quantity: i64,
    is_from_order: bool,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<stock_item::Model, AppError> {
    let cache = cache.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let qf = qf.lock()?.clone();

    let from = if is_from_order {
        "manual_wfm"
    } else {
        "manual"
    };

    let mut created_stock = CreateStockItem::new(
        wfm_url,
        sub_type.clone(),
        Some(bought),
        minimum_price,
        quantity,
        false,
    );
    match helper::progress_stock_item(
        &mut created_stock,
        "--item_by url_name --item_lang en",
        "",
        OrderType::Buy,
        vec![],
        from,
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
            error::create_log_file("command_stock_item_create.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_item_update(
    id: i64,
    bought: Option<i64>,
    quantity: Option<i64>,
    minimum_price: Option<i64>,
    sub_type: Option<SubType>,
    is_hidden: Option<bool>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<stock_item::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();

    let stock = match StockItemQuery::find_by_id(conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockItemUpdate", eyre!(e))),
    };

    if stock.is_none() {
        return Err(AppError::new(
            "StockItemUpdate",
            eyre!(format!("Stock Item not found: {}", id)),
        ));
    }

    let mut new_item = stock.unwrap();

    if let Some(bought) = bought {
        let total_bought = (new_item.bought * new_item.owned) + bought;
        let total_owned = new_item.owned + quantity.unwrap_or(0);
        let weighted_average = total_bought / total_owned;
        new_item.bought = weighted_average;
        new_item.owned = total_owned;
    }

    if let Some(minimum_price) = minimum_price {
        new_item.minimum_price = Some(minimum_price);
    }

    if let Some(sub_type) = sub_type {
        new_item.sub_type = Some(sub_type);
    }

    if let Some(quantity) = quantity {
        new_item.owned = quantity;
    }

    if let Some(is_hidden) = is_hidden {
        new_item.is_hidden = is_hidden;
    }
    new_item.updated_at = chrono::Utc::now();

    match StockItemMutation::update_by_id(conn, new_item.id, new_item.clone()).await {
        Ok(updated) => {
            helper::add_metric("Stock_ItemUpdate", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemMutation::UpdateById", e);
            error::create_log_file("command_stock_item_update.log".to_string(), &error);
            return Err(error);
        }
    }

    Ok(new_item)
}
#[tauri::command]
pub async fn stock_item_update_bulk(
    ids: Vec<i64>,
    minimum_price: Option<i64>,
    is_hidden: Option<bool>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<i64, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();

    let total: i64 = ids.len() as i64;

    match StockItemMutation::update_bulk(conn, ids, minimum_price, is_hidden).await {
        Ok(items) => {
            helper::add_metric("Stock_ItemUpdateBulk", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!(items)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemMutation::UpdateBulk", e);
            error::create_log_file("command_stock_item_update_bulk.log".to_string(), &error);
            return Err(error);
        }
    }

    Ok(total)
}

#[tauri::command]
pub async fn stock_item_delete_bulk(
    ids: Vec<i64>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let wfm = wfm.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    helper::add_metric("Stock_ItemDeleteBulk", "manual");

    let mut my_orders = wfm.orders().get_my_orders().await?;
    let stocks = match StockItemQuery::find_by_ids(conn, ids.clone()).await {
        Ok(stocks) => stocks,
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemQuery::DeleteBulk", e);
            error::create_log_file("command_stock_item_delete_bulk.log".to_string(), &error);
            return Err(error);
        }
    };

    let total = stocks.clone().len() as i64;

    for stock in stocks {
        match StockItemMutation::delete_by_id(conn, stock.id).await {
            Ok(_) => {}
            Err(e) => {
                let error: AppError = AppError::new_db("StockItemMutation::DeleteById", e);
                error::create_log_file("command_stock_item_delete_bulk.log".to_string(), &error);
                return Err(error);
            }
        }
        // Delete the order on WFM
        match my_orders.find_order_by_url_sub_type(
            &stock.wfm_url,
            OrderType::Sell,
            stock.sub_type.as_ref(),
        ) {
            Some(order) => {
                wfm.orders().delete(&order.id).await?;
                my_orders.delete_order_by_id(OrderType::Sell, &order.id);
            }
            None => {}
        }
    }

    // Update the UI
    match StockItemQuery::get_all(conn).await {
        Ok(rivens) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!(rivens)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemQuery::DeleteBulk", e);
            error::create_log_file("command_stock_item_delete_bulk.log".to_string(), &error);
            return Err(error);
        }
    }

    notify.gui().send_event_update(
        UIEvent::UpdateOrders,
        UIOperationEvent::Set,
        Some(json!(my_orders.get_all_orders())),
    );
    Ok(total)
}

#[tauri::command]
pub async fn stock_item_sell(
    url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    is_from_order: bool,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<stock_item::Model, AppError> {
    let notify = notify.lock()?.clone();
    let cache = cache.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let qf = qf.lock()?.clone();

    let from = if is_from_order {
        "manual_wfm"
    } else {
        "manual"
    };

    let mut created_stock = CreateStockItem::new(url, sub_type, Some(price), None, quantity, false);
    match helper::progress_stock_item(
        &mut created_stock,
        "--item_by url_name --item_lang en",
        "",
        OrderType::Sell,
        vec![
            "StockContinueOnError".to_string(),
            "WFMContinueOnError".to_string(),
        ],
        from,
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
            error::create_log_file("command_stock_item_sell.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn stock_item_delete(
    id: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let stock_item = match StockItemQuery::find_by_id(conn, id).await {
        Ok(stock) => stock,
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemMutation::UpdateById", e);
            error::create_log_file("command_stock_item_delete.log".to_string(), &error);
            return Err(error);
        }
    };

    if stock_item.is_none() {
        return Err(AppError::new(
            "StockItemDelete",
            eyre!(format!("Stock Item not found: {}", id)),
        ));
    }
    let stock_item = stock_item.unwrap();

    match StockItemMutation::delete_by_id(conn, id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                helper::add_metric("Stock_ItemDelete", "manual");
                notify.gui().send_event_update(
                    UIEvent::UpdateStockItems,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemMutation::DeleteById", e);
            error::create_log_file("command_stock_item_delete.log".to_string(), &error);
            return Err(error);
        }
    }

    let my_orders = wfm.orders().get_my_orders().await?;
    let order = my_orders.find_order_by_url_sub_type(
        &stock_item.wfm_url,
        OrderType::Sell,
        stock_item.sub_type.as_ref(),
    );
    if order.is_none() {
        return Ok(());
    }
    // Delete the order on WFM
    wfm.orders().delete(&order.clone().unwrap().id).await?;
    notify.gui().send_event_update(
        UIEvent::UpdateOrders,
        UIOperationEvent::Delete,
        Some(json!({ "id": order.clone().unwrap().id })),
    );
    Ok(())
}
