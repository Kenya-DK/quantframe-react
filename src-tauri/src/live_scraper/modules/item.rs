use crate::cache::modules::item_price::ItemPriceModule;
use crate::database::modules::stock_item::StockItemStruct;
use crate::enums::{OrderMode, OrderType, StockStatus};
use crate::error;
use crate::live_scraper::client::LiveScraperClient;

use crate::structs::{Order, Orders, PriceHistory};
use crate::{error::AppError, logger};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::vec;
#[derive(Clone)]
pub struct ItemModule {
    pub client: LiveScraperClient,
    pub debug_id: String,
    component: String,
    interesting_items_cache:
        Arc<Mutex<HashMap<String, Vec<crate::cache::types::item_price_info::ItemPriceInfo>>>>,
}

impl ItemModule {
    pub fn new(client: LiveScraperClient) -> Self {
        ItemModule {
            client,
            debug_id: "wfm_client_item".to_string(),
            component: "Item".to_string(),
            interesting_items_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_item_module(self.clone());
    }
    pub async fn check_stock(&self) -> Result<(), AppError> {
        logger::info_con(&self.component, "Running Item Stock Check");
        // Load Managers.
        let db = self.client.db.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let cache = self.client.cache.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;

        // Get Settings.
        let order_mode = settings.stock_item.order_mode.clone();
        let blacklist_items: Vec<String> = settings.stock_item.blacklist.clone();

        // Variables.
        let mut interesting_items: Vec<String> = settings.stock_item.whitelist.clone();
        // Get interesting items from the price scraper if the order mode is buy or both.
        let price_scraper_interesting_items_new = self.get_interesting_items().await?;

        // Get interesting items from stock items if the order mode is sell or both and remove blacklisted items else return None.
        let stock_items_interesting_items: Option<Vec<StockItemStruct>> =
            if order_mode == OrderMode::Sell || order_mode == OrderMode::Both {
                Some(
                    db.stock_item()
                        .get_items()
                        .await?
                        .into_iter()
                        .filter(|item| item.owned > 0 && !item.hidden)
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };

        match stock_items_interesting_items.clone() {
            Some(items) => {
                for item in items {
                    interesting_items.push(item.url);
                }
            }
            None => {}
        };

        // Get My Orders from Warframe Market.
        let mut my_orders = wfm.orders().get_my_orders().await?;

        // Delete orders if the order mode is buy or sell.
        if order_mode == OrderMode::Buy || order_mode == OrderMode::Sell {
            let order_type = match order_mode {
                OrderMode::Buy => "sell",
                OrderMode::Sell => "buy",
                OrderMode::Both => "",
                OrderMode::Unknown(_) => "",
            };

            // Get order ids by order type and sort out blacklisted items.
            let order_ids = match order_type {
                "buy" => my_orders
                    .buy_orders
                    .iter()
                    .filter(|order| {
                        !blacklist_items.contains(&order.item.as_ref().unwrap().url_name)
                    })
                    .map(|order| order.id.clone())
                    .collect::<Vec<String>>(),
                "sell" => my_orders
                    .sell_orders
                    .iter()
                    .filter(|order| {
                        !blacklist_items.contains(&order.item.as_ref().unwrap().url_name)
                    })
                    .map(|order| order.id.clone())
                    .collect::<Vec<String>>(),
                _ => vec![],
            };
            for id in order_ids {
                wfm.orders().delete(&id).await?;
            }
        }

        // Get potential items to buy from the price scrape if the order mode is buy or both.
        if order_mode == OrderMode::Buy || order_mode == OrderMode::Both {
            let mut item_names = price_scraper_interesting_items_new
                .iter()
                .map(|item| item.url_name.clone())
                .collect::<Vec<String>>();
            interesting_items.append(&mut item_names);

            if my_orders.buy_orders.len() != 0 {
                // Filter only interesting items from the buy orders.
                let buy_orders_df = my_orders
                    .buy_orders
                    .into_iter()
                    .filter(|order| {
                        interesting_items.contains(&order.item.as_ref().unwrap().url_name)
                    })
                    .map(|order| {
                        let price = price_scraper_interesting_items_new
                            .iter()
                            .find(|item| item.url_name == order.item.as_ref().unwrap().url_name);
                        let mut order = order.clone();
                        order.closed_avg = Some(price.map(|item| item.avg_price).unwrap_or(0.0));
                        order.profit =
                            Some(order.closed_avg.unwrap_or(0.0) - order.platinum as f64);

                        order
                    })
                    .collect::<Vec<Order>>();

                // Update the buy orders with the filtered buy orders.
                my_orders.buy_orders = buy_orders_df.clone();
            }
        }
        // Remove duplicates from the interesting items.
        let interesting_items: HashSet<String> = HashSet::from_iter(interesting_items);
        let mut current_index = interesting_items.len();
        logger::info_file(
            &self.get_component("CheckStock"),
            format!(
                "Interesting Items ({}): {:?}",
                current_index, interesting_items
            )
            .as_str(),
            Some(self.client.log_file.as_str()),
        );
        // Loop through all interesting items
        for item in interesting_items.clone() {
            if self.client.is_running() == false || item == "" {
                continue;
            }
            // Find the item in the cache
            let item_info = match cache.item().find_type(&item)? {
                Some(item_info) => item_info,
                None => {
                    logger::warning(
                        &self.get_component("CheckStock"),
                        format!("Item: {} not found in cache", item).as_str(),
                        true,
                        Some(self.client.log_file.as_str()),
                    );
                    continue;
                }
            };
            current_index -= 1;
            // Log the current item
            logger::info_con(
                &self.get_component("CheckStock"),
                format!(
                    "Checking item: {}, ({}/{})",
                    item_info.item_name.clone(),
                    current_index,
                    interesting_items.len()
                )
                .as_str(),
            );
            self.client.send_message("item.checking", Some(json!({ "name": item_info.item_name.clone(), "count": current_index, "total": interesting_items.len()})));

            // Get the item orders from Warframe Market
            let mut live_orders = wfm.orders().get_orders_by_item(&item).await?;
            // Check if item_orders_df is empty and skip if it is
            if live_orders.total_count() == 0 {
                continue;
            }
            // logger::log_json("live_orders.json", &json!(live_orders))?;
            // Check if item is in stock items and get the stock item
            let stock_item = stock_items_interesting_items
                .clone()
                .unwrap_or_else(|| Vec::new())
                .into_iter()
                .find(|stock_item| stock_item.url == item_info.url_name);

            // Get the item stats from the price scraper
            let statistics = price_scraper_interesting_items_new
                .iter()
                .find(|item| item.url_name == item_info.url_name);

            // Get item_id from statistics or item_info
            let item_id = item_info.id.clone();

            // Get rank from statistics or item_info
            let item_rank: Option<f64> = if statistics.is_some() {
                statistics.unwrap().mod_rank
            } else {
                if item_info.mod_max_rank.is_none() {
                    None
                } else {
                    Some(item_info.mod_max_rank.clone().unwrap() as f64)
                }
            };
            // Get item moving average from statistics or item_info
            let moving_avg: f64 = if statistics.is_some() {
                statistics.unwrap().moving_avg.unwrap_or(0.0)
            } else {
                0.0
            };

            // Get Closed Avg from statistics or item_info
            let closed_avg: f64 = if statistics.is_some() {
                statistics.unwrap().avg_price
            } else {
                0.0
            };

            // Get all the live orders for the item from the Warframe Market API
            live_orders.sort_by_platinum();
            live_orders.filter_by_username(&auth.ingame_name, true);

            if order_mode == OrderMode::Buy || order_mode == OrderMode::Both {
                self.compare_live_orders_when_buying(
                    &item,
                    &item_id,
                    item_rank,
                    &mut my_orders.buy_orders,
                    &live_orders,
                    closed_avg,
                    stock_item.as_ref(),
                )
                .await?;
            }

            // Only check if the order mode is sell or both and if the item is in stock items
            if order_mode == OrderMode::Sell || order_mode == OrderMode::Both {
                self.compare_live_orders_when_selling(
                    &item,
                    &item_id,
                    item_rank,
                    moving_avg,
                    &my_orders.sell_orders,
                    &live_orders,
                    stock_item.clone(),
                )
                .await?;
            }
        }
        Ok(())
    }

    pub async fn delete_all_orders(&self, mode: OrderMode) -> Result<(), AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let blacklist = settings.stock_item.blacklist.clone();
        self.client.send_message(
            "item.deleting_orders",
            Some(json!({ "count": 0, "total": 0})),
        );
        let mut current_orders = wfm.orders().get_my_orders().await?;

        let mut orders = vec![];

        if mode == OrderMode::Buy || mode == OrderMode::Both {
            orders.append(&mut current_orders.buy_orders);
        }
        if mode == OrderMode::Sell || mode == OrderMode::Both {
            orders.append(&mut current_orders.sell_orders);
        }

        let mut current_index = 0;
        let total = orders.len();
        self.client.send_message(
            "item.deleting_orders",
            Some(json!({ "count": 0, "total": total})),
        );
        for order in orders {
            current_index += 1;
            self.client.send_message(
                "item.deleting_orders",
                Some(json!({ "count": current_index, "total": total})),
            );
            if self.client.is_running() == false {
                return Ok(());
            }
            // Check if item is in blacklist
            if blacklist.contains(&order.clone().item.unwrap().url_name) {
                continue;
            }
            match wfm.orders().delete(&order.id).await {
                Ok(_) => {}
                Err(e) => {
                    error::create_log_file(self.client.log_file.to_owned(), &e);
                    logger::warning_con(
                        &self.get_component("DeleteAllOrders"),
                        format!("Error trying to delete order: {:?}", e).as_str(),
                    );
                }
            };
        }
        Ok(())
    }

    pub async fn get_interesting_items(
        &self,
    ) -> Result<Vec<crate::cache::types::item_price_info::ItemPriceInfo>, AppError> {
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let cache = self.client.cache.lock()?.clone();
        let db = self.client.db.lock()?.clone();
        let volume_threshold = settings.stock_item.volume_threshold;
        let range_threshold = settings.stock_item.range_threshold;
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let price_shift_threshold = settings.stock_item.price_shift_threshold;
        let strict_whitelist = settings.stock_item.strict_whitelist;
        let whitelist = settings.stock_item.whitelist.clone();
        let black_list = settings.stock_item.blacklist.clone();
        let stock_item = db.stock_item().get_items_names().await?;

        // Create a query uuid.
        let query_id = format!(
            "get_buy|vol:{:?}ran:{:?}avg_p{:?}price_shift:{:?}strict_whitelist:{:?}whitelist{:?}blacklist:{:?}:mode:{:?}", 
            volume_threshold.clone(),
            range_threshold.clone(),
            avg_price_cap.clone(),
            price_shift_threshold.clone(),
            strict_whitelist.clone(),
            whitelist.clone().join(","),
            stock_item.join(","),
            settings.stock_mode.clone()
        );

        match self.get_cache_queried(&query_id) {
            Some(review) => {
                return Ok(review.clone());
            }
            None => {
                // Delete old queries
                self.remove_cache_queried(&query_id);
            }
        }

        let items = cache.item_price().get_items()?;
        let filtered_items = items
            .iter()
            .filter(|item| {
                item.order_type == "closed"
                    && item.volume > volume_threshold as f64
                    && item.range > range_threshold as f64
                    && !black_list.contains(&item.url_name)
                    && ((strict_whitelist && whitelist.contains(&item.url_name))
                        || ((!strict_whitelist
                            || whitelist.contains(&item.url_name)
                                && item.avg_price <= avg_price_cap as f64)
                            && item.week_price_shift >= price_shift_threshold as f64))
                    && item.week_price_shift >= price_shift_threshold as f64
                    || (stock_item.contains(&item.url_name) && item.order_type == "closed")
            })
            .cloned()
            .collect::<Vec<_>>();
        self.add_cache_queried(query_id, filtered_items.clone());
        Ok(filtered_items)
    }
    pub fn add_cache_queried(
        &self,
        key: String,
        df: Vec<crate::cache::types::item_price_info::ItemPriceInfo>,
    ) {
        self.interesting_items_cache.lock().unwrap().insert(key, df);
        self.update_state();
    }

    pub fn get_cache_queried(
        &self,
        key: &str,
    ) -> Option<Vec<crate::cache::types::item_price_info::ItemPriceInfo>> {
        self.interesting_items_cache
            .lock()
            .unwrap()
            .get(key)
            .cloned()
    }

    pub fn remove_cache_queried(&self, key: &str) {
        self.interesting_items_cache
            .lock()
            .unwrap()
            .retain(|k, _| !k.starts_with(key));
        self.update_state();
    }
    async fn get_my_order_information(
        &self,
        item_name: &str,
        _item_rank: Option<f64>,
        orders: &Vec<Order>,
    ) -> Result<(Option<String>, bool, i64, bool), AppError> {
        let order = orders
            .iter()
            .find(|order| order.item.as_ref().unwrap().url_name == item_name);

        if order.is_none() {
            return Ok((None, false, 0, false));
        }

        let order = order.unwrap();
        let id = order.id.clone();
        let visibility = order.visible;
        let price = order.platinum;
        // let rank = item_rank.unwrap_or(0);

        Ok((Some(id), visibility, price, true))
    }

    fn knapsack(
        &self,
        items: Vec<(i64, f64, String, String)>,
        max_weight: i64,
    ) -> Result<
        (
            i64,
            Vec<(i64, f64, String, String)>,
            Vec<(i64, f64, String, String)>,
        ),
        AppError,
    > {
        let n = items.len();
        let mut dp = vec![vec![0; (max_weight + 1) as usize]; (n + 1) as usize];

        for i in 1..=n {
            for w in 1..=max_weight {
                let (weight, value, _, _) = items[i - 1];
                if weight <= w {
                    dp[i][w as usize] =
                        dp[i - 1][w as usize].max(dp[i - 1][(w - weight) as usize] + value as i64);
                } else {
                    dp[i][w as usize] = dp[i - 1][w as usize];
                }
            }
        }

        let mut selected_items = Vec::new();
        let mut unselected_items = Vec::new();
        let mut w = max_weight;
        for i in (0..n).rev() {
            if dp[i + 1][w as usize] != dp[i][w as usize] {
                selected_items.push(items[i].clone());
                w -= items[i].0;
            } else {
                unselected_items.push(items[i].clone());
            }
        }

        Ok((dp[n][max_weight as usize], selected_items, unselected_items))
    }

    pub async fn compare_live_orders_when_buying(
        &self,
        item_name: &str,
        item_id: &str,
        item_rank: Option<f64>,
        my_orders: &mut Vec<Order>,
        live_orders: &Orders,
        closed_avg: f64,
        stock_item: Option<&StockItemStruct>,
    ) -> Result<Option<Vec<Order>>, AppError> {
        // Load Managers.
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let wfm = self.client.wfm.lock()?.clone();
        let current_orders = my_orders.clone();

        // Get Settings.
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let max_total_price_cap = settings.stock_item.max_total_price_cap;
        let mut status = StockStatus::InActive;

        // Get the current orders for the item from the Warframe Market API
        let (order_id, visibility, price, active) = self
            .get_my_order_information(item_name, item_rank, &current_orders)
            .await?;

        // let price_range = live_orders.get_price_range();

        // Probably don't want to be looking at this item right now if there's literally nobody interested in selling it.
        if live_orders.sell_orders.len() <= 0 {
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {item_name} has no sellers. Skipping.").as_str(),
            );
            return Ok(None);
        }

        // Get The highest buy order returns 0 if there are no buy orders.
        let highest_price = live_orders.highest_price(OrderType::Buy);

        // Get the price_range of the item.
        let price_range = live_orders.get_price_range();

        // Set the post price to the highest price.
        let mut post_price = highest_price;

        // Get the stock item bought price if it exists.
        let bought_price = if stock_item.is_some() {
            stock_item.unwrap().price as i64
        } else {
            0
        };

        // Get owned quantity.
        let owned = if stock_item.is_some() {
            stock_item.unwrap().owned as i64
        } else {
            0
        };

        // If there are no buyers, and the average price is greater than 25p, then we should probably update/create our listing.
        if post_price == 0 && closed_avg > 25.0 {
            // Calculate the post price
            // The post price is the maximum of two calculated values:
            // 1. The price range minus 40
            // 2. One third of the price range minus 1
            post_price = (price_range - 40).max((&price_range / 3) - 1);

            // Set the status to live
            status = StockStatus::Live;
        }

        // Get the average price of the item from the Warframe Market API
        let closed_avg_metric = (closed_avg - post_price as f64) as i64;

        // Get the potential profit from the average price of the item from the Warframe Market API
        let potential_profit = closed_avg_metric - 1;

        // Check if the post price is greater than the average price cap and set the status to overpriced if it is.
        if post_price > avg_price_cap as i64 && status != StockStatus::Live {
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {item_name} is overpriced, base of your average price cap of {avg_price_cap} and the current price is {post_price}").as_str(),
            );
            status = StockStatus::Overpriced;
        }

        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            return Ok(None);
        }

        let mut buy_orders_list: Vec<(i64, f64, String, String)> = vec![];

        // Check if either of the following conditions is true:
        // 1. The average closing price (`closed_avg_metric`) is 30 or more, and the price range is 15 or more.
        // 2. The price range (`price_range`) is 21 or more.
        // If either condition is true, the code inside the if statement will be executed.
        if (closed_avg_metric >= 30 && price_range >= 15)
            || price_range >= 21 && status != StockStatus::Live && !active
        {
            if my_orders.len() != 0 {
                buy_orders_list = my_orders
                    .iter()
                    .map(|order| {
                        let platinum = order.platinum;
                        let profit = order.profit.unwrap();
                        let url_name = order.item.as_ref().unwrap().url_name.clone();
                        let id = order.id.clone();
                        (platinum, profit as f64, url_name, id)
                    })
                    .collect::<Vec<(i64, f64, String, String)>>();
            }
            buy_orders_list.append(&mut vec![(
                post_price,
                potential_profit as f64,
                item_name.to_string(),
                "".to_string(),
            )]);

            // Call the `knapsack` method on `self` with the parameters `buy_orders_list` and `max_total_price_cap` cast to i64
            // The `knapsack` method is expected to return a tuple containing the maximum profit, the selected buy orders, and the unselected buy orders
            // If the method call fails (returns an error), propagate the error with `?`
            let (_, selected_buy_orders, unselected_buy_orders) =
                self.knapsack(buy_orders_list, max_total_price_cap as i64)?;

            // Get the selected item names from the selected buy orders
            let se_item_names: Vec<String> = selected_buy_orders
                .iter()
                .map(|order| order.2.clone())
                .collect();

            // Check if the selected item names contain the item name
            if se_item_names.contains(&item_name.to_string()) {
                // Check if the `un_item_names` vector is not empty
                if !unselected_buy_orders.is_empty() {
                    // If the vector is not empty, iterate over its elements
                    for unselected_item in &unselected_buy_orders {
                        // For each `unselected_item`, call the `delete` method on `orders()` of `wfm`
                        // The `delete` method is expected to delete an order with a specific name
                        // The name of the order is the fourth element (index 3) of `unselected_item`
                        // If the `delete` method call fails (returns an error), propagate the error with `?`
                        logger::warning_con(
                            &self.get_component("CompareOrdersWhenBuying"),
                            format!(
                                "Item {} order id {} is unselected. Deleted order.",
                                unselected_item.2.as_str(),
                                unselected_item.3.as_str()
                            )
                            .as_str(),
                        );
                        wfm.orders().delete(unselected_item.3.as_str()).await?;
                    }
                }

                status = StockStatus::Live;
            } else {
                // Set the status to underpriced if the post price is less than the average price cap
                status = StockStatus::Underpriced;
            }
        }

        if status == StockStatus::Underpriced && active {
            // Delete the order if it is active and underpriced
            logger::warning_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!(
                    "Item {item_name} is underpriced order id {}",
                    order_id.clone().unwrap_or("".to_string())
                )
                .as_str(),
            );
            wfm.orders()
                .delete(order_id.clone().unwrap().as_str())
                .await?;
            logger::warning_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {item_name} is underpriced. Deleted order.").as_str(),
            );
        } else if status == StockStatus::Live && active {
            wfm.orders()
                .update(
                    order_id.clone().unwrap().as_str(),
                    post_price as i32,
                    1,
                    visibility,
                )
                .await?;
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {item_name} Updated").as_str(),
            );
        } else if status == StockStatus::Live && !active {
            match wfm
                .orders()
                .create(item_id, "buy", post_price, 1, false, item_rank)
                .await
            {
                Ok((_rep, None)) => {}
                Ok((_, _)) => {}
                Err(e) => {
                    return Err(e);
                }
            }
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {item_name} Created").as_str(),
            );
        } else {
        }

        logger::log_json(
            format!("Buy Stats For {}.json", item_name).as_str(),
            &json!({
                "input": json!({
                    "item_name": item_name,
                    "item_id": item_id,
                    "item_rank": item_rank,
                    "closed_avg": closed_avg,
                    "stock_item": stock_item
                }),
                "settings": json!({
                    "avg_price_cap": avg_price_cap,
                    "max_total_price_cap": max_total_price_cap
                }),
                "my_order": json!({
                    "order_id": order_id,
                    "visibility": visibility,
                    "price": price,
                    "active": active,
                    "list": my_orders
                }),
                "status": status,
                "live_orders": live_orders,
                "highest_price": highest_price,
                "price_range": price_range,
                "post_price": post_price,
                "bought_price": bought_price,
                "owned": owned,
                "closed_avg_metric": closed_avg_metric,
                "owned": potential_profit,
            }),
        )?;

        Ok(None)
    }

    async fn compare_live_orders_when_selling(
        &self,
        item_name: &str,
        item_id: &str,
        item_rank: Option<f64>,
        moving_avg: f64,
        my_orders: &Vec<Order>,
        live_orders: &Orders,
        stock_item: Option<StockItemStruct>,
    ) -> Result<(), AppError> {
        // Load Managers.
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let wfm = self.client.wfm.lock()?.clone();
        let db = self.client.db.lock()?.clone();

        // Get Settings.
        let min_sma = settings.stock_item.min_sma;
        let minimum_profit = 10;
        let moving_avg = moving_avg as i64;

        // Get the current orders for the item from the Warframe Market API
        let (order_id, visibility, price, active) = self
            .get_my_order_information(item_name, item_rank, &my_orders)
            .await?;

        // Check if the item is in the inventory and if it is active and delete the order if it is not.
        if stock_item.is_none() && !active {
            return Ok(());
        } else if stock_item.is_none() && active
            || (stock_item.is_some() && stock_item.clone().unwrap().hidden)
        {
            self.client
                .send_message("item.sell.deleting", Some(json!({ "name": item_name})));

            wfm.orders()
                .delete(order_id.clone().unwrap().as_str())
                .await?;
            logger::info_con(
                &self.get_component("CompareOrdersWhenSelling"),
                format!("Item {item_name} is not in your inventory. Deleted order.",).as_str(),
            );
            return Ok(());
        }

        // Unwrap the StockItemStruct from the Option
        let mut stock_item = stock_item.clone().unwrap();
        let stock_item_original = stock_item.clone();

        // Create a PriceHistory struct
        let mut price_history = PriceHistory {
            user_id: "N/A".to_string(),
            name: "N/A".to_string(),
            price: 0,
            created_at: chrono::Local::now().naive_local().to_string(),
        };

        // Get the average price of the item.
        let bought_price = stock_item.price as i64;

        // Get the quantity of owned item.
        let quantity = stock_item.owned as i64;

        // Get the minimum price of the item.
        let minimum_price = stock_item.minium_price;

        // Get the lowest sell order price from the DataFrame of live sell orders
        let lowest_price = if live_orders.sell_orders.len() > 2 {
            let lowest_order = live_orders.lowest_order(OrderType::Sell).unwrap();
            price_history.user_id = lowest_order.user.clone().unwrap().id;
            price_history.name = lowest_order.user.clone().unwrap().ingame_name;
            lowest_order.platinum
        } else {
            stock_item.status = StockStatus::NoSellers.to_string();
            0
        };

        // Then Price the order will be posted for.
        let mut post_price = lowest_price;
        stock_item.status = StockStatus::Live.to_string();

        if bought_price > post_price {
            post_price = bought_price + minimum_profit;
        }

        // If the item is worth less than moving average the set the post price to be the moving average
        if post_price < (moving_avg - min_sma) as i64 {
            post_price = moving_avg;
            stock_item.status = StockStatus::SMALimit.to_string();
        }

        // If minimum price is set and the post price is less than the minimum price then set the post price to be the minimum price
        if minimum_price.is_some() && post_price < minimum_price.unwrap() as i64 {
            post_price = minimum_price.unwrap() as i64;
        }

        // Calculate the profit from the post price
        let profit = post_price - bought_price as i64;

        price_history.price = post_price;

        if profit <= 0 {
            stock_item.status = StockStatus::ToLowProfit.to_string();
            stock_item.listed_price = None;
        } else {
            let last_price_history = stock_item_original.price_history.last();
            if last_price_history.is_none() || last_price_history.unwrap().price != post_price {
                stock_item.price_history.push(price_history.clone());
            }
            stock_item.listed_price = Some(post_price as i32);
        }

        if active {
            // If the item is too cheap, delete the order
            if stock_item.status == StockStatus::ToLowProfit.to_string() {
                self.client
                    .send_message("item.sell.deleting", Some(json!({ "name": item_name})));
                wfm.orders()
                    .delete(order_id.clone().unwrap().as_str())
                    .await?;
            } else {
                wfm.orders()
                    .update(
                        order_id.clone().unwrap().as_str(),
                        post_price as i32,
                        quantity as i32,
                        visibility,
                    )
                    .await?;
            }
        } else if stock_item.status != StockStatus::ToLowProfit.to_string() {
            match wfm
                .orders()
                .create(item_id, "sell", post_price, quantity, true, item_rank)
                .await
            {
                Ok((rep, None)) => {
                    if &rep == "order_limit_reached" {
                        stock_item.status = StockStatus::OrderLimit.to_string();
                        stock_item.listed_price = None;
                    }
                }
                Ok((_, _)) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Update the stock item in the database
        if stock_item.listed_price != stock_item_original.listed_price
            || stock_item.status != stock_item_original.status
            || stock_item.price_history.len() != stock_item_original.price_history.len()
        {
            db.stock_item()
                .update_by_id(
                    stock_item.id,
                    None,
                    None,
                    None,
                    stock_item.listed_price,
                    Some(StockStatus::from_string(&stock_item.status)),
                    None,
                    Some(price_history),
                    Some(&live_orders.sell_orders),
                )
                .await?;
        }
        // current_orders: DataframeFromType<Vec<Order>>,
        // live_orders: &DataframeFromType<Vec<Order>>,
        logger::log_json(
            format!("Sell Stats For {}.json", item_name).as_str(),
            &json!({
                "input": json!({
                    "item_name": item_name,
                    "item_id": item_id,
                    "item_rank": item_rank,
                    "moving_avg": moving_avg,
                    "stock_item": stock_item,
                    "stock_item_original": stock_item_original,
                }),
                "order_info": json!({
                    "order_id": order_id,
                    "visibility": visibility,
                    "price": price,
                    "active": active,
                }),
                "restructure": json!({
                    "sellers": live_orders.sell_orders.len(),
                }),
                "current_orders": my_orders,
                "live_orders": live_orders,
                "post_price": post_price,
                "bought_price": bought_price,
                "profit": profit,
                "quantity": quantity,
                "minimum_price": minimum_price,
                "lowest_price": lowest_price,
            }),
        )?;
        return Ok(());
    }
}
