use std::sync::{Arc, Mutex};

use create::CreateStockItem;
use entity::stock::item::*;
use entity::sub_type::SubType;
use entity::transaction::transaction::TransactionType;
use eyre::eyre;
use serde_json::json;
use service::{StockItemMutation, StockItemQuery, TransactionMutation};

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

#[tauri::command]
pub async fn stock_item_reload(
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    match StockItemQuery::get_all(&app.conn).await {
        Ok(rivens) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::Set,
                Some(json!(rivens)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("StockItemQuery::reload", e);
            error::create_log_file("command.log".to_string(), &error);
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
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let mut created_stock = CreateStockItem::new(
        wfm_url,
        sub_type.clone(),
        Some(bought),
        minimum_price,
        quantity,
        false,
    );

    // Validate the item and get the stock item
    let stock = match cache
        .tradable_items()
        .validate_create_item(&mut created_stock, "--item_by url_name --item_lang en")
    {
        Ok(stock) => stock.to_stock(),
        Err(e) => {
            return Err(e);
        }
    };

    // Add the stock item to the database and send the update to the UI
    match StockItemMutation::add_item(&app.conn, stock.clone()).await {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
    }

    // Process the order on WFM
    match wfm
        .orders()
        .progress_order(
            &stock.wfm_url,
            stock.clone().sub_type,
            quantity,
            OrderType::Buy,
            false,
        )
        .await
    {
        Ok((operation, order)) => {
            if operation == "order_deleted" {
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": order.unwrap().id })),
                );
            } else if operation == "order_updated" {
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(order)),
                );
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    // Ship Creating the transaction if bought is 0
    if bought == 0 {
        return Ok(stock);
    }

    // Add Transaction to the database
    match TransactionMutation::create(
        &app.conn,
        stock.to_transaction(
            "",
            created_stock.tags,
            quantity,
            bought,
            TransactionType::Purchase,
        ),
    )
    .await
    {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
    }

    Ok(stock)
}

#[tauri::command]
pub async fn stock_item_update(
    id: i64,
    bought: Option<i64>,
    quantity: Option<i64>,
    minimum_price: Option<i64>,
    sub_type: Option<SubType>,
    is_hidden: Option<bool>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    let stock = match StockItemQuery::find_by_id(&app.conn, id).await {
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

    match StockItemMutation::update_by_id(&app.conn, new_item.id, new_item.clone()).await {
        Ok(updated) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemUpdate", eyre!(e))),
    }

    Ok(new_item)
}
#[tauri::command]
pub async fn stock_item_update_bulk(
    ids: Vec<i64>,
    minimum_price: Option<i64>,
    is_hidden: Option<bool>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<i64, AppError> {
    let mut total: i64 = 0;
    for id in ids {
        match stock_item_update(
            id,
            None,
            None,
            minimum_price,
            None,
            is_hidden,
            app.clone(),
            notify.clone(),
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
pub async fn stock_item_delete_bulk(
    ids: Vec<i64>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<i64, AppError> {
    let mut total: i64 = 0;
    for id in ids {
        match stock_item_delete(id, app.clone(), notify.clone(), wfm.clone()).await {
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
pub async fn stock_item_sell(
    id: i64,
    quantity: i64,
    price: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let cache = cache.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    // Sell the item
    let stock_item = match StockItemMutation::sold_by_id(&app.conn, id, quantity).await {
        Ok((operation, item)) => {
            if operation == "Item not found" {
                return Err(AppError::new(
                    "StockItemSell",
                    eyre!(format!("Stock Item not found: {}", id)),
                ));
            } else if operation == "Item deleted" {
                notify.gui().send_event_update(
                    UIEvent::UpdateStockItems,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            } else if operation == "Item updated" {
                notify.gui().send_event_update(
                    UIEvent::UpdateStockItems,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(item)),
                );
            }
            item.unwrap()
        }
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    };

    // Process the order on WFM
    match wfm
        .orders()
        .progress_order(
            &stock_item.wfm_url,
            stock_item.clone().sub_type,
            quantity,
            OrderType::Sell,
            true,
        )
        .await
    {
        Ok((operation, order)) => {
            if operation == "order_deleted" && order.is_some() {
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": order.unwrap().id })),
                );
            } else if operation == "order_updated" {
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(order)),
                );
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    let item_info = cache
        .tradable_items()
        .get_by(&stock_item.wfm_url, "--item_by url_name --item_lang en")
        .or_else(|e| {
            return Err(e);
        })?;
    let item_info = item_info.unwrap();

    // Add Transaction to the database
    let transaction =
        stock_item.to_transaction("", item_info.tags, quantity, price, TransactionType::Sale);

    match TransactionMutation::create(&app.conn, transaction).await {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
    }

    Ok(stock_item)
}

#[tauri::command]
pub async fn stock_item_sell_by_wfm_order(
    url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<stock_item::Model, AppError> {
    let app_state = app.lock()?.clone();

    // Get the stock item returned None if not found
    let stock_item = match StockItemQuery::find_by_url_name_and_sub_type(
        &app_state.conn,
        &url,
        sub_type,
    )
    .await
    {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockItemSellByWFMOrder", eyre!(e))),
    };

    if stock_item.is_none() {
        return Err(AppError::new(
            "StockItemSellByWFMOrder",
            eyre!(format!("Stock Item not found: {}", url)),
        ));
    }
    let stock_item = stock_item.unwrap();
    stock_item_sell(stock_item.id, quantity, price, app, cache, notify, wfm).await?;
    Ok(stock_item)
}

#[tauri::command]
pub async fn stock_item_delete(
    id: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let stock_item = match StockItemQuery::find_by_id(&app.conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    };

    if stock_item.is_none() {
        return Err(AppError::new(
            "StockItemDelete",
            eyre!(format!("Stock Item not found: {}", id)),
        ));
    }
    let stock_item = stock_item.unwrap();

    match StockItemMutation::delete_by_id(&app.conn, id).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                notify.gui().send_event_update(
                    UIEvent::UpdateStockItems,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
        }
        Err(e) => return Err(AppError::new("StockItemDelete", eyre!(e))),
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
