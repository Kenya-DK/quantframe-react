use std::{collections::HashMap, sync::Mutex};

use entity::{dto::*, wish_list::*};
use serde_json::{json, Value};
use service::{WishListMutation, WishListQuery};
use utils::{get_location, group_by, info, Error};
use wf_market::enums::OrderType;

use crate::{
    app::client::AppState,
    cache::client::CacheState,
    enums::{FindBy, FindByType},
    handlers::{
        handle_wfm_item, handle_wish_list, handle_wish_list_by_entity, stock_item::handle_item,
    },
    utils::{CreateWishListItemExt, ErrorFromExt, SubTypeExt},
    DATABASE,
};

#[tauri::command]
pub async fn get_wish_list_pagination(
    query: WishListPaginationQueryDto,
) -> Result<PaginatedResult<Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match WishListQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_db(
                "WishListQuery::get_wish_lists",
                "Failed to get wish list items: {}",
                e,
                get_location!(),
            );
            return Err(error);
        }
    };
}

#[tauri::command]
pub async fn get_wish_list_financial_report(
    query: WishListPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_wish_list_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn get_wish_list_status_counts(
    query: WishListPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_wish_list_pagination(query).await?;
    Ok(group_by(&items.results, |item| item.status.to_string())
        .iter()
        .map(|(status, items)| (status.clone(), items.len()))
        .collect::<HashMap<_, _>>())
}

#[tauri::command]
pub async fn wish_list_create(input: CreateWishListItem) -> Result<Model, Error> {
    match handle_wish_list_by_entity(input, "", OrderType::Sell).await {
        Ok((_, item)) => return Ok(item),
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log(Some("wish_list_buy.log")));
        }
    }
}

#[tauri::command]
pub async fn wish_list_bought(
    wfm_url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
) -> Result<Model, Error> {
    match handle_wish_list(wfm_url, sub_type, quantity, price, "", OrderType::Buy).await {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e
                .with_location(get_location!())
                .log(Some("wish_list_buy.log")));
        }
    }
}

#[tauri::command]
pub async fn wish_list_delete(id: i64) -> Result<(), Error> {
    let conn = DATABASE.get().unwrap();

    let item = WishListQuery::get_by_id(conn, id).await.map_err(|e| {
        Error::from_db(
            "Command::WishListDelete",
            "Failed to get wish list item by ID: {}",
            e,
            get_location!(),
        )
    })?;
    if item.is_none() {
        return Err(Error::new(
            "Command::WishListDelete",
            format!("Wish list item with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    handle_wfm_item(item.wfm_id, &item.sub_type, 1, OrderType::Buy, true)
        .await
        .map_err(|e| {
            e.with_location(get_location!())
                .log(Some("wish_list_delete.log"))
        })?;

    match WishListMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::from_db(
                "Command::WishListDelete",
                "Failed to delete wish list item by ID: {}",
                e,
                get_location!(),
            ));
        }
    }

    Ok(())
}
#[tauri::command]
pub async fn wish_list_update(input: UpdateWishList) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    match WishListMutation::update_by_id(conn, input).await {
        Ok(item) => Ok(item),
        Err(e) => {
            return Err(Error::from_db(
                "Command::WishListUpdate",
                "Failed to update wish list item by ID: {}",
                e,
                get_location!(),
            ))
        }
    }
}

#[tauri::command]
pub async fn wish_list_get_by_id(
    id: i64,
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let item = match WishListQuery::find_by_id(conn, id).await {
        Ok(wish_list_item) => {
            if let Some(item) = wish_list_item {
                item
            } else {
                return Err(Error::new(
                    "Command::WishListGetById",
                    "Wish list item not found",
                    get_location!(),
                ));
            }
        }
        Err(e) => {
            return Err(Error::from_db(
                "Command::WishListGetById",
                "Failed to get wish list item by ID: {}",
                e,
                get_location!(),
            ))
        }
    };

    let order = app.wfm_client.order().cache_orders().find_order(
        &item.wfm_id,
        &SubTypeExt::from_entity(item.sub_type.clone()),
        OrderType::Buy,
    );

    let mut payload = json!({});
    payload["item_info"] = json!(cache
        .tradable_item()
        .get_by(FindBy::new(FindByType::Url, &item.wfm_url))?);
    payload["stock"] = json!(item);
    payload["order_info"] = json!(order);

    Ok(payload)
}
