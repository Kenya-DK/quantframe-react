use std::sync::{Arc, Mutex};

use create::CreateWishListItem;
use entity::wish_list::*;
use entity::{enums::stock_status::StockStatus, sub_type::SubType};

use eyre::eyre;
use migration::Iden;
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery, WishListMutation, WishListQuery};

use crate::cache::client::CacheClient;
use crate::helper::{self, add_metric};
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
pub async fn wish_list_reload(
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    match WishListQuery::get_all(&app.conn).await {
        Ok(items) => {
            add_metric("WishList_Reload", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateWishList,
                UIOperationEvent::Set,
                Some(json!(items)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("WishListQuery::reload", e);
            error::create_log_file("command.log".to_string(), &error);
            return Err(error);
        }
    };
    Ok(())
}

#[tauri::command]
pub async fn wish_list_create(
    wfm_url: String,
    maximum_price: Option<i64>,
    sub_type: Option<SubType>,
    quantity: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<wish_list::Model, AppError> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();
    let notify = notify.lock()?.clone();

    let mut created_item =
        CreateWishListItem::new(wfm_url, sub_type.clone(), maximum_price, quantity);
    match cache
        .tradable_items()
        .validate_create_wish_item(&mut created_item, "--item_by url_name --item_lang en")
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    match WishListMutation::add_item(&app.conn, created_item.to_model()).await {
        Ok(inserted) => {
            notify.gui().send_event_update(
                UIEvent::UpdateWishList,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
            add_metric("WishList_ItemCreated", "manual");
        }
        Err(e) => {
            return Err(AppError::new("StockItemCreate", eyre!(e)));
        }
    }
    Ok(created_item.to_model())
}

#[tauri::command]
pub async fn wish_list_update(
    id: i64,
    maximum_price: Option<i64>,
    sub_type: Option<SubType>,
    quantity: Option<i64>,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<wish_list::Model, AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();

    let item = match WishListQuery::find_by_id(&app.conn, id).await {
        Ok(foundItem) => foundItem,
        Err(e) => return Err(AppError::new("WishListItemUpdate", eyre!(e))),
    };

    if item.is_none() {
        return Err(AppError::new(
            "WishListItemUpdate",
            eyre!(format!("Item with id {} not found", id)),
        ));
    }

    let mut new_item = item.unwrap();

    if let Some(maximum_price) = maximum_price {
        new_item.maximum_price = Some(maximum_price);
    }

    if let Some(sub_type) = sub_type {
        new_item.sub_type = Some(sub_type);
    }

    if let Some(quantity) = quantity {
        new_item.quantity = quantity;
    }
    new_item.updated_at = chrono::Utc::now();

    match WishListMutation::update_by_id(&app.conn, new_item.id, new_item.clone()).await {
        Ok(updated) => {
            helper::add_metric("WishList_ItemUpdated", "manual");
            notify.gui().send_event_update(
                UIEvent::UpdateWishList,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(updated)),
            );
        }
        Err(e) => {
            let error: AppError = AppError::new_db("WishListMutation::UpdateById", e);
            error::create_log_file("wish_list_update.log".to_string(), &error);
            return Err(error);
        }
    }

    Ok(new_item)
}

#[tauri::command]
pub async fn wish_list_delete(
    id: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let item = match WishListQuery::get_by_id(&app.conn, id).await {
        Ok(item) => item,
        Err(e) => {
            return Err(AppError::new("WishListQuery::get_by_id", eyre!(e)));
        }
    };

    if item.is_none() {
        return Err(AppError::new(
            "WishItemDelete",
            eyre!(format!("Item with id {} not found", id)),
        ));
    }
    let item = item.unwrap();

    match WishListMutation::delete_by_id(&app.conn, id).await {
        Ok(_) => {
            notify.gui().send_event_update(
                UIEvent::UpdateWishList,
                UIOperationEvent::Delete,
                Some(json!({ "id": id })),
            );
            add_metric("WishList_ItemDeleted", "manual");
        }
        Err(e) => {
            let error: AppError = AppError::new_db("WishListMutation::DeleteById", e);
            error::create_log_file("wish_list_delete.log".to_string(), &error);
            return Err(error);
        }
    }
    let my_orders = wfm.orders().get_my_orders().await?;
    let order = my_orders.find_order_by_url_sub_type(
        &item.wfm_url,
        OrderType::Sell,
        item.sub_type.as_ref(),
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
