use crate::cache::types::cache_tradable_item::CacheTradableItem;
use crate::cache::types::item_price_info::ItemPriceInfo;
use crate::enums::trade_mode::TradeMode;
use crate::live_scraper::client::LiveScraperClient;

use crate::live_scraper::types::item_entry::ItemEntry;
use crate::live_scraper::types::order_extra_info::OrderDetails;
use crate::utils::enums::log_level::LogLevel;
use crate::utils::enums::ui_events::{UIEvent, UIOperationEvent};
use crate::utils::modules::error::{self, AppError};
use crate::utils::modules::logger;
use crate::wfm_client::enums::order_type::OrderType;
use crate::wfm_client::types::item;
use crate::wfm_client::types::order::Order;
use crate::wfm_client::types::orders::Orders;
use entity::enums::stock_status::StockStatus;
use entity::price_history::{PriceHistory, PriceHistoryVec};

use serde_json::json;
use service::{StockItemMutation, StockItemQuery, WishListMutation, WishListQuery};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec;
#[derive(Clone)]
pub struct ItemModule {
    pub client: LiveScraperClient,
    component: String,
    info_caches: HashMap<String, OrderDetails>,
    interesting_items_cache: Arc<Mutex<HashMap<String, Vec<ItemEntry>>>>,
}

impl ItemModule {
    pub fn new(client: LiveScraperClient) -> Self {
        ItemModule {
            client,
            component: "Item".to_string(),
            interesting_items_cache: Arc::new(Mutex::new(HashMap::new())),
            info_caches: HashMap::new(),
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
    pub fn send_wish_list_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateWishList, operation, Some(value));
    }
    pub fn send_order_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = self.client.notify.lock().unwrap().clone();
        notify
            .gui()
            .send_event_update(UIEvent::UpdateOrders, operation, Some(value));
    }

    pub fn reset(&mut self) {
        self.info_caches = HashMap::new();
        self.update_state();
    }

    pub async fn check_stock(&mut self) -> Result<(), AppError> {
        logger::info_con(&self.component, "Running Item Stock Check");

        // Load Managers.
        let app = self.client.app.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let auth = self.client.auth.lock()?.clone();
        let cache = self.client.cache.lock()?.clone();
        let settings = self.client.settings.lock()?.clone().live_scraper;

        // Send GUI Update.
        self.send_msg("stating", None);

        // Get Settings.
        let trade_mode = settings.trade_mode.clone();
        let blacklist_items: Vec<String> = settings.stock_item.blacklist.clone();

        // Variables.
        let mut interesting_items: HashMap<String, ItemEntry> = HashMap::new();

        // Get interesting items to buy from the price scraper if the order mode is buy or both.
        if trade_mode == TradeMode::Buy || trade_mode == TradeMode::All {
            let buy_list = self.get_interesting_items().await?;
            for item in buy_list {
                interesting_items.insert(item.wfm_url.clone(), item);
            }
        }

        // Get interesting items to sell from the stock items if the order mode is sell or both.
        if trade_mode == TradeMode::Sell || trade_mode == TradeMode::All {
            let sell_list = StockItemQuery::get_all_stock_items(&app.conn, 0)
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            for item in sell_list {
                // Use the entry API for modification or insertion
                interesting_items
                    .entry(item.wfm_url.clone())
                    .and_modify(|entry| {
                        entry.priority = 1;
                        entry.sell_quantity = item.owned;
                        entry.stock_id = Some(item.id);
                        entry.operation.push("Sell".to_string());
                    })
                    .or_insert_with(|| {
                        ItemEntry::new(
                            Some(item.id),
                            None,
                            item.wfm_url.clone(),
                            item.sub_type.clone(),
                            1,
                            0,
                            item.owned,
                            vec!["Sell".to_string()],
                            "closed",
                        )
                    });
            }
        }

        // Get Wishlist items to buy from the wishlist if the order mode is buy or both.
        if trade_mode == TradeMode::WishList || trade_mode == TradeMode::All {
            let wish_list = WishListQuery::get_all(&app.conn)
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            for item in wish_list {
                interesting_items
                    .entry(item.wfm_url.clone())
                    .and_modify(|entry| {
                        entry.priority = 2;
                        entry.buy_quantity = item.quantity;
                        entry.wish_list_id = Some(item.id);
                        entry.operation.push("WishList".to_string());
                    })
                    .or_insert_with(|| {
                        ItemEntry::new(
                            None,
                            Some(item.id),
                            item.wfm_url.clone(),
                            item.sub_type.clone(),
                            2,
                            item.quantity,
                            0,
                            vec!["WishList".to_string()],
                            "buy",
                        )
                    });
            }
        }

        // Get My Orders from Warframe Market.
        let mut my_orders = wfm.orders().get_my_orders().await?;

        // Handle Delete Orders based on the trade mode.
        let order_ids = if (trade_mode == TradeMode::Buy || trade_mode == TradeMode::WishList)
            && trade_mode != TradeMode::All
        {
            my_orders.get_orders_ids(OrderType::Sell, blacklist_items.clone())
        } else if trade_mode == TradeMode::Sell && trade_mode != TradeMode::All {
            my_orders.get_orders_ids(OrderType::Buy, blacklist_items.clone())
        } else {
            vec![]
        };
        for id in &order_ids {
            // Send GUI Update.
            // wfm.orders().delete(&id).await?;
            my_orders.delete_order_by_id(OrderType::All, &id);
            self.send_order_update(UIOperationEvent::Delete, json!({"id": id}));
        }

        // Apply Trade Info.
        my_orders.apply_trade_info(&cache)?;

        logger::log_json(
            "interesting_items.json",
            &json!({
                "order_ids": order_ids,
                "orders": my_orders,
                "settings": settings.stock_item,
                "interesting_items": interesting_items.clone(),
            }),
        )?; // Debugging
            // let mut interesting_items: HashSet<ItemEntry> = HashSet::from_iter(interesting_items);

        let mut current_index = interesting_items.len();
        logger::info_file(
            &self.get_component("CheckStock"),
            format!("Total interesting items: {}", current_index).as_str(),
            Some(self.client.log_file.as_str()),
        );

        // Create a cache for the orders.
        let mut orders_cache: HashMap<String, Orders> = HashMap::new();

        // // Sort the interesting items by the priority.
        let mut interesting_items: Vec<ItemEntry> = interesting_items.into_values().collect();
        interesting_items.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Loop through all interesting items
        for item_entry in interesting_items.clone() {
            if auth.qf_banned || auth.wfm_banned || auth.anonymous {
                self.client.stop_loop();
                break;
            }

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
                let orders = wfm
                    .orders()
                    .get_orders_by_item(&item_entry.wfm_url, item_entry.sub_type.as_ref(), false)
                    .await?;
                orders_cache.insert(item_entry.wfm_url.clone(), orders.clone());
                orders
            };

            // Filter the live orders by the username
            live_orders = live_orders.filter_by_username(&auth.ingame_name, true);
            live_orders.sort_by_platinum();

            // Check if item_orders_df is empty and skip if it is
            if live_orders.total_count() == 0 {
                logger::warning_con(
                    &self.get_component("CheckStock"),
                    format!("Item {} has no orders. Skipping.", item_info.name).as_str(),
                );
                // Send GUI Update.
                self.send_msg("no_data", Some(json!({ "current": current_index, "total": interesting_items.len(), "name": item_info.name.clone()})));
                continue;
            }

            // Get the item stats from the price scraper
            let price = match cache.item_price().get_item_price(
                &item_entry.wfm_url,
                item_entry.sub_type.clone(),
                &item_entry.order_type,
            ) {
                Ok(p) => p,
                Err(_) => ItemPriceInfo::default(),
            };

            // Only check if the order mode is buy or both and if the item is in stock items
            if item_entry.operation.contains(&"Buy".to_string())
                && !item_entry.operation.contains(&"WishList".to_string())
            {
                match self
                    .progress_buying(
                        &item_info,
                        &item_entry,
                        &price,
                        &mut my_orders,
                        live_orders.clone(),
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
            // Only check if the order mode is buy or both and if the item is in stock items
            if item_entry.operation.contains(&"WishList".to_string()) {
                match self
                    .progress_wish_list(
                        &item_info,
                        &item_entry,
                        &price,
                        &mut my_orders,
                        live_orders.clone(),
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
            if item_entry.operation.contains(&"Sell".to_string()) && item_entry.stock_id.is_some() {
                match self
                    .progress_selling(
                        &item_info,
                        &item_entry,
                        &price,
                        &mut my_orders,
                        live_orders.clone(),
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

    pub async fn delete_all_orders(&mut self, mode: TradeMode) -> Result<(), AppError> {
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

        if mode == TradeMode::Buy || mode == TradeMode::All {
            orders.append(&mut current_orders.buy_orders);
        }
        if mode == TradeMode::Sell || mode == TradeMode::All {
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
        self.reset();
        Ok(())
    }

    pub async fn get_interesting_items(&self) -> Result<Vec<ItemEntry>, AppError> {
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let cache = self.client.cache.lock()?.clone();
        let volume_threshold = settings.stock_item.volume_threshold;
        let range_threshold = settings.stock_item.range_threshold;
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let trading_tax_cap = settings.stock_item.trading_tax_cap;
        let price_shift_threshold = settings.stock_item.price_shift_threshold;
        let black_list = settings.stock_item.blacklist.clone();
        let buy_quantity = settings.stock_item.buy_quantity;

        // Create a query uuid.
        let query_id = format!(
            "Volume:{:?}Range:{:?}AvgPrice{:?}Tax{:?}PriceShift:{:?}BlackList:{:?}:StockMode:{:?}:BuyQuantity:{:?}",
            volume_threshold.clone(),
            range_threshold.clone(),
            avg_price_cap.clone(),
            trading_tax_cap.clone(),
            price_shift_threshold.clone(),
            black_list.clone(),
            settings.stock_mode.clone(),
            buy_quantity.clone()
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
        
        // Dynamic filter using closures
        let order_type_filter = |item: &ItemPriceInfo| item.order_type == "closed";
        let volume_filter = |item: &ItemPriceInfo| item.volume > volume_threshold as f64;
        let range_filter = |item: &ItemPriceInfo| item.range > range_threshold as f64;
        let avg_price_filter = |item: &ItemPriceInfo| item.avg_price <= avg_price_cap as f64;
        let week_price_shift_filter =
            |item: &ItemPriceInfo| item.week_price_shift >= price_shift_threshold as f64;
        let trading_tax_cap_filter =
            |item: &ItemPriceInfo| trading_tax_cap <= 0 || item.trading_tax < trading_tax_cap;
        let black_list_filter = |item: &ItemPriceInfo| !black_list.contains(&item.url_name);

        // Combine multiple filters dynamically
        let combined_filter = |item: &ItemPriceInfo| {
            order_type_filter(item)
                && volume_filter(item)
                && range_filter(item)
                && avg_price_filter(item)
                && week_price_shift_filter(item)
                && trading_tax_cap_filter(item)
                && black_list_filter(item)
        };
        
        let filtered_items = cache.item_price().get_by_filter(combined_filter)?;

        // Convert to ItemEntry vector
        let entries = filtered_items
            .iter()
            .map(|item| {
                ItemEntry::new(
                    None,
                    None,
                    item.url_name.clone(),
                    item.sub_type.clone(),
                    0,
                    buy_quantity,
                    0,
                    vec!["Buy".to_string()],
                    "closed",
                )
            })
            .collect::<Vec<ItemEntry>>();

        self.add_cache_queried(query_id, entries.clone());
        Ok(entries)
    }
    pub fn add_cache_queried(&self, key: String, df: Vec<ItemEntry>) {
        self.interesting_items_cache.lock().unwrap().insert(key, df);
        self.update_state();
    }

    pub fn get_cache_queried(&self, key: &str) -> Option<Vec<ItemEntry>> {
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
        max_weight: i64,
    ) -> Result<
        (
            Vec<(i64, f64, String, String)>,
            Vec<(i64, f64, String, String)>,
        ),
        AppError,
    > {
        let n = items.len();
        let mut dp = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let (_, value, _, _) = items[i - 1];
            dp[i] = value as i64;
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

        Ok((selected_items, unselected_items))
    }
    pub async fn progress_wish_list(
        &mut self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        my_orders: &mut Orders,
        live_orders: Orders,
    ) -> Result<Option<Vec<Order>>, AppError> {
        // Load Managers.
        let settings = self.client.settings.lock()?.clone().live_scraper;
        let wfm = self.client.wfm.lock()?.clone();
        let blacklist = settings.stock_item.blacklist.clone();
        let app = self.client.app.lock()?.clone();

        // Check if the item is in the blacklist and skip if it is
        if blacklist.contains(&item_info.wfm_url_name) {
            return Ok(None);
        }

        // Get the stock item from the stock item if it exists.
        let wish_list_item = match entry.wish_list_id {
            Some(stock_id) => match WishListQuery::get_by_id(&app.conn, stock_id).await {
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
        if wish_list_item.is_none() {
            return Ok(None);
        }
        let mut wish_list_item = wish_list_item.unwrap();

        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_url_name,
            OrderType::Buy,
            wish_list_item.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
            None => Order::default(),
        };

        // Get The highest buy order returns 0 if there are no buy orders.
        let highest_price = live_orders.highest_price(OrderType::Buy);

        // Set the post price to the highest price.
        let mut post_price = highest_price;

        // Get Maximum Price
        let maximum_price = wish_list_item.maximum_price.unwrap_or(0);

        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            post_price = price.avg_price as i64;
        }

        // Check if the price is higher than the max price
        if post_price > maximum_price && maximum_price > 0 {
            post_price = maximum_price;
        }

        // Create a cache id for the order info.
        let cache_id = format!("WishList:{}", wish_list_item.id.clone());
        // Get/Create Order Info
        let price_history =
            PriceHistory::new(chrono::Local::now().naive_local().to_string(), post_price);

        let mut info = match self.info_caches.get_mut(&cache_id) {
            Some(order_info) => {
                // Update the order info with the current price history
                order_info.set_highest_price(highest_price);
                order_info.set_quantity(entry.buy_quantity);
                order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
                order_info.set_total_buyers(live_orders.buy_orders.len() as i64);
                order_info.set_orders(live_orders.buy_orders.clone());
                order_info.add_price_history(price_history.clone());
                user_order.operation.push("Updated".to_string());
                order_info.clone()
            }
            None => {
                let order_info = OrderDetails::new(
                    cache_id.clone(),
                    live_orders.buy_orders.len() as i64,
                    live_orders.sell_orders.len() as i64,
                    0,
                    live_orders.lowest_price(OrderType::Buy),
                    highest_price,
                    0,
                    live_orders.buy_orders.clone(),
                    entry.buy_quantity.clone(),
                    0,
                    vec![price_history.clone()],
                );
                if user_order.id != "N/A" {
                    user_order.operation.push("Updated".to_string());
                } else {
                    user_order.operation.push("Created".to_string());
                }
                order_info
            }
        };

        if user_order.operation.contains(&"Created".to_string()) {
            // Send GUI Update.
            self.send_msg(
                "created",
                Some(json!({ "name": item_info.name, "price": post_price, "profit": 0})),
            );
            match wfm
                .orders()
                .create(
                    &item_info.wfm_id,
                    "buy",
                    post_price,
                    entry.buy_quantity,
                    true,
                    entry.sub_type.clone(),
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
                    }
                }
                Ok((_, Some(mut order))) => {
                    order.info = user_order.info.clone();
                    order.operation = user_order.operation.clone();
                    order.tags = vec!["WishList".to_string()];
                    my_orders.buy_orders.push(order.clone());
                    order.info.is_dirty = false;
                    wish_list_item.set_status(StockStatus::Live);
                    wish_list_item.set_list_price(Some(post_price));
                    self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(order.clone()));
                    self.info_caches
                        .insert(info.cache_id.clone(), order.info.clone());
                    self.update_state();
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
            };
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} Created", item_info.name).as_str(),
            );
        } else if user_order.operation.contains(&"Updated".to_string()) {
            match wfm
                .orders()
                .update(
                    &user_order.id,
                    post_price,
                    entry.buy_quantity,
                    user_order.visible,
                )
                .await
            {
                Ok(_) => {
                    wish_list_item.set_status(StockStatus::Live);
                    wish_list_item.set_list_price(Some(post_price));
                    if user_order.platinum != post_price || user_order.info.is_dirty {
                        user_order.platinum = post_price;
                        user_order.tags = vec!["WishList".to_string()];
                        my_orders.update_order(user_order.clone());
                        user_order.info.is_dirty = false;
                        user_order.info.changes = None;
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
                        self.update_state();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        wish_list_item.price_history =
            PriceHistoryVec(info.price_history.clone().into_iter().collect());

        if info.is_dirty || wish_list_item.is_dirty {
            WishListMutation::update_by_id(&app.conn, wish_list_item.id, wish_list_item.clone())
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            let mut payload = json!(wish_list_item);
            payload["info"] = json!(info);
            info.is_dirty = false;
            info.changes = None;
            if self.info_caches.contains_key(&cache_id) {
                *self.info_caches.get_mut(&cache_id).unwrap() = info.clone();
            } else {
                self.info_caches.insert(cache_id.clone(), info.clone());
            }
            self.update_state();
            self.send_wish_list_update(UIOperationEvent::CreateOrUpdate, json!(payload));
        }
        Ok(None)
    }

    pub async fn progress_buying(
        &mut self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        my_orders: &mut Orders,
        live_orders: Orders,
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
        let min_range_threshold = settings.stock_item.range_threshold;
        let closed_avg = price.moving_avg.unwrap_or(0.0);

        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_url_name,
            OrderType::Buy,
            entry.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
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

        // Get the price_range of the item highest_price - lowest_price
        let price_range = live_orders.get_price_range(OrderType::Buy);

        // Set the post price to the highest price.
        let mut post_price = highest_price;

        // If there are no buyers, and the average price is greater than 25p, then we should probably update/create our listing.
        if post_price == 0 && closed_avg > 25.0 {
            // Calculate the post price
            // The post price is the maximum of two calculated values:
            // 1. The price range minus 40
            // 2. One third of the price range minus 1
            post_price = (price_range - 40).max((&price_range / 3) - 1);
        }

        // Get the average price of the item from the Warframe Market API
        let closed_avg_metric = (closed_avg - post_price as f64) as i64;

        // Get the potential profit from the average price of the item from the Warframe Market API
        let potential_profit = closed_avg_metric - 1;

        // Check if the post price is greater than the average price cap and set the status to overpriced if it is.
        if post_price > avg_price_cap as i64 {
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} is overpriced, base of your average price cap of {} and the current price is {}", item_info.name, avg_price_cap, post_price).as_str(),
            );
            user_order.operation.push("Deleted".to_string());
        }

        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            return Ok(None);
        }

        // Create a cache id for the order info.
        let cache_id = format!("Buying:{}", user_order.id.clone());

        // Get/Create Order Info
        let price_history =
            PriceHistory::new(chrono::Local::now().naive_local().to_string(), post_price);
        let info = match self.info_caches.get_mut(&cache_id) {
            Some(order_info) => {
                // Update the order info with the current price history
                order_info.set_highest_price(highest_price);
                // order_info.set_quantity(item.quantity);
                order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
                order_info.set_range(price_range);
                order_info.set_total_buyers(live_orders.buy_orders.len() as i64);
                order_info.set_orders(live_orders.buy_orders.clone());
                order_info.set_moving_avg(closed_avg as i64);
                order_info.add_price_history(price_history.clone());
                user_order.operation.push("Updated".to_string());
                order_info.clone()
            }
            None => {
                let order_info = OrderDetails::new(
                    cache_id,
                    live_orders.buy_orders.len() as i64,
                    live_orders.sell_orders.len() as i64,
                    potential_profit,
                    live_orders.lowest_price(OrderType::Buy),
                    highest_price,
                    closed_avg as i64,
                    live_orders.sell_orders.clone(),
                    entry.buy_quantity.clone(),
                    price_range,
                    vec![price_history.clone()],
                );
                if user_order.id != "N/A" {
                    user_order.operation.push("Updated".to_string());
                } else {
                    user_order.operation.push("Created".to_string());
                }
                order_info
            }
        };

        let mut buy_orders_list: Vec<(i64, f64, String, String)> = vec![];

        if closed_avg_metric >= 0 && price_range >= min_range_threshold {
            user_order
                .operation
                .push("ValidatedMaxPriceCap".to_string());
            if user_order.operation.contains(&"Created".to_string()) {
                if my_orders.buy_orders.len() != 0 {
                    buy_orders_list = my_orders
                        .buy_orders
                        .iter()
                        .filter(|order| !order.tags.contains(&"WishList".to_string()))
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
                let (selected_buy_orders, unselected_buy_orders) =
                    self.knapsack(buy_orders_list.clone(), max_total_price_cap)?;

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
                            match wfm.orders().delete(&unselected_item.3).await {
                                Ok(_) => {
                                    // Send GUI Update.
                                    self.send_msg(
                                        "knapsack_delete",
                                        Some(json!({ "name": item_info.name})),
                                    );
                                    self.send_order_update(
                                        UIOperationEvent::Delete,
                                        json!({"id": unselected_item.3}),
                                    );
                                    if user_order.id == unselected_item.3 {
                                        user_order.operation = vec!["Skip".to_string()];
                                    }
                                    my_orders
                                        .delete_order_by_id(OrderType::Buy, &unselected_item.3);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                    }
                } else {
                    user_order.operation = vec!["NotOptimal".to_string()];
                }
            }
        } else if user_order.operation.contains(&"Updated".to_string()) {
            user_order.operation.push("Deleted".to_string());
        } else if price_range <= min_range_threshold {
            user_order.operation = vec!["NotInRange".to_string()];
        } else {
            user_order.operation = vec!["NotProfitable".to_string()];
        }

        if user_order.operation.contains(&"Created".to_string())
            && !user_order.operation.contains(&"Deleted".to_string())
        {
            // Send GUI Update.
            self.send_msg("created", Some(json!({ "name": item_info.name, "price": post_price, "profit": potential_profit})));
            match wfm
                .orders()
                .create(
                    &item_info.wfm_id,
                    "buy",
                    post_price,
                    entry.buy_quantity,
                    true,
                    entry.sub_type.clone(),
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
                    }
                }
                Ok((_, Some(mut order))) => {
                    order.info = info.clone();
                    order.operation = user_order.operation.clone();
                    order.profit = Some(potential_profit as f64);
                    order.closed_avg = Some(closed_avg);
                    my_orders.buy_orders.push(order.clone());
                    order.info.is_dirty = false;
                    self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(order.clone()));
                    self.info_caches
                        .insert(info.cache_id.replace("N/A", &order.id), info.clone());
                    self.update_state();
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
            };
            logger::info_con(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} Created", item_info.name).as_str(),
            );
        } else if user_order.operation.contains(&"Updated".to_string())
            && !user_order.operation.contains(&"Deleted".to_string())
        {
            match wfm
                .orders()
                .update(&user_order.id, post_price, 1, user_order.visible)
                .await
            {
                Ok(_) => {
                    if user_order.platinum != post_price || user_order.info.is_dirty {
                        user_order.platinum = post_price;
                        user_order.profit = Some(potential_profit as f64);
                        my_orders.update_order(user_order.clone());
                        user_order.info.is_dirty = false;
                        user_order.info.changes = None;
                        if self.info_caches.contains_key(&info.cache_id) {
                            *self.info_caches.get_mut(&info.cache_id).unwrap() = info.clone();
                        } else {
                            self.info_caches
                                .insert(info.cache_id.replace("N/A", &user_order.id), info.clone());
                        }
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
                        self.update_state();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if user_order.operation.contains(&"Updated".to_string())
            && user_order.operation.contains(&"Deleted".to_string())
        {
            match wfm.orders().delete(&user_order.id).await {
                Ok(_) => {
                    self.info_caches.remove(&info.cache_id);
                    my_orders.delete_order_by_id(OrderType::Buy, &user_order.id);
                    self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
                    self.update_state();
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
            }
        }

        Ok(None)
    }

    async fn progress_selling(
        &mut self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        my_orders: &mut Orders,
        live_orders: Orders,
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
        let moving_avg = price.moving_avg.unwrap_or(0.0) as i64;

        // Get the stock item from the stock item if it exists.
        let stock_item = match entry.stock_id {
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
        if stock_item.is_none() {
            return Ok(());
        }
        let mut stock_item = stock_item.unwrap();

        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_url_name,
            OrderType::Sell,
            stock_item.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
            None => Order::default(),
        };

        // If the order is visible and the item is hidden, delete the order.
        if stock_item.is_hidden {
            stock_item.set_status(StockStatus::InActive);
            stock_item.set_list_price(None);
            if user_order.visible {
                wfm.orders().delete(&user_order.id).await?;
                my_orders.delete_order_by_id(OrderType::Sell, &user_order.id);
                self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
            }

            // Send GUI Update.
            self.send_msg("is_hidden", Some(json!({ "name": item_info.name.clone()})));
            if stock_item.is_dirty {
                StockItemMutation::update_by_id(&app.conn, stock_item.id, stock_item.clone())
                    .await
                    .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
                self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(stock_item));
            }
            return Ok(());
        }

        // Get the price the item was bought for.
        let bought_price = stock_item.bought as i64;

        // Get the quantity of owned item.
        let quantity = entry.sell_quantity;

        // Get the minimum price of the item.
        let minimum_price = stock_item.minimum_price;

        // Get the lowest sell order price from the DataFrame of live sell orders
        let lowest_price = if live_orders.sell_orders.len() > 2 {
            live_orders.lowest_price(OrderType::Sell)
        } else {
            stock_item.set_status(StockStatus::NoSellers);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            0
        };

        // Get the highest sell order price from the DataFrame of live sell orders
        let highest_price = live_orders.highest_price(OrderType::Sell);

        // Then Price the order will be posted for.
        let mut post_price = lowest_price;

        // If the item is worth less than moving average the set the post price to be the moving average
        if post_price < (moving_avg - min_sma) {
            post_price = moving_avg;
            user_order.operation.push("SMALimit".to_string());
        }

        // If minimum price is set and the post price is less than the minimum price then set the post price to be the minimum price
        if minimum_price.is_some() && post_price < minimum_price.unwrap() {
            post_price = minimum_price.unwrap();
        }

        // Calculate the profit from the post price
        let profit = post_price - bought_price;

        if profit < minimum_profit {
            user_order.operation.push("LowProfit".to_string());
        }

        // Create a cache id for the order info.
        let cache_id = format!("Selling:{}", stock_item.id);

        // Get/Create Order Info
        let price_history =
            PriceHistory::new(chrono::Local::now().naive_local().to_string(), post_price);
        let mut info = match self.info_caches.get_mut(&cache_id) {
            Some(order_info) => {
                // Update the order info with the current price history
                order_info.set_highest_price(highest_price);
                order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
                order_info.set_total_sellers(live_orders.sell_orders.len() as i64);
                order_info.set_orders(live_orders.sell_orders.clone());
                order_info.set_moving_avg(moving_avg);
                order_info.add_price_history(price_history.clone());
                user_order.operation.push("Updated".to_string());
                order_info.clone()
            }
            None => {
                let order_info = OrderDetails::new(
                    cache_id.clone(),
                    live_orders.buy_orders.len() as i64,
                    live_orders.sell_orders.len() as i64,
                    profit,
                    live_orders.lowest_price(OrderType::Buy),
                    highest_price,
                    moving_avg,
                    live_orders.sell_orders.clone(),
                    stock_item.owned,
                    0,
                    vec![price_history.clone()],
                );
                if user_order.id != "N/A" {
                    user_order.operation.push("Updated".to_string());
                } else {
                    user_order.operation.push("Created".to_string());
                }
                order_info
            }
        };

        // Update/Create/Delete the order on Warframe Market API and update the UI if needed.
        if user_order.operation.contains(&"Created".to_string())
            && !user_order.operation.contains(&"LowProfit".to_string())
        {
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
                        stock_item.set_status(StockStatus::OrderLimit);
                    }
                }
                Ok((_, order)) => {
                    if order.is_some() {
                        let mut order = order.unwrap();
                        order.info = user_order.info.clone();
                        order.operation = user_order.operation.clone();
                        order.closed_avg = Some(moving_avg as f64);
                        order.profit = Some(profit as f64);
                        user_order = order.clone();
                        my_orders.sell_orders.push(order.clone());
                        order.info.is_dirty = false;
                        if user_order.operation.contains(&"SMALimit".to_string()) {
                            stock_item.set_status(StockStatus::SMALimit);
                        } else {
                            stock_item.set_status(StockStatus::Live);
                        }
                        stock_item.set_list_price(Some(post_price));
                        self.send_order_update(
                            UIOperationEvent::CreateOrUpdate,
                            json!(order.clone()),
                        );
                        self.update_state();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if user_order.operation.contains(&"LowProfit".to_string()) {
            stock_item.set_status(StockStatus::ToLowProfit);
            stock_item.set_list_price(None);
            if user_order.id != "N/A" {
                match wfm.orders().delete(&user_order.id).await {
                    Ok(_) => {
                        stock_item.set_status(StockStatus::ToLowProfit);
                        stock_item.set_list_price(None);
                        my_orders.delete_order_by_id(OrderType::Sell, &user_order.id);
                        self.send_order_update(
                            UIOperationEvent::Delete,
                            json!({"id": user_order.id}),
                        );
                        self.update_state();
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        } else if user_order.operation.contains(&"Updated".to_string()) {
            match wfm
                .orders()
                .update(&user_order.id, post_price, quantity, user_order.visible)
                .await
            {
                Ok(_) => {
                    if user_order.operation.contains(&"SMALimit".to_string()) {
                        stock_item.set_status(StockStatus::SMALimit);
                    } else {
                        stock_item.set_status(StockStatus::Live);
                    }
                    stock_item.set_list_price(Some(post_price));
                    if user_order.platinum != post_price || user_order.info.is_dirty {
                        user_order.platinum = post_price;
                        user_order.quantity = quantity;
                        user_order.profit = Some(profit as f64);
                        my_orders.update_order(user_order.clone());
                        user_order.info.is_dirty = false;
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
                        self.update_state();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Update/Create/Delete the stock item on the database and update the UI if needed.

        stock_item.price_history =
            PriceHistoryVec(info.price_history.clone().into_iter().collect());

        if info.is_dirty || stock_item.is_dirty {
            StockItemMutation::update_by_id(&app.conn, stock_item.id, stock_item.clone())
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            let mut payload = json!(stock_item);
            payload["info"] = json!(info);
            info.is_dirty = false;
            info.changes = None;
            if self.info_caches.contains_key(&cache_id) {
                *self.info_caches.get_mut(&cache_id).unwrap() = info.clone();
            } else {
                self.info_caches.insert(cache_id.clone(), info.clone());
            }
            self.update_state();
            self.send_stock_update(UIOperationEvent::CreateOrUpdate, json!(payload));
        }

        return Ok(());
    }
}
