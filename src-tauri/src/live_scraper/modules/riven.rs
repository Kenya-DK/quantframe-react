use std::sync::{atomic::Ordering, Arc, Weak};

use entity::{dto::PriceHistory, enums::*, stock_riven::*};
use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};
use utils::{
    average_filtered_lowest_prices, error, get_location, info, warning, Error, LoggerOptions,
};
use wf_market::{
    enums::{AuctionType, Polarity, StatusType},
    types::{
        AuctionFilter, AuctionList, AuctionWithOwner, CreateAuctionItem, CreateAuctionParams,
        ItemAttribute, UpdateAuctionParams,
    },
};
static COMPONENT: &str = "LiveScraper:RivenModule";
use crate::{
    cache::types::CacheRivenWeapon,
    enums::FindBy,
    live_scraper::{is_disabled, LiveScraperState},
    send_event,
    types::*,
    utils::{auction_ext::AuctionDetails, modules::states, AuctionExt, ErrorFromExt},
    DATABASE,
};

#[derive(Debug)]
pub struct RivenModule {
    client: Weak<LiveScraperState>,
}

impl RivenModule {
    /**
     * Creates a new `RivenModule` with an empty item list.
     * The `client` parameter is an `Arc<LiveScraperState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    fn send_event(&self, i18nKey: &str, values: Option<serde_json::Value>) {
        send_event!(
            UIEvent::SendLiveScraperMessage,
            json!({"i18nKey": format!("riven.{}", i18nKey), "values": values})
        );
    }
    async fn interesting_items() -> Result<Vec<Model>, Error> {
        let conn = DATABASE.get().unwrap();
        let stocks = StockRivenQuery::get_all(conn, StockRivenPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:InterestingItems", COMPONENT),
                    "Failed to get stock riven items",
                    e,
                    get_location!(),
                )
            })?;
        Ok(stocks.results)
    }

    pub async fn check(&self) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let cache = states::cache_client()?;
        let app = states::app_state()?;
        let wfm_client = app.wfm_client;
        info(
            format!("{}:Check", COMPONENT),
            "Checking Riven items...",
            &LoggerOptions::default(),
        );
        let client = self.client.upgrade().expect("Client should not be dropped");
        let interesting_items = Self::interesting_items().await?;
        let total = interesting_items.len();
        let mut current_index = interesting_items.len();
        let log_options = &LoggerOptions::default()
            .set_file("progress_riven.log")
            .set_show_component(false)
            .set_show_time(false);
        for stock_riven in interesting_items {
            let stock_riven = &mut stock_riven.clone();
            // Stop if client stopped running or user is banned
            if !client.is_running.load(Ordering::SeqCst) || app.user.is_banned() {
                warning(
                    format!("{}:ProcessRiven", COMPONENT),
                    "Live Scraper is not running or user is banned, stopping processing.",
                    &&LoggerOptions::default(),
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
            // GUI event for progress
            self.send_event(
                "checking",
                Some(json!({
                    "current": current_index,
                    "total": total,
                    "name": stock_riven.weapon_name,
                    "mod_name": stock_riven.mod_name,
                    "sub_type": stock_riven.sub_type,
                })),
            );

            // Settings
            let settings = states::get_settings()?.live_scraper.stock_riven;

            // Get tradable item info from cache
            let Some(item_info) = cache.riven().get_riven_by(FindBy::new(
                crate::enums::FindByType::Url,
                &stock_riven.wfm_weapon_url,
            ))?
            else {
                error(
                    format!("{}:Check", COMPONENT),
                    &format!(
                        "Item not found in riven items: {}",
                        stock_riven.wfm_weapon_url
                    ),
                    &&LoggerOptions::default(),
                );
                continue;
            };

            let mut auction_info = get_auction_details(&stock_riven.uuid, &item_info, &wfm_client);
            if stock_riven.is_hidden && stock_riven.status == StockStatus::InActive {
                info(
                    format!("{}Skip", COMPONENT),
                    &format!(
                        "Riven {} is marked as hidden and inactive. Skipping.",
                        stock_riven.weapon_name
                    ),
                    &log_options,
                );
                continue;
            } else if stock_riven.is_hidden && stock_riven.status != StockStatus::InActive {
                stock_riven.set_status(StockStatus::InActive);
                stock_riven.set_list_price(None);
                stock_riven.locked = true;
                auction_info.add_operation("Delete");
            }

            let mut live_auctions = if stock_riven.is_hidden {
                AuctionList::<AuctionWithOwner>::new(vec![])
            } else {
                match wfm_client
                    .auction()
                    .search_auctions(get_filter(&stock_riven))
                    .await
                {
                    Ok(auctions) => auctions,
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
            // Apply filters to auctions
            live_auctions.filter_username(&app.user.wfm_username, true);
            live_auctions.sort_by_platinum();

            // Get the price the item was bought for.
            let mut post_price = average_filtered_lowest_prices(
                live_auctions.prices(),
                settings.limit_to,
                settings.threshold_percentage,
            );

            // Check if there are no live auctions
            if live_auctions.total_auctions() == 0 {
                post_price = stock_riven.bought + settings.min_profit + 1;
                stock_riven.set_status(StockStatus::NoSellers);
                stock_riven.set_list_price(Some(post_price));
                stock_riven.locked = true;
            }

            if let Some(minimum_price) = stock_riven.minimum_price {
                let capped_price = post_price.max(minimum_price);
                if capped_price != post_price {
                    post_price = capped_price;
                    auction_info.add_operation("MinimumPrice");
                }
            }

            // Calculate the profit from the post price
            let mut profit = post_price - stock_riven.bought;

            // Handle Low Profit
            if !is_disabled(settings.min_profit) && profit < settings.min_profit {
                post_price += settings.min_profit - profit;
                stock_riven.set_status(StockStatus::ToLowProfit);
                stock_riven.set_list_price(Some(post_price));
                stock_riven.locked = true;
                auction_info.add_operation("LowProfit");
                profit = post_price - stock_riven.bought;
            }

            // Summary log
            info(
            format!("{}Summary", COMPONENT),
            format!(
                "Auction {}: PostPrice: {} | Profit: {} | IsStockDirty: {} | StockStatus: {:?} | StockListPrice: {:?} | {}",
                stock_riven.weapon_name, post_price, profit, stock_riven.is_dirty, stock_riven.status, stock_riven.list_price, auction_info
            ),
            &log_options,
            );

            auction_info = auction_info.set_highest_price(live_auctions.highest_price());
            auction_info = auction_info.set_lowest_price(live_auctions.lowest_price());
            auction_info = auction_info.set_auctions(live_auctions.take_top(5));

            let can_create = wfm_client.auction().can_create_auction();
            if auction_info.has_operation("Create")
                && !auction_info.has_operation("Delete")
                && can_create
            {
                match wfm_client
                    .auction()
                    .create(
                        CreateAuctionParams::new(
                            post_price as i32,
                            Some(post_price as i32),
                            0,
                            true,
                            &stock_riven.comment,
                            CreateAuctionItem::new_riven(
                                &stock_riven.wfm_weapon_url,
                                &stock_riven.mod_name,
                                stock_riven
                                    .attributes
                                    .0
                                    .iter()
                                    .map(|attr| {
                                        ItemAttribute::new(
                                            &attr.url_name,
                                            attr.positive,
                                            attr.value,
                                        )
                                    })
                                    .collect(),
                                stock_riven.re_rolls as i32,
                                stock_riven.mastery_rank as i32,
                                stock_riven.sub_type.clone().unwrap().rank.unwrap_or(0) as i32,
                                Polarity::from_str(&stock_riven.polarity).unwrap_or_default(),
                            ),
                        )
                        .with_properties(json!(auction_info)),
                    )
                    .await
                {
                    Ok(auction) => {
                        info(
                            format!("{}CreateSuccess", COMPONENT),
                            &format!(
                                "Created auction for weapon {}: {}",
                                auction.item.weapon_url_name, auction.id
                            ),
                            &log_options,
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
                        ));
                    }
                }
            } else if auction_info.has_operation("Update") && !auction_info.has_operation("Delete")
            {
                match wfm_client
                    .auction()
                    .update(
                        &auction_info.auction_id,
                        UpdateAuctionParams::new()
                            .with_buyout_price(Some(post_price as u32))
                            .with_starting_price(post_price as u32)
                            .with_properties(json!(auction_info)),
                    )
                    .await
                {
                    Ok(auction) => {
                        info(
                            format!("{}UpdateSuccess", COMPONENT),
                            &format!(
                                "Updated auction for weapon {}: {}",
                                auction.item.weapon_url_name, auction.id
                            ),
                            &log_options,
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
                        ));
                    }
                }
            } else if auction_info.has_operation("Update") && auction_info.has_operation("Delete") {
                match wfm_client.auction().delete(&auction_info.auction_id).await {
                    Ok(_) => {
                        send_event!(UIEvent::RefreshWfmAuctions, json!({"source": COMPONENT}));
                        info(
                            format!("{}DeleteSuccess", COMPONENT),
                            &format!(
                                "Deleted auction for weapon {}: {}",
                                stock_riven.weapon_name, auction_info.auction_id
                            ),
                            &log_options,
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
                        ));
                    }
                }
            } else if auction_info.has_operation("Delete") {
                info(
                    format!("{}Skip", COMPONENT),
                    &format!(
                        "Auction {} is marked as hidden or inactive. Skipping.",
                        stock_riven.weapon_name
                    ),
                    &log_options,
                );
            } else if !can_create {
                warning(
                    format!("{}Skip", COMPONENT),
                    &format!(
                        "Auction {} has reached the auction limit. Skipping.",
                        stock_riven.weapon_name
                    ),
                    &log_options,
                );
            } else {
                warning(
                    format!("{}Skip", COMPONENT),
                    &format!(
                        "Auction {} is not optimal for buying. Skipping.",
                        stock_riven.weapon_name
                    ),
                    &log_options,
                );
            }
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
                    Err(e) => {
                        return Err(Error::from_db(
                            format!("{}StockRivenUpdate", COMPONENT),
                            &format!("Failed to update stock item {:?}", stock_riven.id),
                            e,
                            get_location!(),
                        ));
                    }
                }
            }
            current_index -= 1;
        }
        Ok(())
    }
}

pub fn get_auction_details(
    uuid: impl Into<String>,
    item_info: &CacheRivenWeapon,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
) -> AuctionDetails {
    wfm_client
        .auction()
        .cache_auctions()
        .get_by_uuid(uuid)
        .map(|auction| {
            auction
                .get_details()
                .set_operation(&["Update"])
                .set_auction_id(auction.id.clone())
        })
        .unwrap_or_default()
        .set_info(item_info)
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
                .map(|a| ItemAttribute::new(&a.url_name, a.positive, a.value))
                .collect::<Vec<_>>();
            auction_filter = auction_filter.with_similarity(similarity as i64, attributes);
        }
    }
    auction_filter
}
