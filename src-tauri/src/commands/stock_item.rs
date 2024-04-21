use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use entity::{enums::stock_status::StockStatus, price_history::PriceHistoryVec, sub_type::SubType, transaction::TransactionItemType};
use eyre::eyre;
use serde_json::{json, Value};
use service::{StockItemMutation, TransactionMutation};

use crate::{
    app::client::AppState,
    cache::client::CacheClient,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::{error::AppError, logger},
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType, types::order_by_item},
};

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
) -> Result<entity::stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();
    let notify = notify.lock()?.clone();
    let item = cache
        .tradable_items()
        .find_by_url_name(&wfm_url)
        .ok_or_else(|| {
            AppError::new(
                "StockItemCreate",
                eyre!(format!("Item not found: {}", wfm_url)),
            )
        })?;

    // Check if the stock item already exists in the database
    let found_stock = match StockItemMutation::find_by_url_name_and_sub_type(
        &app.conn,
        &item.wfm_url_name,
        sub_type.clone(),
    )
    .await
    {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
    };
    let new_item = if found_stock.is_some() {
        let mut stock = found_stock.unwrap();
        let total_owned = stock.owned + quantity;

        // Get Price Per Unit
        let total_bought = (stock.bought * stock.owned) + bought;
        let weighted_average = total_bought / total_owned;
        stock.owned = total_owned;
        stock.bought = weighted_average;
        stock.updated_at = chrono::Utc::now();
        match StockItemMutation::update_by_id(&app.conn, stock.id, stock.clone()).await {
            Ok(updated) => updated,
            Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
        }
    } else {
        let stock_item = entity::stock_item::Model::new(
            item.wfm_id,
            item.wfm_url_name,
            item.name,
            item.unique_name,
            sub_type,
            bought,
            minimum_price,
            quantity,
        );
        // Add the stock item to the database
        match StockItemMutation::create(&app.conn, stock_item.clone()).await {
            Ok(inserted) => inserted,
            Err(e) => return Err(AppError::new("StockItemCreate", eyre!(e))),
        }
    };

    // Send Update to the UI
    notify.gui().send_event_update(
        UIEvent::UpdateStockItems,
        UIOperationEvent::CreateOrUpdate,
        Some(json!(new_item)),
    );

    if bought == 0 {
        return Ok(new_item);
    }
    // Add Transaction to the database
    let transaction = entity::transaction::Model::new(
        new_item.wfm_id.clone(),
        new_item.wfm_url.clone(),
        new_item.item_name.clone(),
        TransactionItemType::Item,
        new_item.item_unique_name.clone(),
        new_item.sub_type.clone(),
        item.tags,
        entity::transaction::TransactionType::Purchase,
        quantity,
        "".to_string(),
        bought,
        None,
    );

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

    // Send Update to the UI

    Ok(new_item)
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
) -> Result<entity::stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    let stock = match StockItemMutation::find_by_id(&app.conn, id).await {
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
pub async fn stock_item_sell(
    id: i64,
    mut quantity: i64,
    price: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<entity::stock_item::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let cache = cache.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let stock = match StockItemMutation::find_by_id(&app.conn, id).await {
        Ok(stock) => stock,
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    };

    if stock.is_none() {
        return Err(AppError::new(
            "StockItemSell",
            eyre!(format!("Stock Item not found: {}", id)),
        ));
    }
    let mut new_item = stock.unwrap();

    // Get WFM Order
    let wfm_orders = wfm.orders().get_my_orders().await?;
    let wfm_order = wfm_orders.find_order_by_url_sub_type(
        &new_item.wfm_url,
        OrderType::Sell,
        new_item.sub_type.as_ref(),
    );

    let item_info = cache
        .tradable_items()
        .find_by_url_name(&new_item.wfm_url)
        .ok_or_else(|| {
            AppError::new(
                "StockItemSell",
                eyre!(format!("Item not found: {}", new_item.wfm_url)),
            )
        })?;

    // Add Transaction to the database
    let transaction = entity::transaction::Model::new(
        new_item.wfm_id.clone(),
        new_item.wfm_url.clone(),
        new_item.item_name.clone(),
        TransactionItemType::Item,
        new_item.item_unique_name.clone(),
        new_item.sub_type.clone(),
        item_info.tags,
        entity::transaction::TransactionType::Sale,
        quantity,
        "".to_string(),
        price,
        None,
    );

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

    if quantity < 0 {
        quantity = 1;
    }

    new_item.owned -= quantity;

    // If the quantity is 0, delete the stock item
    if new_item.owned <= 0 {
        // Delete the stock item
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
            Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
        }

        if wfm_order.is_some() {
            wfm.orders().delete(&wfm_order.unwrap().id).await?;
        }
        return Ok(new_item);
    }

    // Update the stock item
    match StockItemMutation::update_by_id(&app.conn, new_item.id, new_item.clone()).await {
        Ok(updated) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockItems,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
    }

    // Update the WFM Order
    if wfm_order.is_some() {
        let wfm_order = wfm_order.unwrap();
        wfm.orders()
            .update(&wfm_order.id, quantity, price, wfm_order.visible)
            .await?;
    }

    Ok(new_item)
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

    let stock_item = match StockItemMutation::find_by_id(&app.conn, id).await {
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
    wfm.orders().delete(&order.unwrap().id).await?;
    Ok(())
}