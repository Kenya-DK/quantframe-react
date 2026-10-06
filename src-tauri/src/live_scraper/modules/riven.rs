use std::sync::{atomic::Ordering, Arc, Weak};

use entity::{dto::PriceHistory, enums::*, stock_riven::*};
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};
use utils::{
    average_filtered_lowest_prices, get_location, info, warning, Error, LoggerOptions, OperationSet,
};
use wf_market::{
    client::Authenticated as WFAuthenticated,
    enums::{AuctionType, Polarity, StatusType},
    types::{
        AuctionFilter, AuctionList, AuctionWithOwner, CreateAuctionItem, CreateAuctionParams,
        ItemAttribute, UpdateAuctionParams,
    },
    Client as WFClient,
};

use crate::{
    cache::types::RivenPriceInput,
    live_scraper::{is_disabled, LiveScraperState},
    send_event,
    types::*,
    utils::{modules::states, ErrorFromExt},
    DATABASE,
};

static COMPONENT: &str = "LiveScraper:RivenModule";

#[derive(Debug)]
pub struct RivenModule {
    client: Weak<LiveScraperState>,
}

impl RivenModule {
    pub fn new(client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    fn send_event(&self, i18n_key: &str, values: Option<serde_json::Value>) {
        send_event!(
            UIEvent::SendLiveScraperMessage,
            json!({"i18nKey": format!("riven.{}", i18n_key), "values": values})
        );
    }

    async fn interesting_items() -> Result<Vec<Model>, Error> {
        let conn = DATABASE.get().unwrap();
        let stocks = StockRivenQuery::get_all(conn, StockRivenPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        Ok(stocks.results)
    }

    pub async fn check(&self) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let cache = states::cache_client()?;
        let app = states::app_state()?;
        let wfm_client = &app.wfm_client;
        let log_options = &LoggerOptions::default()
            .set_file("progress_riven.log")
            .set_show_component(false)
            .set_show_time(false);

        info(
            format!("{}:Check", COMPONENT),
            "Checking Riven items...",
            &LoggerOptions::default(),
        );

        let client = self.client.upgrade().expect("Client should not be dropped");
        let interesting_items = Self::interesting_items().await?;
        let total = interesting_items.len();

        for (index, mut stock_riven) in interesting_items.into_iter().enumerate() {
            let stock_riven = &mut stock_riven;

            // Stop if the scraper stopped running or the user is banned.
            if !client.is_running.load(Ordering::SeqCst) || app.user.is_banned() {
                warning(
                    format!("{}:ProcessRiven", COMPONENT),
                    "Live Scraper is not running or user is banned, stopping processing.",
                    &LoggerOptions::default(),
                );
                return Ok(());
            }

            info(
                COMPONENT,
                &format!(
                    "Starting process for riven mod: {}",
                    stock_riven.weapon_name
                ),
                &log_options
                    .set_centered(true)
                    .set_width(180)
                    .set_enable(true),
            );
            self.send_event(
                "checking",
                Some(json!({
                    "current": index + 1,
                    "total": total,
                    "name": stock_riven.weapon_name,
                    "mod_name": stock_riven.mod_name,
                    "sub_type": stock_riven.sub_type,
                })),
            );

            let settings = states::get_settings()?.live_scraper.rivens;
            let weapon_info = cache
                .weapon()
                .get_by(&stock_riven.wfm_weapon_url)
                .map_err(|e| e.with_location(get_location!()))?;

            let (mut operations, auction_id) = wfm_client
                .auction()
                .cache_auctions()
                .get_by_uuid(&stock_riven.uuid)
                .map(|auction| (OperationSet::from(vec!["Update"]), auction.id))
                .unwrap_or_else(|| (OperationSet::from(vec!["Create"]), String::new()));

            // Hidden rivens are taken down; inactive ones are simply skipped.
            if stock_riven.is_hidden {
                if stock_riven.status == StockStatus::InActive {
                    info(
                        format!("{}Skip", COMPONENT),
                        &format!(
                            "Riven {} is marked as hidden and inactive. Skipping.",
                            stock_riven.weapon_name
                        ),
                        &log_options,
                    );
                    continue;
                }
                stock_riven.set_status(StockStatus::InActive);
                stock_riven.set_list_price(None);
                stock_riven.locked = true;
                operations.add("Delete");
            }

            let mut live_auctions = if stock_riven.is_hidden {
                AuctionList::<AuctionWithOwner>::new(vec![])
            } else {
                match wfm_client
                    .auction()
                    .search_auctions(get_filter(stock_riven))
                    .await
                {
                    Ok(auctions) => auctions,
                    Err(wf_market::errors::ApiError::TooManyRequests(err)) => {
                        warning(
                            format!("{}:Check", COMPONENT),
                            &format!(
                                "Rate limited when getting live auctions for item {}. Skipping this item for now.",
                                stock_riven.wfm_weapon_url
                            ),
                            &log_options,
                        );
                        self.send_event("rate_limited", Some(json!({"seconds": err.retry_after})));
                        continue;
                    }
                    Err(e) => {
                        return Err(Error::from_wfm(
                            format!("{}:Check", COMPONENT),
                            &format!(
                                "Failed to get live auctions for item {}",
                                stock_riven.wfm_weapon_url
                            ),
                            e,
                            get_location!(),
                        ))
                    }
                }
            };
            live_auctions.filter_username(&app.user.wfm_username, true);
            live_auctions.sort_by_platinum();

            // Base price: the model estimate when enabled, otherwise the market average.
            let predicted = if settings.wts.use_predict {
                cache
                    .riven_pricer()
                    .predict(&price_input(stock_riven))
                    .map(|estimate| estimate.price.round() as i64)
            } else {
                None
            };

            // Track whether the listed price is driven by the prediction model.
            let is_predicted = predicted.is_some();
            if stock_riven.properties.get_property_value("is_predicted", false) != is_predicted {
                stock_riven
                    .properties
                    .set_property_value("is_predicted", is_predicted);
                stock_riven.is_dirty = true;
            }

            let mut post_price = predicted.unwrap_or_else(|| {
                average_filtered_lowest_prices(
                    live_auctions.prices(),
                    settings.wts.max_results,
                    settings.wts.threshold_percentage,
                )
            });

            // No live auctions and no prediction: fall back to the minimum profitable price.
            if live_auctions.total_auctions() == 0 && predicted.is_none() {
                post_price = stock_riven.bought + settings.wts.min_profit + 1;
                stock_riven.set_status(StockStatus::NoSellers);
                stock_riven.set_list_price(Some(post_price));
                stock_riven.locked = true;
            }

            // Respect the user configured minimum price.
            if let Some(minimum_price) = stock_riven
                .properties
                .get_property_value("min_price", None::<i64>)
            {
                let capped_price = post_price.max(minimum_price);
                if capped_price != post_price {
                    post_price = capped_price;
                    operations.add("MinimumPrice");
                }
            }

            let mut profit = post_price - stock_riven.bought;

            // Bump the price up to the minimum profit when needed.
            if !is_disabled(settings.wts.min_profit) && profit < settings.wts.min_profit {
                post_price += settings.wts.min_profit - profit;
                stock_riven.set_status(StockStatus::ToLowProfit);
                stock_riven.set_list_price(Some(post_price));
                stock_riven.locked = true;
                operations.add("LowProfit");
                profit = post_price - stock_riven.bought;
            }

            let highest_price = live_auctions.highest_price();
            let lowest_price = live_auctions.lowest_price();
            info(
                format!("{}Summary", COMPONENT),
                format!(
                    "Auction {}: PostPrice: {} | Profit: {} | IsStockDirty: {} | StockStatus: {:?} | StockListPrice: {:?} | Operations: {:?} | HighestPrice: {:?} | LowestPrice: {:?} | Predicted: {}",
                    stock_riven.weapon_name,
                    post_price,
                    profit,
                    stock_riven.is_dirty,
                    stock_riven.status,
                    stock_riven.list_price,
                    operations,
                    highest_price,
                    lowest_price,
                    predicted.is_some(),
                ),
                &log_options,
            );

            let properties = json!({
                "name": weapon_info.name,
                "mod_name": stock_riven.mod_name,
                "highest_price": highest_price,
                "lowest_price": lowest_price,
                "profit": profit,
                "operations": json!(operations.operations)
            });

            apply_operations(
                wfm_client,
                stock_riven,
                &operations,
                &auction_id,
                post_price,
                properties,
                log_options,
            )
            .await?;

            stock_riven.set_list_price(Some(post_price));
            stock_riven.set_status(StockStatus::Live);
            if stock_riven.status == StockStatus::Live {
                stock_riven.add_price_history(PriceHistory::new(
                    chrono::Local::now().naive_local().to_string(),
                    post_price,
                ));
            }
            if stock_riven.is_dirty {
                match StockRivenMutation::update_by_id(conn, stock_riven.to_update()).await {
                    Ok(_) => {
                        info(
                            format!("{}StockRivenUpdate", COMPONENT),
                            &format!("Updated stock item: {:?}", stock_riven.id),
                            &log_options,
                        );
                        send_event!(
                            UIEvent::RefreshStockRivens,
                            json!({"id": stock_riven.id, "source": COMPONENT})
                        );
                    }
                    Err(e) => return Err(e.with_location(get_location!())),
                }
            }
        }
        Ok(())
    }
}

/// Creates, updates or deletes the WFM auction for a stock riven.
async fn apply_operations(
    wfm_client: &WFClient<WFAuthenticated>,
    stock_riven: &Model,
    operations: &OperationSet,
    auction_id: &str,
    post_price: i64,
    properties: serde_json::Value,
    log_options: &LoggerOptions,
) -> Result<(), Error> {
    let can_create = wfm_client.auction().can_create_auction();

    if operations.has("Create") && !operations.has("Delete") && can_create {
        let item = CreateAuctionItem::new_riven(
            &stock_riven.wfm_weapon_url,
            &stock_riven.mod_name,
            stock_riven
                .attributes
                .0
                .iter()
                .map(|attr| {
                    ItemAttribute::new_with_properties(
                        &attr.wfm_url,
                        attr.positive,
                        attr.value,
                        json!({"formattedValue": attr.formatted_value}),
                    )
                })
                .collect(),
            stock_riven.re_rolls as i32,
            stock_riven.mastery_rank as i32,
            stock_riven.sub_type.clone().unwrap().rank.unwrap_or(0) as i32,
            Polarity::from_str(&stock_riven.polarity).unwrap_or_default(),
        );
        let params = CreateAuctionParams::new(
            post_price as i32,
            Some(post_price as i32),
            0,
            true,
            &stock_riven.comment,
            item,
        )
        .with_properties(json!(properties));

        match wfm_client.auction().create(params).await {
            Ok(auction) => {
                info(
                    format!("{}CreateSuccess", COMPONENT),
                    &format!(
                        "Created auction for weapon {}: {}",
                        auction.item.weapon_url_name, auction.id
                    ),
                    log_options,
                );
                send_event!(UIEvent::RefreshWfmAuctions, json!({"source": COMPONENT}));
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}CreateFail", COMPONENT),
                    format!(
                        "Failed to create auction for weapon {}",
                        stock_riven.weapon_name
                    ),
                    e,
                    get_location!(),
                ))
            }
        }
    } else if operations.has("Update") && !operations.has("Delete") {
        let params = UpdateAuctionParams::new()
            .with_buyout_price(Some(post_price as u32))
            .with_starting_price(post_price as u32)
            .with_properties(properties);
        match wfm_client.auction().update(auction_id, params).await {
            Ok(auction) => {
                info(
                    format!("{}UpdateSuccess", COMPONENT),
                    &format!(
                        "Updated auction for weapon {}: {}",
                        auction.item.weapon_url_name, auction.id
                    ),
                    log_options,
                );
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}UpdateFail", COMPONENT),
                    format!(
                        "Failed to update auction for weapon {}",
                        stock_riven.weapon_name
                    ),
                    e,
                    get_location!(),
                ))
            }
        }
    } else if operations.has("Update") && operations.has("Delete") {
        match wfm_client.auction().delete(auction_id).await {
            Ok(_) => {
                send_event!(UIEvent::RefreshWfmAuctions, json!({"source": COMPONENT}));
                info(
                    format!("{}DeleteSuccess", COMPONENT),
                    &format!(
                        "Deleted auction for weapon {}: {}",
                        stock_riven.weapon_name, auction_id
                    ),
                    log_options,
                );
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}DeleteFail", COMPONENT),
                    format!(
                        "Failed to delete auction for weapon {}",
                        stock_riven.weapon_name
                    ),
                    e,
                    get_location!(),
                ))
            }
        }
    } else if operations.has("Delete") {
        info(
            format!("{}Skip", COMPONENT),
            &format!(
                "Auction {} is marked as hidden or inactive. Skipping.",
                stock_riven.weapon_name
            ),
            log_options,
        );
    } else if !can_create {
        warning(
            format!("{}Skip", COMPONENT),
            &format!(
                "Auction {} has reached the auction limit. Skipping.",
                stock_riven.weapon_name
            ),
            log_options,
        );
    } else {
        warning(
            format!("{}Skip", COMPONENT),
            &format!(
                "Auction {} is not optimal for buying. Skipping.",
                stock_riven.weapon_name
            ),
            log_options,
        );
    }
    Ok(())
}

/// Builds the riven price model input from a stock riven.
fn price_input(stock_riven: &Model) -> RivenPriceInput {
    RivenPriceInput {
        weapon: stock_riven.wfm_weapon_url.clone(),
        re_rolls: stock_riven.re_rolls as u32,
        positives: stock_riven
            .attributes
            .0
            .iter()
            .filter(|attr| attr.positive)
            .map(|attr| attr.wfm_url.clone())
            .collect(),
        negative: stock_riven
            .attributes
            .0
            .iter()
            .find(|attr| !attr.positive)
            .map(|attr| attr.wfm_url.clone()),
    }
}

fn get_filter(entity: &Model) -> AuctionFilter {
    let mut auction_filter = AuctionFilter::new(AuctionType::Riven, &entity.wfm_weapon_url)
        .with_buyout_policy("direct")
        .with_user_activity(StatusType::InGame)
        .with_sort_by("price_asc");
    if entity.filter.enabled.unwrap_or(false) {
        if let Some(attributes) = &entity.filter.attributes {
            let positive_stats = attributes
                .iter()
                .filter(|a| a.positive && a.is_required)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();
            let negative_stats = attributes
                .iter()
                .filter(|a| !a.positive && a.is_required)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();
            if !positive_stats.is_empty() {
                auction_filter = auction_filter.with_positive_stats(positive_stats);
            }
            if !negative_stats.is_empty() {
                auction_filter = auction_filter.with_negative_stats(negative_stats);
            }
        }
        if let Some(mastery_rank) = &entity.filter.mastery_rank {
            if mastery_rank.min != 0 {
                auction_filter = auction_filter.with_mastery_rank_min(mastery_rank.min as u32);
            }
            if let Some(max) = mastery_rank.max {
                auction_filter = auction_filter.with_mastery_rank_max(max as u32);
            }
        }
        if let Some(re_rolls) = &entity.filter.re_rolls {
            if re_rolls.min != 0 {
                auction_filter = auction_filter.with_re_rolls_min(re_rolls.min as u32);
            }
            if let Some(max) = re_rolls.max {
                auction_filter = auction_filter.with_re_rolls_max(max as u32);
            }
        }
        if let Some(polarity) = &entity.filter.polarity {
            auction_filter = auction_filter
                .with_polarity(wf_market::enums::Polarity::from_str(polarity).unwrap());
        }
        if let Some(similarity) = entity.filter.similarity {
            let attributes = entity
                .attributes
                .0
                .iter()
                .map(|a| ItemAttribute::new(&a.wfm_url, a.positive, a.value))
                .collect::<Vec<_>>();
            auction_filter = auction_filter.with_similarity(similarity as i64, attributes);
        }
    }
    auction_filter
}
