use std::collections::HashMap;

use entity::{enums::stock_status::StockStatus, price_history::PriceHistory};
use entity::stock::riven::*;

use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::{
    live_scraper::{client::LiveScraperClient, types::riven_extra_info::StockRivenDetails},
    logger,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::AppError,
    },
    wfm_client::types::auction_item::AuctionItem,
};

#[derive(Clone)]
pub struct RivenModule {
    pub client: LiveScraperClient,
    pub debug_id: String,
    component: String,
    stock_info: HashMap<i64, StockRivenDetails>,
}

impl RivenModule {
    pub fn new(client: LiveScraperClient) -> Self {
        RivenModule {
            client,
            debug_id: "wfm_client_item".to_string(),
            component: "Riven".to_string(),
            stock_info: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_riven_module(self.clone());
    }
    pub fn send_msg(&self, i18n_key: &str, values: Option<serde_json::Value>) {
        self.client
            .send_gui_update(format!("riven.{}", i18n_key).as_str(), values);
    }
    pub fn send_stock_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateStockRivens, operation, Some(value));
    }
    pub fn send_auction_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateAuction, operation, Some(value));
    }
    pub async fn check_stock(&mut self) -> Result<(), AppError> {
        let app = self.client.app.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let min_profit = settings.stock_riven.min_profit;
        let threshold_percentage = settings.stock_riven.threshold_percentage / 100;
        let limit_to = settings.stock_riven.limit_to;
        logger::info_con("RivenModule", "Run Riven Stock Check");

        // Send GUI Update.
        self.send_msg("stating", None);

        let stock_ids = StockRivenQuery::get_all_ids(&app.conn)
            .await
            .map_err(|e| AppError::new("RivenModule", eyre::eyre!(e)))?;

        let my_auctions = wfm.auction().get_my_auctions().await?;
        let my_rivens = my_auctions
            .iter()
            .filter(|a| a.item.item_type == "riven".to_string())
            .collect::<Vec<_>>();
        let mut current_index = stock_ids.len();
        let total = stock_ids.len();
        for id in stock_ids {
            // Get the stock riven
            let stock_riven = StockRivenQuery::get_by_id(&app.conn, id)
                .await
                .map_err(|e| AppError::new("RivenModule", eyre::eyre!(e)))?;
            if stock_riven.is_none() {
                continue;
            }
            let mut stock_riven = stock_riven.unwrap();

            // Clone the stock riven
            let stock_riven_original = stock_riven.clone();

            // Check if client is running
            if self.client.is_running() == false {
                break;
            }

            // Find my auction for this riven if exists
            let auction = my_rivens
                .iter()
                .find(|a| a.id == stock_riven.wfm_order_id.clone().unwrap_or("".to_string()));

            // Check if riven is private
            if stock_riven.is_hidden {
                // Update Auction on warframe.market
                if auction.is_some() {
                    let auction = auction.unwrap();
                    self.send_auction_update(
                        UIOperationEvent::CreateOrUpdate,
                        json!({ "id": auction.id}),
                    );
                    wfm.auction().delete(auction.id.as_str()).await?;
                }
                stock_riven.status = StockStatus::InActive;
                stock_riven.list_price = None;
                stock_riven.wfm_order_id = None;

                self.update_stock(
                    &stock_riven_original,
                    &mut stock_riven,
                    StockRivenDetails::new(None, None, None, None, None),
                )
                .await?;
                continue;
            }

            let riven_attributes = stock_riven.attributes.0.iter().cloned().collect::<Vec<_>>();
            let match_data = stock_riven.filter.clone();
            let use_match = match_data.enabled.unwrap_or(false);

            // Find Positive stats
            let positive_stats = match_data
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| a.positive && a.is_required && use_match)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Find Negative stats
            let mut negative_stats = match_data
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| !a.positive && a.is_required && use_match)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Match Required Negative stats
            if match_data.required_negative.unwrap_or(false) && use_match {
                negative_stats.push("has".to_string());
            }

            // Match Rerolls
            let mut min_rerolls: Option<i64> = None;
            let mut max_rerolls: Option<i64> = None;
            if stock_riven.filter.re_rolls.is_some() && use_match {
                let re_rolls = stock_riven.filter.re_rolls.clone().unwrap();
                if re_rolls.min != 0 {
                    min_rerolls = Some(re_rolls.min);
                }
                max_rerolls = re_rolls.max;
            }

            // Match Mastery Rank
            let mut min_mastery_rank: Option<i64> = None;
            let mut max_mastery_rank: Option<i64> = None;
            if stock_riven.filter.mastery_rank.is_some() && use_match {
                let mastery_rank = stock_riven.filter.mastery_rank.clone().unwrap();
                if mastery_rank.min != 0 {
                    min_mastery_rank = Some(mastery_rank.min);
                }
                max_mastery_rank = mastery_rank.max;
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

            // Send GUI Update.
            self.send_msg(
                "searching_riven",
                Some(json!({ "current": current_index,"total": total, "weapon_name": stock_riven.weapon_name.clone(), "mod_name": stock_riven.mod_name.clone()})),
            );

            // Get live auctions for this riven
            let live_auctions = wfm
                .auction()
                .search(
                    "riven",
                    &stock_riven.wfm_weapon_url,
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
                        && a.owner.status == "ingame"
                        && (a.item.similarity.unwrap_or(0.0)
                            >= if use_match {
                                match_data.similarity.unwrap_or(0.0) as f64
                            } else {
                                0.0
                            })
                })
                .cloned()
                .collect::<Vec<_>>();

            // Get the average price of the item.
            let bought_price = stock_riven.bought;

            // Get the minimum price of the item.
            let minimum_price = stock_riven.minimum_price;

            // Get the lowest sell order price from the DataFrame of live sell orders
            let lowest_price = if live_auctions.len() > 1 {
                // Get the top 5 lowest prices
                let top_lowest = live_auctions
                    .iter()
                    .map(|a| a.starting_price)
                    .take(limit_to as usize)
                    .collect::<Vec<_>>();

                // Find maximum and minimum prices
                let max_price = *top_lowest.iter().max().unwrap_or(&0);

                // Calculate 15% of the maximum price
                let threshold = max_price * threshold_percentage;

                // Filter out prices that are less than 15% lower than the maximum price
                let valid_prices: Vec<i64> = top_lowest
                    .iter()
                    .filter(|&price| *price >= max_price - (threshold as i64))
                    .cloned()
                    .collect();

                // Get the average price of the valid prices
                valid_prices.iter().sum::<i64>() / valid_prices.len() as i64
            } else {
                0
            };

            // The new price of the riven
            let mut post_price = lowest_price;

            // Check if the lowest price is 0 then set the status to NoSellers
            if lowest_price == 0 {
                stock_riven.status = StockStatus::NoSellers;
            } else {
                stock_riven.status = StockStatus::Live;
            }

            // If minimum price is set and the post price is less than the minimum price then set the post price to be the minimum price
            if minimum_price.is_some() && post_price < minimum_price.unwrap() {
                post_price = minimum_price.unwrap();
            }
            
            // Calculate profit of the riven
            let profit = post_price - bought_price;

            if profit <= min_profit {
                stock_riven.status = StockStatus::ToLowProfit;
                stock_riven.list_price = None;
            } else {
                stock_riven.list_price = Some(post_price);
            }

            // Check if there are no sellers
            if live_auctions.len() <= 1 {
                stock_riven.status = StockStatus::NoSellers;
                stock_riven.list_price = None;
            }

            let details = StockRivenDetails::new(
                Some(live_auctions.len() as i64),
                Some(profit),
                Some(lowest_price),
                Some(post_price),
                Some(live_auctions.clone()),
            );

            match auction {
                Some(auction) => {
                    if stock_riven.status == StockStatus::ToLowProfit
                        || stock_riven.status == StockStatus::NoSellers
                    {
                        wfm.auction().delete(auction.id.as_str()).await?;
                        self.send_auction_update(
                            UIOperationEvent::Delete,
                            json!({ "id": auction.id}),
                        );
                    } else if auction.starting_price != post_price as i64
                        || stock_riven.comment.clone() != auction.note
                    {
                        wfm.auction()
                            .update(
                                auction.id.as_str(),
                                post_price as i32,
                                0,
                                &stock_riven.comment.clone(),
                                post_price as i32,
                                true,
                            )
                            .await?;
                        // Send GUI Update.
                        self.send_auction_update(
                            UIOperationEvent::CreateOrUpdate,
                            json!(auction),
                        );
                    }
                }
                None => {
                    if stock_riven.status != StockStatus::ToLowProfit
                        && stock_riven.status != StockStatus::NoSellers
                    {
                        let new_aut = wfm
                            .auction()
                            .create(
                                "riven",
                                stock_riven.comment.clone().as_str(),
                                post_price,
                                post_price,
                                0,
                                1,
                                false,
                                AuctionItem {
                                    item_type: "riven".to_string(),
                                    weapon_url_name: Some(stock_riven.wfm_weapon_url.clone()),
                                    re_rolls: Some(stock_riven.re_rolls as i64),
                                    attributes: Some(stock_riven.attributes.0.clone()),
                                    name: Some(stock_riven.mod_name.clone()),
                                    mod_rank: Some(
                                        stock_riven.sub_type.clone().unwrap().rank.unwrap_or(0),
                                    ),
                                    polarity: Some(stock_riven.polarity.clone()),
                                    mastery_level: Some(stock_riven.mastery_rank as i64),
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
                        stock_riven.wfm_order_id = Some(new_aut.id.clone());
                        // Send GUI Update.
                        self.send_msg(
                            "riven_created",
                            Some(json!({ "weapon_name": stock_riven.weapon_name.clone(), "mod_name": stock_riven.mod_name.clone(),"price": post_price, "profit": profit})),
                        );
                        self.send_auction_update(
                            UIOperationEvent::CreateOrUpdate,
                            json!(new_aut),
                        );
                    }
                }
            }
            self.update_stock(&stock_riven_original, &mut stock_riven, details)
                .await?;

            logger::debug_con(
                &self.get_component("CheckStock"),
                format!(
                    "Name: {} {} | Lowest Price: {} | Sell Price: {} | Profit: {} | Total Sellers: {} | Status: {}",
                    stock_riven.weapon_name, stock_riven.mod_name, lowest_price, post_price, profit, live_auctions.len(), stock_riven.status.as_str()
                )
                .as_str(),
            );
            current_index -= 1;
        }
        Ok(())
    }
    async fn update_stock(
        &mut self,
        stock_riven_original: &stock_riven::Model,
        stock_riven: &mut stock_riven::Model,
        details: StockRivenDetails,
    ) -> Result<(), AppError> {
        let app = self.client.app.lock()?.clone();
        let mut need_update = false;

        // Get Stock Cache Info
        let stock_info = self.stock_info.get(&stock_riven.id);

        if stock_info.is_some() {
            let stock_info = stock_info.unwrap();
            if stock_info.total_sellers != details.total_sellers {
                need_update = true;
            } else if stock_info.profit != details.profit {
                need_update = true;
            } else if stock_info.lowest_price != details.lowest_price {
                need_update = true;
            } else if stock_info.highest_price != details.highest_price {
                need_update = true;
            } else if stock_info.auctions.is_some()
                && details.auctions.is_some()
                && (stock_info.auctions.clone().unwrap().len()
                    != details.auctions.clone().unwrap().len())
            {
                need_update = true;
            }
        } else {
            need_update = true;
        }

        self.stock_info.insert(stock_riven.id, details.clone());
        self.update_state();
        // Check if the stock riven needs to be updated
        if stock_riven_original.wfm_order_id != stock_riven.wfm_order_id {
            need_update = true;
        } else if stock_riven_original.status != stock_riven.status {
            need_update = true;
        } else if stock_riven_original.list_price != stock_riven.list_price {
            // Create a PriceHistory struct
            if stock_riven.list_price.is_some() {
                let price_history =
                    PriceHistory::new(chrono::Local::now().naive_local().to_string(), stock_riven.list_price.unwrap());
                let last_price_history = stock_riven_original.price_history.0.last();
                if last_price_history.is_none() || last_price_history.unwrap().price != stock_riven.list_price.unwrap() {
                    stock_riven.price_history.0.push(price_history.clone());
                }
            }
            need_update = true;
        }

        if need_update {
            logger::info_con(
                &self.get_component("UpdateStock"),
                format!(
                    "Name: {} {} | Order ID: {:?} -> {:?} | Listed Price: {:?} -> {:?} | Status: {:?} -> {:?}",
                    stock_riven.weapon_name,
                    stock_riven.mod_name,
                    stock_riven_original.wfm_order_id,
                    stock_riven.wfm_order_id,
                    stock_riven_original.list_price,
                    stock_riven.list_price,
                    stock_riven_original.status,
                    stock_riven.status
                )
                .as_str(),
            );
            let mut payload = json!(stock_riven);
            payload["info"] = json!(details);
            self.send_stock_update(UIOperationEvent::CreateOrUpdate, payload);
            StockRivenMutation::update_by_id(&app.conn, stock_riven.id, stock_riven.clone())
                .await
                .map_err(|e| AppError::new("RivenModule", eyre::eyre!(e)))?;
        }
        Ok(())
    }
}
