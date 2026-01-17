use std::sync::{Arc, Mutex};

use entity::{
    dto::*,
    enums::*,
    stock_riven::{self, StockRivenPaginationQueryDto},
};
use service::StockRivenQuery;
use utils::{filters_by, get_location, Error};
use wf_market::{enums::OrderType, types::Auction};

use crate::{
    add_metric, app::client::AppState, cache::client::CacheState, handlers::handle_riven_by_entity,
    helper::paginate, live_scraper::LiveScraperState, send_event, types::*, utils::*, DATABASE,
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
            err.log("auction_refresh.log");
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
pub async fn get_wfm_auctions_pagination(
    query: WfmAuctionPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<PaginatedResult<Auction>, Error> {
    let conn = DATABASE.get().expect("Database not initialized");
    let app = app.lock()?.clone();

    let ids = match StockRivenQuery::get_all(conn, StockRivenPaginationQueryDto::new(1, -1)).await {
        Ok(res) => res
            .results
            .iter()
            .map(|r| r.uuid.clone())
            .collect::<Vec<String>>(),
        Err(e) => {
            let err = Error::new(
                "GetWfmAuctionsPagination",
                format!("Failed to get rivens from database: {}", e),
                get_location!(),
            );
            err.log("get_wfm_auctions_pagination.log");
            return Err(err);
        }
    };

    let mut filtered_auctions =
        filters_by(&app.wfm_client.auction().cache_auctions().to_vec(), |o| {
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
            match &query.auction_type {
                FieldChange::Value(auction_type) => {
                    if &o.item.item_type != auction_type {
                        return false;
                    }
                }
                _ => {}
            }

            true
        });

    match &query.sort_by {
        FieldChange::Value(sort_by) => {
            let dir = match &query.sort_direction {
                FieldChange::Value(dir) => dir,
                _ => &SortDirection::Asc,
            };
            // Only allow sorting by known columns for safety
            match sort_by.as_str() {
                "created_at" => filtered_auctions.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.created.cmp(&b.created),
                    SortDirection::Desc => b.created.cmp(&a.created),
                }),
                "updated_at" => filtered_auctions.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.updated.cmp(&b.updated),
                    SortDirection::Desc => b.updated.cmp(&a.updated),
                }),
                "platinum" => filtered_auctions.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.starting_price.cmp(&b.starting_price),
                    SortDirection::Desc => b.starting_price.cmp(&a.starting_price),
                }),
                _ => {}
            }
        }
        _ => {}
    }

    let mut paginate = paginate(
        &filtered_auctions,
        query.pagination.page,
        query.pagination.limit,
    );
    for auction in paginate.results.iter_mut() {
        if auction.is_direct_sell && !ids.contains(&auction.uuid) {
            auction.update_details(auction.get_details().set_can_import(true));
        }
    }
    Ok(paginate)
}

#[tauri::command]
pub async fn get_wfm_auctions_overview(
    query: WfmAuctionPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(i64, i32, i64), Error> {
    let items = get_wfm_auctions_pagination(query, app).await?.results;

    let total = items.len() as i64;
    let revenue = items
        .iter()
        .filter(|a| a.is_direct_sell)
        .map(|a| a.starting_price)
        .sum::<i32>();
    let profit = items
        .iter()
        .filter(|a| a.is_direct_sell)
        .map(|a| {
            let details = a.get_details();
            details.profit
        })
        .sum::<i64>();
    Ok((total, revenue, profit))
}
#[tauri::command]
pub async fn auction_delete_all(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    live_scraper.stop();
    let auctions = match app.wfm_client.auction().my_auctions().await {
        Ok(orders) => orders,
        Err(e) => {
            let err = Error::from_wfm(
                "AuctionDeleteAll",
                "Failed to get auctions",
                e,
                get_location!(),
            );
            err.log("auction_delete_all.log");
            return Err(err);
        }
    };
    let total = auctions.total_auctions();
    let mut current = total;
    for auction in auctions.to_vec() {
        if let Err(e) = app.wfm_client.auction().delete(&auction.id).await {
            let err = Error::from_wfm(
                "AuctionDeleteAll",
                "Failed to delete auction",
                e,
                get_location!(),
            );
            err.log("auction_delete_all.log");
            return Err(err);
        }
        current -= 1;
        send_event!(
            UIEvent::OnDeleteWfmAuctions,
            json!({"source": "auction_delete_all", "current": current, "total": total})
        );
    }
    add_metric!("auction_delete_all", "manual");
    Ok(())
}
#[tauri::command]
pub async fn auction_delete_by_id(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    let auction = app.wfm_client.auction().cache_auctions().get_by_id(&id);
    if auction.is_none() {
        return Err(Error::new(
            "Command::AuctionDeleteById",
            "Auction not found",
            get_location!(),
        ));
    }
    let auction = auction.unwrap();
    match app.wfm_client.auction().delete(&auction.id).await {
        Ok(_) => {}
        Err(e) => {
            let err = Error::from_wfm(
                "Command::AuctionDeleteById",
                "Failed to delete auction",
                e,
                get_location!(),
            );
            err.log("auction_delete_by_id.log");
            return Err(err);
        }
    }
    add_metric!("auction_delete_by_id", "manual");
    Ok(())
}
#[tauri::command]
pub async fn auction_import_by_id(
    id: String,
    bought: i64,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<stock_riven::Model, Error> {
    let app = app.lock()?.clone();
    let auction = app.wfm_client.auction().cache_auctions().get_by_id(&id);
    if auction.is_none() {
        return Err(Error::new(
            "Command::AuctionImportById",
            "Auction not found",
            get_location!(),
        ));
    }
    let auction = auction.unwrap();
    let (_, model) = handle_riven_by_entity(
        auction.to_create()?.set_bought(bought),
        "",
        OrderType::Buy,
        &[],
    )
    .await?;
    add_metric!("auction_import_by_id", "manual");
    Ok(model)
}
