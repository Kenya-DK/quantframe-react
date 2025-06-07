use std::sync::{Arc, Mutex};

use create::CreateWishListItem;
use entity::sub_type::SubType;
use entity::wish_list::*;

use eyre::eyre;
use serde_json::json;
use service::{WishListMutation, WishListQuery};

use crate::cache::client::CacheClient;
use crate::helper::{self, add_metric};
use crate::utils::modules::error;
use crate::wfm_client::enums::order_type::OrderType;
use crate::DATABASE;
use crate::{
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::AppError,
    },
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn get_wish_lists(
    query: entity::wish_list::dto::pagination_wish_list::WishListPaginationQueryDto,
) -> Result<entity::dto::pagination::PaginatedDto<wish_list::Model>, AppError> {
    let conn = DATABASE.get().unwrap();
    match WishListQuery::get_all_v2(conn, query).await {
        Ok(items) => {
            helper::add_metric("WishList_ItemGetAll", "manual");
            return Ok(items);
        }
        Err(e) => {
            let error: AppError = AppError::new_db("WishListQuery::reload", e);
            error::create_log_file("command_stock_item_reload.log", &error);
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn wish_list_create(
    wfm_url: String,
    maximum_price: Option<i64>,
    sub_type: Option<SubType>,
    quantity: i64,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
) -> Result<wish_list::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let cache = cache.lock()?.clone();

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

    match WishListMutation::add_item(conn, created_item.to_model()).await {
        Ok(_) => add_metric("WishList_ItemCreated", "manual"),
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
    is_hidden: Option<bool>,
) -> Result<wish_list::Model, AppError> {
    let conn = DATABASE.get().unwrap();

    let item = match WishListQuery::find_by_id(conn, id).await {
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
    if let Some(is_hidden) = is_hidden {
        new_item.is_hidden = is_hidden;
    }
    if let Some(quantity) = quantity {
        new_item.quantity = quantity;
    }
    new_item.updated_at = chrono::Utc::now();

    match WishListMutation::update_by_id(conn, new_item.id, new_item.clone()).await {
        Ok(_) => add_metric("WishList_ItemUpdated", "manual"),
        Err(e) => {
            let error: AppError = AppError::new_db("WishListMutation::UpdateById", e);
            error::create_log_file("wish_list_update.log", &error);
            return Err(error);
        }
    }

    Ok(new_item)
}

#[tauri::command]
pub async fn wish_list_delete(
    id: i64,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = notify.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let item = match WishListQuery::get_by_id(conn, id).await {
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

    match WishListMutation::delete_by_id(conn, id).await {
        Ok(_) => add_metric("WishList_ItemDeleted", "manual"),
        Err(e) => {
            let error: AppError = AppError::new_db("WishListMutation::DeleteById", e);
            error::create_log_file("wish_list_delete.log", &error);
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

#[tauri::command]
pub async fn wish_list_bought(id: i64, price: i64) -> Result<wish_list::Model, AppError> {
    let conn = DATABASE.get().unwrap();

    let item = match WishListQuery::get_by_id(conn, id).await {
        Ok(item) => item,
        Err(e) => {
            return Err(AppError::new("WishListQuery::get_by_id", eyre!(e)));
        }
    };

    if item.is_none() {
        return Err(AppError::new(
            "WishItemBought",
            eyre!(format!("Item with id {} not found", id)),
        ));
    }
    let item = item.unwrap();

    let mut created_stock = CreateWishListItem::new_valid(
        item.wfm_id.clone(),
        item.wfm_url.clone(),
        item.item_name.clone(),
        item.item_unique_name.clone(),
        vec![],
        item.sub_type.clone(),
        item.maximum_price,
        1,
        Some(price),
    );

    match helper::progress_wish_item(
        &mut created_stock,
        "--item_by url_name --item_lang en",
        "",
        OrderType::Buy,
        vec![],
        "manual",
    )
    .await
    {
        Ok((_, _)) => {
            return Ok(item);
        }
        Err(e) => {
            return Err(e);
        }
    }
}
