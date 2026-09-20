use std::sync::{Arc, Mutex};

use entity::{
    dto::*,
    enums::*,
    stock_riven::{self, StockRivenPaginationQueryDto},
};
use qf_api::enums::ApplicationEvent;
use service::StockRivenQuery;
use utils::{filters_by, get_location, sorting::SortDirection, Error, OperationSet, Properties};
use wf_market::{enums::OrderType, types::Auction};

use crate::{
    app::AppState,
    cache::client::CacheState,
    handlers::handle_riven_by_entity,
    helper::{self, paginate},
    live_scraper::LiveScraperState,
    send_event, track_event,
    types::*,
    utils::*,
    DATABASE,
};

// --------------------------------------------------
// Auction
// --------------------------------------------------

#[tauri::command]
pub async fn auction_refresh(
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    let cache = cache.lock()?.clone();

    let auction = app.wfm_client.auction();
    let old_count = auction.cache_auctions().total_auctions();

    auction.my_auctions().await.map_err(|e| {
        track_event!(
            ApplicationEvent::WFMAuctionRefresh,
            [
                ("success", "false".to_string()),
                ("error_type", e.error_type().to_string()),
            ]
        );

        let err = Error::from_wfm(
            "AuctionRefresh",
            "Failed to refresh auctions",
            e,
            get_location!(),
        );

        err.log("auction_refresh.log");
        err
    })?;

    auction.cache_auctions_mut().apply_item_info(&cache)?;

    let new_count = auction.cache_auctions().total_auctions();

    track_event!(
        ApplicationEvent::WFMAuctionRefresh,
        [
            ("success", "true".to_string()),
            ("old_count", old_count.to_string()),
            ("new_count", new_count.to_string()),
        ]
    );

    Ok(())
}

// --------------------------------------------------
// Pagination
// --------------------------------------------------

#[tauri::command]
pub async fn get_wfm_auctions_pagination(
    query: WfmAuctionPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<PaginatedResult<Auction>, Error> {
    let conn = DATABASE.get().expect("Database not initialized");
    let app = app.lock()?.clone();

    let ids = StockRivenQuery::get_all(conn, StockRivenPaginationQueryDto::new(1, -1))
        .await
        .map_err(|e| {
            let err = Error::new(
                "GetWfmAuctionsPagination",
                format!("Failed to get rivens from database: {e}"),
                get_location!(),
            );

            err.log("get_wfm_auctions_pagination.log");
            err
        })?
        .results
        .into_iter()
        .map(|r| r.uuid)
        .collect::<Vec<_>>();

    let mut auctions = filters_by(
        &app.wfm_client.auction().cache_auctions().to_vec(),
        |auction| {
            if let FieldChange::Value(query) = &query.query {
                let query = query.to_lowercase();

                let name = auction
                    .properties
                    .get_property_value("name", String::new())
                    .to_lowercase();

                let mod_name = auction
                    .properties
                    .get_property_value("mod_name", String::new())
                    .to_lowercase();

                if !name.contains(&query) && !mod_name.contains(&query) {
                    return false;
                }
            }

            if let FieldChange::Value(auction_type) = &query.auction_type {
                if auction.item.item_type != *auction_type {
                    return false;
                }
            }

            true
        },
    );

    if let FieldChange::Value(sort_by) = &query.sort_by {
        let direction = match &query.sort_direction {
            FieldChange::Value(direction) => direction,
            _ => &SortDirection::Asc,
        };

        match sort_by.as_str() {
            "created_at" => auctions.sort_by(|a, b| match direction {
                SortDirection::Asc => a.created.cmp(&b.created),
                SortDirection::Desc => b.created.cmp(&a.created),
            }),

            "updated_at" => auctions.sort_by(|a, b| match direction {
                SortDirection::Asc => a.updated.cmp(&b.updated),
                SortDirection::Desc => b.updated.cmp(&a.updated),
            }),

            "platinum" => auctions.sort_by(|a, b| match direction {
                SortDirection::Asc => a.starting_price.cmp(&b.starting_price),
                SortDirection::Desc => b.starting_price.cmp(&a.starting_price),
            }),

            _ => {}
        }
    }

    let mut result = paginate(&auctions, query.pagination.page, query.pagination.limit);

    for auction in &mut result.results {
        if auction.is_direct_sell && !ids.contains(&auction.uuid) {
            auction.properties.set_property_value("can_import", true);
        }
    }

    Ok(result)
}

// --------------------------------------------------
// Overview
// --------------------------------------------------

#[tauri::command]
pub async fn get_wfm_auctions_overview(
    query: WfmAuctionPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(i64, i32, i64), Error> {
    let items = get_wfm_auctions_pagination(query, app).await?.results;

    let total = items.len() as i64;

    let revenue = items
        .iter()
        .filter(|auction| auction.is_direct_sell)
        .map(|auction| auction.starting_price)
        .sum();

    let profit = items
        .iter()
        .filter(|auction| auction.is_direct_sell)
        .map(|auction| auction.properties.get_property_value("potential_profit", 0) as i64)
        .sum();

    Ok((total, revenue, profit))
}

// --------------------------------------------------
// Get auction
// --------------------------------------------------

#[tauri::command]
pub async fn get_wfm_auction_by_id(
    id: String,
    operations: Option<Vec<String>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Auction, Error> {
    let cache = cache.lock()?.clone();
    let app = app.lock()?.clone();

    let mut auction = app
        .wfm_client
        .auction()
        .cache_auctions()
        .get_by_id(&id)
        .ok_or_else(|| {
            Error::new(
                "Command::GetWfmAuctionById",
                "Auction not found",
                get_location!(),
            )
        })?;

    let mut properties = Properties::default();

    let operations = operations.unwrap_or_else(|| {
        [
            "MarketInfo",
            "TransactionInfo",
            "GradeInfo",
            "EvaluateRolls",
            "VariantInfo",
            "EndoInfo",
            "KuvaInfo",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    });

    helper::populate_riven_market_properties(
        &mut properties,
        &auction.item.weapon_url_name,
        auction.item.mastery_level.unwrap_or(8) as i64,
        auction.item.re_rolls.unwrap_or(0) as i64,
        auction.item.mod_rank.unwrap_or(0) as i32,
        auction.item.as_raw_attributes(),
        auction.uuid.clone(),
        0,
        Some(auction.starting_price as i64),
        OperationSet::from(operations),
        &cache,
        &app.wfm_client,
    )
    .await?;

    auction.properties.set_properties(properties.properties);

    Ok(auction)
}

// --------------------------------------------------
// Delete all
// --------------------------------------------------

#[tauri::command]
pub async fn auction_delete_all(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();

    live_scraper.stop();

    let auctions = app.wfm_client.auction().my_auctions().await.map_err(|e| {
        track_event!(
            ApplicationEvent::WFMAuctionDelete,
            [
                ("success", "false".to_string()),
                ("error_type", e.error_type().to_string()),
            ]
        );

        let err = Error::from_wfm(
            "AuctionDeleteAll",
            "Failed to get auctions",
            e,
            get_location!(),
        );

        err.log("auction_delete_all.log");
        err
    })?;

    let total = auctions.total_auctions();
    let mut current = total;

    for auction in auctions.to_vec() {
        if let Err(e) = app.wfm_client.auction().delete(&auction.id).await {
            track_event!(
                ApplicationEvent::WFMAuctionDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

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
            json!({
                "source": "auction_delete_all",
                "current": current,
                "total": total,
            })
        );
    }

    track_event!(
        ApplicationEvent::WFMAuctionDelete,
        [
            ("success", "true".to_string()),
            ("count", total.to_string()),
        ]
    );

    Ok(())
}

// --------------------------------------------------
// Delete by ID
// --------------------------------------------------

#[tauri::command]
pub async fn auction_delete_by_id(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();

    let auction = app
        .wfm_client
        .auction()
        .cache_auctions()
        .get_by_id(&id)
        .ok_or_else(|| {
            track_event!(
                ApplicationEvent::WFMAuctionDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", "auction_not_found".to_string()),
                ]
            );

            Error::new(
                "Command::AuctionDeleteById",
                "Auction not found",
                get_location!(),
            )
        })?;

    app.wfm_client
        .auction()
        .delete(&auction.id)
        .await
        .map_err(|e| {
            track_event!(
                ApplicationEvent::WFMAuctionDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

            let err = Error::from_wfm(
                "Command::AuctionDeleteById",
                "Failed to delete auction",
                e,
                get_location!(),
            );

            err.log("auction_delete_by_id.log");
            err
        })?;

    track_event!(
        ApplicationEvent::WFMAuctionDelete,
        [("success", "true".to_string())]
    );

    Ok(())
}

// --------------------------------------------------
// Import by ID
// --------------------------------------------------

#[tauri::command]
pub async fn auction_import_by_id(
    id: String,
    bought: i64,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<stock_riven::Model, Error> {
    let app = app.lock()?.clone();

    let result = async {
        let auction = app
            .wfm_client
            .auction()
            .cache_auctions()
            .get_by_id(&id)
            .ok_or_else(|| {
                Error::new(
                    "Command::AuctionImportById",
                    "Auction not found",
                    get_location!(),
                )
            })?;

        let (_, model) = handle_riven_by_entity(
            auction.to_create()?.set_bought(bought),
            "",
            OrderType::Buy,
            &OperationSet::new(),
        )
        .await?;

        Ok::<_, Error>(model)
    }
    .await;

    match &result {
        Ok(_) => track_event!(
            ApplicationEvent::WFMAuctionImport,
            [("success", "true".to_string())]
        ),
        Err(_err) => track_event!(
            ApplicationEvent::WFMAuctionImport,
            [
                ("success", "false".to_string()),
                ("error_type", "import_failed".to_string()),
            ]
        ),
    }

    result
}
