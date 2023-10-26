use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    auth::AuthState, error::AppError, live_scraper::client::LiveScraperClient, logger,
    structs::AuctionItem,
};
pub struct RivenModule<'a> {
    pub client: &'a LiveScraperClient,
}

impl<'a> RivenModule<'a> {
    pub async fn check_stock(&self) -> Result<(), AppError> {
        let db = self.client.db.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        logger::info_con("RivenModule", "Run riven module");
        let stockrivens = db.stock_riven().get_rivens().await?;
        let my_auctions = wfm.auction().get_my_auctions().await?;
        let my_rivens = my_auctions
            .iter()
            .filter(|a| a.item.item_type == "riven".to_string())
            .collect::<Vec<_>>();
        for riven in stockrivens {
            // Check if client is running
            if self.client.is_running() == false {
                break;
            }

            // Find my auction for this riven if exists
            let auction = my_rivens
                .iter()
                .find(|a| a.id == riven.order_id.clone().unwrap_or("".to_string()));

            // Check if riven is private
            if riven.private {
                // Update Auction on warframe.market
                if auction.is_some() {
                    let auction = auction.unwrap();
                    if auction.private == false {
                        wfm.auction()
                            .update(
                                auction.id.as_str(),
                                auction.starting_price as i32,
                                auction.minimal_reputation as i32,
                                "",
                                auction.starting_price as i32,
                                riven.private,
                            )
                            .await?;
                    }
                }

                // Update database status to inactive
                if riven.status != "inactive" {
                    db.stock_riven()
                        .update_by_id(
                            riven.id,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some("inactive".to_string()),
                        )
                        .await?;
                }
                continue;
            }

            // Find Positive stats
            let positive_stats = riven
                .attributes
                .iter()
                .cloned()
                .filter(|a| a.positive && a.match_type.unwrap_or(false))
                .collect::<Vec<_>>();

            // Find Negative stats
            let negative_stats = riven
                .attributes
                .iter()
                .cloned()
                .filter(|a| !a.positive && a.match_type.unwrap_or(false))
                .collect::<Vec<_>>();

            // Match Rerolls
            let mut min_rerolls: Option<u8> = None;
            let mut max_rerolls: Option<u8> = None;
            if riven.match_riven.re_rolls.is_some() {
                let re_rolls = riven.match_riven.re_rolls.clone().unwrap();
                min_rerolls = Some(re_rolls.min as u8);
                max_rerolls = Some(re_rolls.max as u8);
            }

            // Search for live auctions for this riven
            let live_auctions = wfm
                .auction()
                .search(
                    "riven",
                    &riven.weapon_url,
                    Some(positive_stats.clone()),
                    negative_stats.get(0).cloned(),
                    None,
                    None,
                    None,
                    min_rerolls,
                    max_rerolls,
                    Some("direct"),
                    Some("price_asc"),
                )
                .await?;

            // Filter auctions that are not mine and are not closed and are player is ingame
            let live_auctions = live_auctions
                .iter()
                .filter(|a| {
                    a.owner.ingame_name != auth.ingame_name
                        && a.closed == false
                        && a.visible
                        && a.owner.status == "ingame"
                })
                .collect::<Vec<_>>();

            // Check if there are no live auctions continue to next riven if there is no live auctions
            if live_auctions.len() == 0 {
                logger::info_con(
                    "RivenModule",
                    format!("No live auctions for {}", riven.weapon_url).as_str(),
                );
                db.stock_riven()
                    .update_by_id(
                        riven.id,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some("no_offers".to_string()),
                    )
                    .await?;
                continue;
            }

            // Get the minimum price of the riven.
            let minimum_price = riven.minium_price;

            // Get the lowest price
            let lowest_price = live_auctions.get(0).unwrap().starting_price;

            // The new price of the riven
            let mut post_price = lowest_price;

            // Check if the rivens price is lower than the minimum price
            if minimum_price.is_some() && post_price < minimum_price.unwrap() as i64 {
                logger::info_con(
                    "RivenModule",
                    format!(
                        "Minimum price for {} is {} and yours is {}",
                        riven.weapon_url, lowest_price, riven.price
                    )
                    .as_str(),
                );
                post_price = minimum_price.unwrap() as i64;
            }

            // Calculate profit of the riven
            let profit = post_price as f64 - riven.price;

            // Print lowest price and profit
            logger::debug_con(
                "RivenModule",
                format!(
                    "Lowest price for {} is {} and yours is {} and profit is {}",
                    riven.weapon_url, lowest_price, riven.price, profit
                )
                .as_str(),
            );

            // Check if profit is greater than the range threshold
            let mut order_id: Option<String> = None;
            if profit > settings.stock_riven.range_threshold as f64 {
                // If profit is greater than the range threshold
                match auction {
                    Some(auction) => {
                        if auction.starting_price != post_price as i64 {
                            // Update auction
                            wfm.auction()
                                .update(
                                    auction.id.as_str(),
                                    post_price as i32,
                                    0,
                                    "",
                                    post_price as i32,
                                    true,
                                )
                                .await?;
                        }
                    }
                    None => {
                        // Post auction on warframe.market
                        let new_aut = wfm
                            .auction()
                            .create(
                                "riven",
                                "",
                                post_price,
                                post_price,
                                0,
                                1,
                                false,
                                AuctionItem {
                                    item_type: "riven".to_string(),
                                    weapon_url_name: Some(riven.weapon_url.clone()),
                                    re_rolls: Some(riven.re_rolls as i64),
                                    attributes: Some(riven.attributes.0.clone()),
                                    name: Some(riven.mod_name),
                                    mod_rank: Some(riven.rank as i64),
                                    polarity: Some(riven.polarity),
                                    mastery_level: Some(riven.mastery_rank as i64),
                                    element: None,
                                    quirk: None,
                                    having_ephemera: None,
                                    damage: None,
                                },
                            )
                            .await?;
                        order_id = Some(new_aut.id);
                    }
                }
                // Update database status to live
                if post_price != riven.listed_price.unwrap_or(0) as i64
                    || riven.status != "live"
                    || order_id.is_some()
                {
                    db.stock_riven()
                        .update_by_id(
                            riven.id,
                            order_id,
                            None,
                            Some(post_price as i32),
                            None,
                            None,
                            None,
                            None,
                            Some("live".to_string()),
                        )
                        .await?;
                }
            } else {
                // If profit is not greater than the range threshold
                match auction {
                    Some(auction) => {
                        if auction.visible {
                            // Update auction
                            wfm.auction()
                                .update(
                                    auction.id.as_str(),
                                    (riven.price + 50.0) as i32,
                                    0,
                                    "",
                                    (riven.price + 50.0) as i32,
                                    false,
                                )
                                .await?;
                        }
                    }
                    None => {}
                }
                if riven.listed_price.is_some() || riven.status != "to_low_profit" {
                    db.stock_riven()
                        .update_by_id(
                            riven.id,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some("to_low_profit".to_string()),
                        )
                        .await?;
                }
            }
        }
        Ok(())
    }
}
