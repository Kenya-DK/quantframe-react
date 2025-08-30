use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use entity::{dto::*, enums::*};
use serde_json::{json, Value};
use service::StockItemQuery;
use utils::{filters_by, get_location, group_by, Error};
use wf_market::{
    enums::OrderType,
    types::{item, Auction, Order},
};

use crate::{
    app::client::AppState, cache::client::CacheState, enums::*, helper::paginate,
    live_scraper::LiveScraperState, notification::enums::UIEvent, send_event, utils::*, DATABASE,
};
#[tauri::command]
pub async fn auction_refresh(
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<(), Error> {
    let app_state = app.lock()?.clone();
    let cache_state = cache.lock()?.clone();
    app_state
        .wfm_client
        .auction()
        .my_auctions()
        .await
        .map_err(|e| {
            let err = Error::from_wfm(
                "AuctionRefresh",
                "Failed to refresh auctions",
                e,
                get_location!(),
            );
            err.log(Some("auction_refresh.log"));
            err
        })?;
    app_state
        .wfm_client
        .auction()
        .cache_auctions_mut()
        .apply_item_info(&cache_state)?;
    Ok(())
}

#[tauri::command]
pub fn get_wfm_auctions_pagination(
    query: WfmAuctionPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<PaginatedResult<Auction>, Error> {
    let app = app.lock()?.clone();

    let filtered_auctions = filters_by(&app.wfm_client.auction().cache_auctions().to_vec(), |o| {
        match &query.query {
            FieldChange::Value(q) => {
                let q = q.to_lowercase();
                let details = o.get_details();
                let item_name = details.item_name.to_lowercase();
                let mod_name = o
                    .item
                    .mod_name
                    .clone()
                    .unwrap_or("".to_string())
                    .to_lowercase();
                if !item_name.contains(&q) && !mod_name.contains(&q) {
                    return false;
                }
            }
            _ => {}
        }

        true
    });

    let p = paginate(
        &filtered_auctions,
        query.pagination.page,
        query.pagination.limit,
    );

    Ok(p)
}

#[tauri::command]
pub async fn auction_delete_all(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    live_scraper.stop();
    let orders = match app.wfm_client.order().my_orders().await {
        Ok(orders) => orders,
        Err(e) => {
            let err = Error::from_wfm(
                "AuctionDeleteAll",
                "Failed to get auctions",
                e,
                get_location!(),
            );
            err.log(Some("auction_delete_all.log"));
            return Err(err);
        }
    };
    let total = orders.total_orders();
    let mut current = total;
    for order in orders.to_vec() {
        if let Err(e) = app.wfm_client.order().delete(&order.id).await {
            let err = Error::from_wfm(
                "OrderDeleteAll",
                "Failed to delete order",
                e,
                get_location!(),
            );
            err.log(Some("auction_delete_all.log"));
            return Err(err);
        }
        current -= 1;
        send_event!(
            UIEvent::OnDeleteWfmOrders,
            json!({"source": "auction_delete_all", "current": current, "total": total})
        );
    }
    Ok(())
}
#[tauri::command]
pub async fn auction_delete_by_id(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    let order = app.wfm_client.order().cache_orders().get_by_id(&id);
    if order.is_none() {
        return Err(Error::new(
            "Command::AuctionDeleteById",
            "Auction not found",
            get_location!(),
        ));
    }
    let order = order.unwrap();
    match app.wfm_client.auction().delete(&order.id).await {
        Ok(_) => {}
        Err(e) => {
            let err = Error::from_wfm(
                "Command::AuctionDeleteById",
                "Failed to delete auction",
                e,
                get_location!(),
            );
            err.log(Some("auction_delete_by_id.log"));
            return Err(err);
        }
    }
    Ok(())
}
