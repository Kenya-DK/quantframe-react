use std::iter::Map;

use reqwest::header::HeaderMap;
use serde_json::{json, map};
use std::collections::HashMap; // Add missing import

use crate::{
    auth::AuthState, error::AppError, live_scraper::client::LiveScraperClient, logger,
    structs::AuctionItem,
};
#[derive(Clone)]
pub struct RivenModule {
    pub client: LiveScraperClient,
    pub debug_id: String,
    component: String,
    riven_auctions: HashMap<String, i32>,
}

impl RivenModule {
    pub fn new(client: LiveScraperClient) -> Self {
        RivenModule {
            client,
            debug_id: "wfm_client_item".to_string(),
            component: "Riven".to_string(),
            riven_auctions: HashMap::new(), // Change Map::new() to HashMap::new()
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_riven_module(self.clone());
    }
    pub async fn check_stock(&mut self) -> Result<(), AppError> {
        let db = self.client.db.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        logger::info_con("RivenModule", "Run Riven Stock Check");

        let stockrivens = db.stock_riven().get_rivens().await?;
        let my_auctions = wfm.auction().get_my_auctions().await?;
        let my_rivens = my_auctions
            .iter()
            .filter(|a| a.item.item_type == "riven".to_string())
            .collect::<Vec<_>>();
        let mut current_index = stockrivens.len();
        let total = stockrivens.len();
        for mut riven in stockrivens {
            current_index -= 1;

            // if riven.weapon_url != "torid" {
            //     continue;
            // }

            // Create a variable to check if the riven is dirty
            let mut is_dirty = false;

            self.client.send_message(
                "riven.checking",
                Some(json!({ "name": riven.weapon_name, "count": current_index, "total": total})),
            );
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
                    self.client
                        .send_message("riven.deleting", Some(json!({ "name": riven.weapon_name})));
                    wfm.auction().delete(auction.id.as_str()).await?;
                }

                // Update database status to inactive
                if riven.status != "inactive" {
                    db.stock_riven()
                        .update_by_id(
                            riven.id,
                            Some("".to_string()),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some("inactive".to_string()),
                            None,
                            None,
                            None,
                        )
                        .await?;
                }
                continue;
            }

            let riven_attributes = riven.attributes.iter().cloned().collect::<Vec<_>>();
            let match_data = riven.match_riven.clone();
            let use_match = match_data.enabled.unwrap_or(false);

            logger::log_json(
                format!("{} {} Match.json", riven.weapon_name, riven.mod_name).as_str(),
                &json!(match_data),
            )?;
            // Find Positive stats
            let positive_stats = match_data
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| !a.is_negative && a.is_required && use_match)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Find Negative stats
            let mut negative_stats = match_data
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| a.is_negative && a.is_required && use_match)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Match Required Negative stats
            if match_data.required_negative.unwrap_or(false) && use_match {
                negative_stats.push("has".to_string());
            }

            // Match Rerolls
            let mut min_rerolls: Option<i64> = None;
            let mut max_rerolls: Option<i64> = None;
            if riven.match_riven.re_rolls.is_some() && use_match {
                let re_rolls = riven.match_riven.re_rolls.clone().unwrap();
                if re_rolls.min != 0 {
                    min_rerolls = Some(re_rolls.min);
                }
                if re_rolls.max != 0 {
                    max_rerolls = Some(re_rolls.max);
                }
            }

            // Match Mastery Rank
            let mut min_mastery_rank: Option<i64> = None;
            let mut max_mastery_rank: Option<i64> = None;
            if riven.match_riven.mastery_rank.is_some() && use_match {
                let mastery_rank = riven.match_riven.mastery_rank.clone().unwrap();
                if mastery_rank.min != 0 {
                    min_mastery_rank = Some(mastery_rank.min);
                }
                if mastery_rank.max != 0 {
                    max_mastery_rank = Some(mastery_rank.max);
                }
            }

            // Match Polarity
            let mut polarity: Option<String> = None;
            if match_data.polarity.is_some() && use_match {
                if match_data.polarity.clone().unwrap() == "" {
                    polarity = Some("any".to_string());
                } else {
                    polarity = Some(match_data.polarity.clone().unwrap());
                }
            }

            // Search for live auctions for this riven
            self.client
                .send_message("riven.searching", Some(json!({ "name": riven.weapon_name})));
            let live_auctions = wfm
                .auction()
                .search(
                    "riven",
                    &riven.weapon_url,
                    Some(positive_stats.clone()),
                    negative_stats.get(0).cloned(),
                    polarity.as_deref(),
                    min_mastery_rank,
                    max_mastery_rank,
                    min_rerolls,
                    max_rerolls,
                    Some("direct"),
                    Some("price_asc"),
                    Some(riven_attributes.clone()),
                )
                .await?;

            // Filter auctions that are not mine and are not closed and are player is ingame and similarity is greater than the riven similarity
            let live_auctions = live_auctions
                .iter()
                .filter(|a| {
                    a.owner.ingame_name != auth.ingame_name
                        && a.closed == false
                        && a.visible
                        // && a.owner.status == "ingame"
                        && (a.item.similarity.unwrap_or(0.0)
                            >= if use_match {
                                match_data.similarity.unwrap_or(0.0)
                            } else {
                                0.0
                            })
                })
                .collect::<Vec<_>>();

            // Check if there are no live auctions continue to next riven if there is no live auctions
            if live_auctions.len() == 0 {
                logger::info_con(
                    "RivenModule",
                    format!("No live auctions for {}", riven.weapon_name).as_str(),
                );
                self.client
                    .send_message("riven.no_offers", Some(json!({ "name": riven.weapon_name})));
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
                        None,
                        None,
                        Some(live_auctions.clone()),
                    )
                    .await?;
                continue;
            }

            logger::log_json(
                format!("{} {}.json", riven.weapon_name, riven.mod_name).as_str(),
                &json!(live_auctions),
            )?;
            // Get the minimum price of the riven.
            let minimum_price = riven.minium_price;

            // Get the lowest price
            let lowest_price = live_auctions.get(0).unwrap().starting_price;

            // The new price of the riven
            let mut post_price = lowest_price;

            // Check if the rivens price is lower than the minimum price
            if minimum_price.is_some() && post_price < minimum_price.unwrap() as i64 {
                post_price = minimum_price.unwrap() as i64;
            }

            // Calculate profit of the riven
            let profit = post_price as f64 - riven.price;

            // Print lowest price and profit
            logger::debug_con(
                &self.get_component("CheckStock"),
                format!(
                    "Lowest price for {} is {} and yours is {} and profit is {}",
                    riven.weapon_name, lowest_price, riven.price, profit
                )
                .as_str(),
            );

            // Get Cache Auctions for this riven
            let cache_auctions = self.riven_auctions.get(riven.id.to_string().as_str());
            if cache_auctions.is_some() && cache_auctions.unwrap().to_owned() != live_auctions.len() as i32 {
                is_dirty = true;
                self.riven_auctions.remove(riven.id.to_string().as_str());
                self.riven_auctions.insert(riven.id.to_string(), live_auctions.len() as i32);
                self.update_state();
            }else if cache_auctions.is_none() {
                self.riven_auctions.insert(riven.id.to_string(), live_auctions.len() as i32);
                self.update_state();
            }

            // Check if profit is greater than the range threshold
            if profit > settings.stock_riven.range_threshold as f64 {
                // If profit is greater than the range threshold
                match auction {
                    Some(auction) => {
                        let listed_price = post_price as i32;
                        if auction.starting_price != post_price as i64 {
                            // Update auction
                            self.client.send_message(
                                "riven.updating",
                                Some(json!({ "name": riven.weapon_name, "price": post_price})),
                            );
                            wfm.auction()
                                .update(
                                    auction.id.as_str(),
                                    listed_price,
                                    0,
                                    "",
                                    listed_price,
                                    true,
                                )
                                .await?;

                        }
                        if riven.status != "live" || riven.listed_price != Some(listed_price) {
                            riven.status = "live".to_string();
                            riven.listed_price = Some(listed_price);
                            is_dirty = true;                            
                        }
                    }
                    None => {
                        // Post auction on warframe.market
                        self.client.send_message(
                            "riven.creating",
                            Some(json!({ "name": riven.weapon_name, "price": post_price})),
                        );
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
                                    similarity: None,
                                    extra_attributes: None,
                                    missing_attributes: None,
                                },
                            )
                            .await?;
                        riven.status = "live".to_string();
                        riven.listed_price = Some(post_price as i32);
                        riven.order_id = Some(new_aut.id);
                        is_dirty = true;
                    }
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
                            riven.status = "to_low_profit".to_string();
                            riven.listed_price = None;
                            is_dirty = true;
                        }
                    }
                    None => {}
                }
            }
            // Check if Stock Riven Was Changed
            if is_dirty {
                db.stock_riven()
                    .update_by_id(
                        riven.id,
                        riven.order_id.clone(),
                        None,
                        riven.listed_price.clone(),
                        None,
                        None,
                        None,
                        None,
                        Some(riven.status.clone()),
                        None,
                        None,
                        Some(live_auctions.clone()),
                    )
                    .await?;
            }
        }
        Ok(())
    }
}
