use crate::cache::types::cache_tradable_item::CacheTradableItem;
use crate::enums::order_mode::OrderMode;
use crate::live_scraper::client::LiveScraperClient;

use crate::live_scraper::types::item_entry::ItemEntry;
use crate::live_scraper::types::item_extra_info::StockItemDetails;
use crate::live_scraper::types::order_extra_info::OrderDetails;
use crate::utils::enums::log_level::LogLevel;
use crate::utils::enums::ui_events::{UIEvent, UIOperationEvent};
use crate::utils::modules::error::{self, AppError};
use crate::utils::modules::logger;
use crate::wfm_client::enums::order_type::OrderType;
use crate::wfm_client::types::order::Order;
use crate::wfm_client::types::orders::Orders;
use entity::enums::stock_status::StockStatus;
use entity::price_history::PriceHistory;
use entity::stock::item::stock_item;

use serde_json::json;
use service::{StockItemMutation, StockItemQuery};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::vec;
#[derive(Clone)]
pub struct ItemModule {
    pub client: LiveScraperClient,
    pub debug_id: String,
    component: String,
    stock_info: HashMap<i64, StockItemDetails>,
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
            stock_info: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_item_module(self.clone());
    }
    pub fn send_msg(&self, i18n_key: &str, values: Option<serde_json::Value>) {
        self.client
            .send_gui_update(format!("item.{}", i18n_key).as_str(), values);
    }
    pub fn send_stock_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateStockItems, operation, Some(value));
    }
    pub fn send_order_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateOrders, operation, Some(value));
    }

    pub async fn check_stock(&self) -> Result<(), AppError> {
        logger::info_con(&self.component, "Running Item Stock Check");

        // Load Managers.
        let app = self.client.app.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let cache = self.client.cache.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;

        // Send GUI Update.
        self.send_msg("stating", None);

        // Get Settings.
        let order_mode = settings.stock_item.order_mode.clone();
        let blacklist_items: Vec<String> = settings.stock_item.blacklist.clone();

        // Variables.
        let mut interesting_items: Vec<ItemEntry> =
            ItemEntry::from_string_list(settings.stock_item.whitelist.clone());

        // Get interesting items from the price scraper if the order mode is buy or both.
        let price_scraper_interesting_items_new = self.get_interesting_items().await?;

        // Get interesting items from stock items if the order mode is sell or both and remove blacklisted items else return None.
        let stock_items_interesting_items: Option<Vec<stock_item::Model>> =
            if order_mode == OrderMode::Sell || order_mode == OrderMode::Both {
                Some(
                    StockItemQuery::get_all_stock_items(&app.conn, 0)
                        .await
                        .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?,
                )
            } else {
                None
            };

        match stock_items_interesting_items.clone() {
            Some(items) => {
                for item in items {
                    interesting_items.push(ItemEntry::from_stock_item(&item));
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
                // Send GUI Update.
                wfm.orders().delete(&id).await?;
                self.send_order_update(UIOperationEvent::Delete, json!({"id": id}));
            }
        }

        // Get potential items to buy from the price scrape if the order mode is buy or both.
        if order_mode == OrderMode::Buy || order_mode == OrderMode::Both {
            let mut item_names = price_scraper_interesting_items_new
                .iter()
                .map(|item| ItemEntry::from_item_price(item))
                .collect::<Vec<ItemEntry>>();
            interesting_items.append(&mut item_names);

            if my_orders.buy_orders.len() != 0 {
                // Filter only interesting items from the buy orders.
                let buy_orders_df = my_orders
                    .buy_orders
                    .into_iter()
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
        logger::log_json("interesting_items.json", &json!(interesting_items.clone()))?;

        let mut interesting_items: HashSet<ItemEntry> = HashSet::from_iter(interesting_items);
        // Remove empty items from the interesting items.
        interesting_items = interesting_items
            .into_iter()
            .filter(|item| item.wfm_url != "")
            .collect();

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

        // Create a cache for the orders.
        let mut orders_cache: HashMap<String, Orders> = HashMap::new();

        logger::info_con(
            &self.get_component("CheckStock"),
            format!("Checking {} items", interesting_items.len()).as_str(),
        );

        // Sort the interesting items by the priority.
        let mut interesting_items: Vec<ItemEntry> = interesting_items.into_iter().collect();
        interesting_items.sort_by(|a, b| a.priority.cmp(&b.priority));

        // Loop through all interesting items
        for item_entry in interesting_items.clone() {
            if self.client.is_running() == false {
                current_index -= 1;
                continue;
            }
            // Find the item in the cache
            let item_info = match cache
                .tradable_items()
                .get_by(&item_entry.wfm_url, "--item_by url_name --item_lang en")?
            {
                Some(item_info) => item_info,
                None => {
                    logger::warning(
                        &self.get_component("CheckStock"),
                        format!("Item: {} not found in cache", item_entry.wfm_url).as_str(),
                        true,
                        Some(self.client.log_file.as_str()),
                    );
                    continue;
                }
            };

            // Send GUI Update.
            self.send_msg("checking_item", Some(json!({ "current": current_index,"total": interesting_items.len(), "name": item_info.name.clone()})));

            // Log the current item
            logger::info_con(
                &self.get_component("CheckStock"),
                format!(
                    "Checking item: {}, ({}/{})",
                    item_info.name.clone(),
                    current_index,
                    interesting_items.len()
                )
                .as_str(),
            );

            // Get the item orders from Warframe Market or the cache.
            let mut live_orders = if orders_cache.contains_key(&item_entry.wfm_url) {
                orders_cache.get(&item_entry.wfm_url).unwrap().clone()
            } else {
                let orders = wfm.orders().get_orders_by_item(&item_entry.wfm_url).await?;
                orders_cache.insert(item_entry.wfm_url.clone(), orders.clone());
                orders
            };

            // Remove all orders where the sub type is not the same as the stock item sub type.
            live_orders = live_orders.filter_by_sub_type(item_entry.sub_type.as_ref(), false);

            // Check if item_orders_df is empty and skip if it is
            if live_orders.total_count() == 0 {
                logger::info_con(
                    &self.get_component("CheckStock"),
                    format!("Item {} has no orders. Skipping.", item_info.name).as_str(),
                );
                continue;
            }
            let stock_item = match item_entry.stock_id {
                Some(stock_id) => match StockItemQuery::get_by_id(&app.conn, stock_id).await {
                    Ok(stock_item) => stock_item,
                    Err(e) => {
                        error::create_log_file(
                            self.client.log_file.to_owned(),
                            &AppError::new(&self.component, eyre::eyre!(e)),
                        );
                        None
                    }
                },
                None => None,
            };

            // Get the item stats from the price scraper
            let statistics = price_scraper_interesting_items_new.iter().find(|item| {
                item.url_name == item_info.wfm_url_name && item.sub_type == item_entry.sub_type
            });

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
            live_orders = live_orders.filter_by_username(&auth.ingame_name, true);
            live_orders.sort_by_platinum();

            // Only check if the order mode is buy or both and if the item is in stock items
            if order_mode == OrderMode::Buy || order_mode == OrderMode::Both {
                match self
                    .compare_live_orders_when_buying(
                        &item_info,
                        item_entry,
                        &mut my_orders,
                        live_orders.clone(),
                        closed_avg,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if e.log_level() == LogLevel::Warning {
                            self.client.report_error(&e);
                        } else {
                            return Err(e);
                        }
                    }
                }
            }

            // Only check if the order mode is sell or both and if the item is in stock items
            if (order_mode == OrderMode::Sell || order_mode == OrderMode::Both)
                && stock_item.is_some()
            {
                match self
                    .compare_live_orders_when_selling(
                        &item_info,
                        moving_avg,
                        &mut my_orders,
                        live_orders.clone(),
                        &mut stock_item.unwrap(),
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if e.log_level() == LogLevel::Warning {
                            self.client.report_error(&e);
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            current_index -= 1;
        }
        Ok(())
    }

    pub async fn delete_all_orders(&self, mode: OrderMode) -> Result<(), AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        let app = self.client.app.lock()?.clone();
        let _notify = self.client.notify.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let blacklist = settings.stock_item.blacklist.clone();
        let mut current_orders = wfm.orders().get_my_orders().await?;

        match StockItemMutation::update_all(&app.conn, StockStatus::Pending, None).await {
            Ok(orders) => {
                self.send_stock_update(UIOperationEvent::Set, json!(orders));
            }
            Err(e) => {
                error::create_log_file(
                    self.client.log_file.to_owned(),
                    &AppError::new(&self.component, eyre::eyre!(e)),
                );
            }
        }

        let mut orders = vec![];

        if mode == OrderMode::Buy || mode == OrderMode::Both {
            orders.append(&mut current_orders.buy_orders);
        }
        if mode == OrderMode::Sell || mode == OrderMode::Both {
            orders.append(&mut current_orders.sell_orders);
        }

        let mut current_index = 0;
        let total = orders.len();
        for order in orders {
            current_index += 1;
            if self.client.is_running() == false {
                self.send_msg("idle", None);
                return Ok(());
            }
            // Send GUI Update.
            self.send_msg(
                "deleting_orders",
                Some(json!({ "current": current_index,"total": total})),
            );
            // Check if item is in blacklist
            if blacklist.contains(&order.clone().item.unwrap().url_name) {
                continue;
            }
            match wfm.orders().delete(&order.id).await {
                Ok(_) => {
                    // Send GUI Update.
                    self.send_order_update(UIOperationEvent::Delete, json!({"id": order.id}));
                }
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
        let app = self.client.app.lock()?.clone();
        let volume_threshold = settings.stock_item.volume_threshold;
        let range_threshold = settings.stock_item.range_threshold;
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let trading_tax_cap = settings.stock_item.trading_tax_cap;
        let price_shift_threshold = settings.stock_item.price_shift_threshold;
        let strict_whitelist = settings.stock_item.strict_whitelist;
        let whitelist = settings.stock_item.whitelist.clone();
        let black_list = settings.stock_item.blacklist.clone();
        let stock_item = StockItemQuery::get_all_stock_items(&app.conn, 0)
            .await
            .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?
            .iter()
            .map(|item| item.wfm_url.clone())
            .collect::<Vec<String>>();

        // Create a query uuid.
        let query_id = format!(
            "get_buy|vol:{:?}ran:{:?}avg_p{:?}tax_p{:?}price_shift:{:?}strict_whitelist:{:?}whitelist{:?}blacklist:{:?}:mode:{:?}", 
            volume_threshold.clone(),
            range_threshold.clone(),
            avg_price_cap.clone(),
            trading_tax_cap.clone(),
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
                    && ((trading_tax_cap >= -1 && item.trading_tax <= trading_tax_cap as f64)
                        || trading_tax_cap <= -1)
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

    fn knapsack(
        &self,
        items: Vec<(i64, f64, String, String)>,
        avg_price_cap: i64,
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
        let mut dp = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let (weight, value, _, _) = items[i - 1];
            dp[i] = (weight <= avg_price_cap).then(|| value as i64).unwrap_or(0);
        }
        let mut selected_items = Vec::new();
        let mut unselected_items = Vec::new();
        let mut w = max_weight;
        for i in 0..n - 1 {
            if w - items[i].0 < 0 {
                unselected_items.push(items[i].clone());
            } else if dp[i + 1] != 0 {
                selected_items.push(items[i].clone());
                w -= items[i].0;
            } else {
                unselected_items.push(items[i].clone());
            }
        }

        // In the `items` parameter, the last element is always not on Warframe Market (the one currently getting checked),
        // so it should be added only if it's not already posted, unless the price would go over the max price cap limit.
        // Because if it is posted and gets added in unselected_items,
        // it will be expecting an order_id because the item is posted on Warframe Market.
        if !selected_items
            .iter()
            .any(|&(_, _, ref name, _)| name == &items[n - 1].2)
        {
            if w - items[n - 1].0 < 0 {
                unselected_items.push(items[n - 1].clone());
            } else {
                selected_items.push(items[n - 1].clone());
            }
        }

        Ok((dp[n], selected_items, unselected_items))
    }

    pub async fn compare_live_orders_when_buying(
        &self,
        item_info: &CacheTradableItem,
        item: ItemEntry,
        my_orders: &mut Orders,
        live_orders: Orders,
        closed_avg: f64,
    ) -> Result<Option<Vec<Order>>, AppError> {
        // Load Managers.
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let wfm = self.client.wfm.lock()?.clone();
        let blacklist = settings.stock_item.blacklist.clone();

        // Check if the item is in the blacklist and skip if it is
        if blacklist.contains(&item_info.wfm_url_name) {
            return Ok(None);
        }

        // Get Settings.
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let max_total_price_cap = settings.stock_item.max_total_price_cap;
        let mut status = StockStatus::InActive;

        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_url_name,
            OrderType::Buy,
            item.sub_type.as_ref(),
        ) {
            Some(order) => order,
            None => Order::default(),
        };

        // Probably don't want to be looking at this item right now if there's literally nobody interested in selling it.
        if live_orders.sell_orders.len() <= 0 {
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} has no sellers. Skipping.", item_info.name).as_str(),
            );
            return Ok(None);
        }

        // Get The highest buy order returns 0 if there are no buy orders.
        let highest_price = live_orders.highest_price(OrderType::Buy);

        // Get the price_range of the item.
        let price_range = live_orders.get_price_range();

        // Set the post price to the highest price.
        let mut post_price = highest_price;

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
                format!("Item {} is overpriced, base of your average price cap of {} and the current price is {}", item_info.name, avg_price_cap, post_price).as_str(),
            );
            status = StockStatus::Overpriced;
        }

        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            return Ok(None);
        }

        // Get Order Info
        let order_info_original = user_order.info.clone();
        let mut order_info = match user_order.info {
            Some(order_info) => order_info,
            None => OrderDetails::default(),
        };
        // Update the order info with the current price history
        order_info.highest_price = highest_price;
        order_info.lowest_price = live_orders.lowest_price(OrderType::Buy);
        order_info.total_buyers = live_orders.buy_orders.len() as i64;
        order_info.orders = live_orders.buy_orders.clone();
        order_info.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));
        user_order.info = Some(order_info.clone());

        if order_info_original.is_none() {
            user_order.operation = "Updated".to_string();
        }

        let mut buy_orders_list: Vec<(i64, f64, String, String)> = vec![];

        // Check if either of the following conditions is true:
        // 1. The average closing price (`closed_avg_metric`) is 30 or more, and the price range is 15 or more.
        // 2. The price range (`price_range`) is 21 or more.
        // If either condition is true, the code inside the if statement will be executed.
        if (closed_avg_metric >= 30 && price_range >= 15)
            || price_range >= 21 && status != StockStatus::Live && !user_order.visible
        {
            if my_orders.buy_orders.len() != 0 {
                buy_orders_list = my_orders
                    .buy_orders
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

            // Its important that the currently checking item is appended to `buy_orders_list`
            // as the last element so that it doesn't break the way knapsack works.
            buy_orders_list.append(&mut vec![(
                post_price,
                potential_profit as f64,
                item_info.wfm_url_name.clone(),
                "".to_string(),
            )]);

            // Call the `knapsack` method on `self` with the parameters `buy_orders_list`, `avg_price_cap` and `max_total_price_cap` cast to i64
            // The `knapsack` method is expected to return a tuple containing the maximum profit, the selected buy orders, and the unselected buy orders
            // If the method call fails (returns an error), propagate the error with `?`
            let (_, selected_buy_orders, unselected_buy_orders) =
                self.knapsack(buy_orders_list, avg_price_cap, max_total_price_cap)?;

            // Get the selected item names from the selected buy orders
            let se_item_names: Vec<String> = selected_buy_orders
                .iter()
                .map(|order| order.2.clone())
                .collect();

            // Check if the selected item names contain the item name
            if se_item_names.contains(&item_info.wfm_url_name) {
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
                        // Send GUI Update.
                        self.send_msg("knapsack_delete", Some(json!({ "name": item_info.name})));
                        self.send_order_update(
                            UIOperationEvent::Delete,
                            json!({"id": unselected_item.3}),
                        );

                        wfm.orders().delete(&unselected_item.3).await?;
                        my_orders.delete_order_by_id(OrderType::Buy, &unselected_item.3);
                    }
                }

                status = StockStatus::Live;
            } else {
                // Set the status to underpriced if the post price is less than the average price cap
                status = StockStatus::Underpriced;
            }
        }

        if status == StockStatus::Underpriced && user_order.visible {
            // Delete the order if it is active and underpriced
            logger::warning_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!(
                    "Item {} is underpriced order id {}",
                    item_info.name,
                    user_order.id.as_str()
                )
                .as_str(),
            );

            // Send GUI Update.
            self.send_msg(
                "underpriced_delete",
                Some(json!({ "name": item_info.name.clone()})),
            );
            user_order.operation = "Deleted".to_string();

            logger::warning_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} is underpriced. Deleted order.", item_info.name).as_str(),
            );
        } else if status == StockStatus::Live && user_order.id != "N/A" {
            wfm.orders()
                .update(&user_order.id, post_price, 1, user_order.visible)
                .await?;
            user_order.platinum = post_price;
            user_order.operation = "Updated".to_string();

            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} Updated", item_info.name).as_str(),
            );
        } else if status == StockStatus::Live && user_order.id == "N/A" {
            // Send GUI Update.
            self.send_msg("created", Some(json!({ "name": item_info.name, "price": post_price, "profit": potential_profit})));
            user_order = match wfm
                .orders()
                .create(&item_info.wfm_id, "buy", post_price, 1, true, item.sub_type)
                .await
            {
                Ok((rep, None)) => {
                    if &rep == "order_limit_reached" {
                        // Send GUI Update.
                        self.send_msg(
                            "order_limit_reached",
                            Some(json!({ "name": item_info.name.clone()})),
                        );
                    }
                    Order::default()
                }
                Ok((_, Some(mut order))) => {
                    order.info = Some(order_info.clone());
                    order.profit = Some(potential_profit as f64);
                    order.closed_avg = Some(closed_avg);
                    order.operation = "Created".to_string();
                    order
                }
                Err(e) => {
                    return Err(e);
                }
            };
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} Created", item_info.name).as_str(),
            );
        } else {
        }
        if user_order.operation == "Deleted" {
            my_orders.delete_order_by_id(OrderType::Buy, &user_order.id);
            self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
            wfm.orders().delete(&user_order.id).await?;
        } else if user_order.operation == "Created" {
            my_orders.buy_orders.push(user_order.clone());
            self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order.clone()));
        }

        Ok(None)
    }

    async fn compare_live_orders_when_selling(
        &self,
        item_info: &CacheTradableItem,
        moving_avg: f64,
        my_orders: &mut Orders,
        live_orders: Orders,
        stock_item: &mut stock_item::Model,
    ) -> Result<(), AppError> {
        // Load Managers.
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let wfm = self.client.wfm.lock()?.clone();
        let app = self.client.app.lock()?.clone();
        let blacklist = settings.stock_item.blacklist.clone();

        // Check if the item is in the blacklist and skip if it is
        if blacklist.contains(&item_info.wfm_url_name) {
            return Ok(());
        }

        // Get Settings.
        let min_sma = settings.stock_item.min_sma;
        let minimum_profit = settings.stock_item.min_profit;
        let moving_avg = moving_avg as i64;

        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_url_name,
            OrderType::Sell,
            stock_item.sub_type.as_ref(),
        ) {
            Some(order) => order,
            None => Order::default(),
        };

        // If the order is visible and the item is hidden, delete the order.
        if stock_item.is_hidden {
            stock_item.status = StockStatus::InActive;
            if user_order.visible {
                wfm.orders().delete(&user_order.id).await?;
                my_orders.delete_order_by_id(OrderType::Sell, &user_order.id);
                self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
            }

            // Send GUI Update.
            self.send_msg("is_hidden", Some(json!({ "name": item_info.name.clone()})));
            self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(stock_item));
            return Ok(());
        }

        let stock_item_original = stock_item.clone();

        // Remove all orders where the sub type is not the same as the stock item sub type.
        let live_orders = live_orders.filter_by_sub_type(stock_item.sub_type.as_ref(), false);

        // Get the average price of the item.
        let bought_price = stock_item.bought as i64;

        // Get the quantity of owned item.
        let quantity = stock_item.owned as i64;

        // Get the minimum price of the item.
        let minimum_price = stock_item.minimum_price;

        // Get the lowest sell order price from the DataFrame of live sell orders
        let lowest_price = if live_orders.sell_orders.len() > 2 {
            live_orders.lowest_price(OrderType::Sell)
        } else {
            stock_item.status = StockStatus::NoSellers;
            0
        };

        // Get the highest sell order price from the DataFrame of live sell orders
        let highest_price = live_orders.highest_price(OrderType::Sell);

        // Then Price the order will be posted for.
        let mut post_price = lowest_price;
        stock_item.status = StockStatus::Live;

        if bought_price > post_price {
            post_price = bought_price + minimum_profit;
        }

        // If the item is worth less than moving average the set the post price to be the moving average
        if post_price < (moving_avg - min_sma) as i64 {
            post_price = moving_avg;
            stock_item.status = StockStatus::SMALimit;
        }

        // If minimum price is set and the post price is less than the minimum price then set the post price to be the minimum price
        if minimum_price.is_some() && post_price < minimum_price.unwrap() as i64 {
            post_price = minimum_price.unwrap() as i64;
        }

        // Calculate the profit from the post price
        let profit = post_price - bought_price as i64;

        if profit <= 0 {
            stock_item.status = StockStatus::ToLowProfit;
            stock_item.list_price = None;
        } else {
            stock_item.list_price = Some(post_price);
        }

        // Get Order Info
        let mut order_info = match user_order.info {
            Some(order_info) => order_info,
            None => OrderDetails::default(),
        };
        // Update the order info with the current price history
        order_info.highest_price = highest_price;
        order_info.lowest_price = live_orders.lowest_price(OrderType::Buy);
        order_info.total_buyers = live_orders.sell_orders.len() as i64;
        order_info.orders = live_orders.sell_orders.clone();
        order_info.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));
        user_order.info = Some(order_info.clone());

        if user_order.id != "N/A" {
            // If the item is too cheap, delete the order
            if stock_item.status == StockStatus::ToLowProfit {
                // Send GUI Update.
                self.send_msg(
                    "low_profit_delete",
                    Some(json!({ "name": item_info.name.clone()})),
                );
                wfm.orders().delete(&user_order.id).await?;
                my_orders.delete_order_by_id(OrderType::Sell, &user_order.id);
                self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
                self.send_stock_update(UIOperationEvent::Delete, json!({"id": stock_item.id}));
            } else {
                wfm.orders()
                    .update(&user_order.id, post_price, quantity, user_order.visible)
                    .await?;
                if user_order.platinum != post_price {
                    user_order.platinum = post_price;
                    user_order.quantity = quantity;
                    user_order.operation = "Updated".to_string();
                    my_orders.update_order(user_order.clone());
                    self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
                }
            }
        } else if stock_item.status != StockStatus::ToLowProfit {
            // Send GUI Update.
            self.send_msg(
                "created",
                Some(
                    json!({ "name": item_info.name.clone(), "price": post_price, "profit": profit}),
                ),
            );
            match wfm
                .orders()
                .create(
                    &item_info.wfm_id,
                    "sell",
                    post_price,
                    quantity,
                    true,
                    stock_item.sub_type.clone(),
                )
                .await
            {
                Ok((rep, None)) => {
                    if &rep == "order_limit_reached" {
                        // Send GUI Update.
                        self.send_msg(
                            "order_limit_reached",
                            Some(json!({ "name": item_info.name.clone()})),
                        );
                        stock_item.status = StockStatus::OrderLimit;
                        stock_item.list_price = None;
                    }
                }
                Ok((_, order)) => {
                    if order.is_some() {
                        let mut order = order.unwrap();
                        order.closed_avg = Some(moving_avg as f64);
                        order.profit = Some(profit as f64);
                        order.info = Some(order_info.clone());
                        my_orders.sell_orders.push(order.clone());
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(order));
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Get Stock Information.
        let stock_info = self.stock_info.get(&stock_item.id);
        let details = StockItemDetails::new(
            Some(live_orders.sell_orders.len() as i64),
            Some(profit),
            Some(lowest_price),
            Some(moving_avg),
            Some(highest_price),
            Some(live_orders.sell_orders),
        );

        // Need UI Update
        let mut need_ui_update = false;

        if stock_info.is_some() {
            let stock_info = stock_info.unwrap();
            if stock_info.total_sellers != details.total_sellers {
                need_ui_update = true;
            } else if stock_info.profit != details.profit {
                need_ui_update = true;
            } else if stock_info.lowest_price != details.lowest_price {
                need_ui_update = true;
            } else if stock_info.highest_price != details.highest_price {
                need_ui_update = true;
            } else if stock_info.orders.is_some()
                && details.orders.is_some()
                && (stock_info.orders.clone().unwrap().len()
                    != details.orders.clone().unwrap().len())
            {
                need_ui_update = true;
            }
        } else {
            need_ui_update = true;
        }

        if stock_item.list_price != stock_item_original.list_price {
            // Create a PriceHistory struct
            if stock_item.list_price.is_some() {
                let price_history = PriceHistory::new(
                    chrono::Local::now().naive_local().to_string(),
                    stock_item.list_price.unwrap(),
                );
                let last_price_history = stock_item_original.price_history.0.last();
                if last_price_history.is_none()
                    || last_price_history.unwrap().price != stock_item.list_price.unwrap()
                {
                    stock_item.price_history.0.push(price_history.clone());
                }
            }
            need_ui_update = true;
        } else if stock_item.status != stock_item_original.status {
            need_ui_update = true;
        } else if stock_item.price_history.0.len() != stock_item_original.price_history.0.len() {
            need_ui_update = true;
        }

        if need_ui_update {
            StockItemMutation::update_by_id(&app.conn, stock_item.id, stock_item.clone())
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            let mut payload = json!(stock_item);
            payload["info"] = json!(details);
            self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(payload));
        }
        return Ok(());
    }
}
