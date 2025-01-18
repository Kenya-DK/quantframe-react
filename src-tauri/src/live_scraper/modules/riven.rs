use std::collections::HashMap;

use entity::price_history::PriceHistoryVec;
use entity::{enums::stock_status::StockStatus, price_history::PriceHistory};

use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::wfm_client::types::auction::Auction;
use crate::{helper, DATABASE};
use crate::{
    live_scraper::{client::LiveScraperClient, types::riven_extra_info::AuctionDetails},
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
    component: String,
    stock_info: HashMap<i64, AuctionDetails>,
}

impl RivenModule {
    pub fn new(client: LiveScraperClient) -> Self {
        RivenModule {
            client,
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
        let conn = DATABASE.get().unwrap();
        let app = self.client.app.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let min_profit = settings.stock_riven.min_profit;
        let threshold_percentage = settings.stock_riven.threshold_percentage / 100.0;
        let limit_to = settings.stock_riven.limit_to;
        logger::info_con(&self.get_component("CheckStock"), "Run Riven Stock Check");

        // Send GUI Update.
        self.send_msg("stating", None);

        let stocks = StockRivenQuery::get_all(conn)
            .await
            .map_err(|e| AppError::new("RivenModule", eyre::eyre!(e)))?;

        let mut my_auctions = wfm.auction().get_my_auctions().await?;

        // Obtain only the rivens from my auctions
        my_auctions = my_auctions.filter_by_type("riven");

        let mut current_index: i64 = 0;
        let total = stocks.len() as i64;
        for mut stock_riven in stocks {
            // Check if client is running
            if !self.client.is_running() {
                continue;
            }
            current_index += 1;
            // Send GUI Update.
            self.send_msg(
                "searching_riven",
                Some(json!({ "current": current_index,"total": total, "weapon_name": stock_riven.weapon_name.clone(), "mod_name": stock_riven.mod_name.clone()})),
            );

            // Get my auction for this riven if exists, else create a new one with the empty values
            let mut user_auction = match my_auctions
                .get_auction_by_id(&stock_riven.wfm_order_id.clone().unwrap_or("".to_string()))
            {
                Some(mut auction) => {
                    auction.operation = vec![];
                    auction
                }
                None => Auction::default(),
            };

            if stock_riven.is_hidden {
                stock_riven.set_status(StockStatus::InActive);
                stock_riven.set_list_price(None);
                stock_riven.wfm_order_id(None);
                if user_auction.visible {
                    self.send_auction_update(
                        UIOperationEvent::Delete,
                        json!({ "id": user_auction.id}),
                    );
                    wfm.auction()
                        .delete(user_auction.id.as_str())
                        .await
                        .map_err(|e| AppError::new("RivenModule", eyre::eyre!(e.cause())))?;
                    self.stock_info.remove(&stock_riven.id);
                    self.update_state();
                }
                self.send_msg("riven_hidden", Some(json!({ "weapon_name": stock_riven.weapon_name.clone(), "mod_name": stock_riven.mod_name.clone()})));
                if stock_riven.is_dirty {
                    StockRivenMutation::update_by_id(conn, stock_riven.id, stock_riven.clone())
                        .await
                        .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
                    self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(stock_riven));
                }
                continue;
            }

            // Get Settings For Riven
            let match_data = stock_riven.filter.clone();

            // Get Search Query for Riven
            let (
                positive_stats,
                negative_stats,
                min_rerolls,
                max_rerolls,
                min_mastery_rank,
                max_mastery_rank,
                polarity,
            ) = match_data.get_auction_search_query();

            let mut live_auctions = wfm
                .auction()
                .search(
                    "riven",
                    &stock_riven.wfm_weapon_url,
                    positive_stats,
                    negative_stats,
                    polarity.as_deref(),
                    min_mastery_rank,
                    max_mastery_rank,
                    min_rerolls,
                    max_rerolls,
                    Some("direct"),
                    Some("price_asc"),
                )
                .await?;

            // Filter the auctions.
            live_auctions = live_auctions.remove_offline_auctions();
            live_auctions = live_auctions.filter_by_username(auth.ingame_name.as_str(), true);
            live_auctions =
                live_auctions.calculate__riven_similarity(stock_riven.attributes.0.clone());
            live_auctions.sort_by_platinum();

            // Get the price the item was bought for.
            let bought_price = stock_riven.bought;
            let highest_price = live_auctions.highest_price();
            let lowest_price = live_auctions.lowest_price();
            let total_sellers = live_auctions.total();
            let minimum_price = stock_riven.minimum_price;
            // Get the average price of the top X lowest prices
            let average_price = helper::calculate_average_of_top_lowest_prices(
                live_auctions.prices(),
                limit_to,
                threshold_percentage,
            );

            // Set the list price of the riven
            let mut post_price = average_price;

            // If minimum price is set and the post price is less than the minimum price then set the post price to be the minimum price
            if minimum_price.is_some() && post_price < minimum_price.unwrap() {
                post_price = minimum_price.unwrap();
            }

            // If the post price is 0, then set the status to NoSellers
            if live_auctions.total() == 0 {
                stock_riven.set_status(StockStatus::NoSellers);
                stock_riven.set_list_price(None);
                stock_riven.locked = true;
                user_auction.operation.push("Delete".to_string());
            }
            // Calculate profit of the riven
            let profit = post_price - bought_price;

            // If the profit is less than the minimum profit, then set the status to NoProfit
            if profit < min_profit {
                stock_riven.set_status(StockStatus::ToLowProfit);
                stock_riven.set_list_price(None);
                user_auction.operation.push("Delete".to_string());
            } else if user_auction.id == "N/A" {
                stock_riven.set_status(StockStatus::Live);
                stock_riven.set_list_price(Some(post_price));
                user_auction.operation.push("Created".to_string());
            } else {
                stock_riven.set_status(StockStatus::Live);
                stock_riven.set_list_price(Some(post_price));
                user_auction.operation.push("Updated".to_string());
            }

            // Get/Create Order Info
            let price_history =
                PriceHistory::new(chrono::Local::now().naive_local().to_string(), post_price);
            user_auction.info = match self.stock_info.get_mut(&stock_riven.id) {
                Some(auction_info) => {
                    if !user_auction.operation.contains(&"Delete".to_string()) {
                        // Update the order info with the current price history
                        auction_info.set_total_sellers(total_sellers);
                        auction_info.set_lowest_price(lowest_price);
                        auction_info.set_highest_price(highest_price);
                        auction_info.set_profit(profit);
                        auction_info.set_auctions(live_auctions.auctions.clone());
                        auction_info.add_price_history(price_history.clone());
                    }
                    auction_info.clone()
                }
                None => {
                    let mut auction_info = AuctionDetails::new(
                        0,
                        total_sellers,
                        profit,
                        lowest_price,
                        highest_price,
                        live_auctions.auctions.clone(),
                        stock_riven.price_history.clone().0,
                    );
                    if user_auction.operation.contains(&"Delete".to_string()) {
                        auction_info.is_dirty = false;
                    } else {
                        auction_info.add_price_history(price_history.clone());
                    }
                    auction_info
                }
            };

            // Update/Create/Delete the order on Warframe Market API and update the UI if needed.
            if user_auction.operation.contains(&"Delete".to_string()) {
                if user_auction.id != "N/A" {
                    match wfm
                        .auction()
                        .delete(&stock_riven.wfm_order_id.clone().unwrap())
                        .await
                    {
                        Ok(_) => {
                            self.send_auction_update(
                                UIOperationEvent::Delete,
                                json!({ "id": user_auction.id}),
                            );
                            self.stock_info.remove(&stock_riven.id);
                            self.update_state();
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            } else if user_auction.operation.contains(&"Created".to_string()) {
                match wfm
                    .auction()
                    .create(
                        "riven",
                        stock_riven.comment.clone().as_str(),
                        post_price,
                        post_price,
                        0,
                        1,
                        false,
                        AuctionItem::from_stock_riven(stock_riven.clone()),
                    )
                    .await
                {
                    Ok(auction) => {
                        user_auction.id = auction.id.clone();
                        user_auction.visible = true;
                        stock_riven.wfm_order_id(Some(auction.id.clone()));
                        self.send_auction_update(UIOperationEvent::CreateOrUpdate, json!(auction));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if user_auction.operation.contains(&"Updated".to_string())
                && stock_riven.is_dirty
            {
                match wfm
                    .auction()
                    .update(
                        &user_auction.id,
                        post_price,
                        0,
                        &stock_riven.comment.clone(),
                        post_price,
                        true,
                    )
                    .await
                {
                    Ok(a) => {
                        // Send GUI Update.
                        self.send_auction_update(UIOperationEvent::CreateOrUpdate, json!(a));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            // Update/Create/Delete the stock item on the database and update the UI if needed.
            if user_auction.info.is_dirty || stock_riven.is_dirty {
                stock_riven.price_history = PriceHistoryVec(
                    user_auction
                        .info
                        .price_history
                        .clone()
                        .into_iter()
                        .collect(),
                );
                StockRivenMutation::update_by_id(conn, stock_riven.id, stock_riven.clone())
                    .await
                    .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
                let mut payload = json!(stock_riven);
                payload["info"] = json!(user_auction.info);
                user_auction.info.is_dirty = false;
                user_auction.info.changes = None;
                if self.stock_info.contains_key(&stock_riven.id) {
                    *self.stock_info.get_mut(&stock_riven.id).unwrap() = user_auction.info.clone();
                } else {
                    self.stock_info
                        .insert(stock_riven.id.clone(), user_auction.info.clone());
                }
                self.update_state();
                self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(payload));
            }
        }
        Ok(())
    }
}
