use std::{
    collections::HashSet,
    sync::{atomic::Ordering, Arc, Weak},
};

use entity::{dto::PriceHistory, enums::stock_status::StockStatus};
use serde_json::json;
use service::{StockItemMutation, WishListMutation};
use utils::*;
use wf_market::{
    enums::{OrderType, StatusType},
    errors::ApiError,
    types::{Order, OrderList, OrderWithUser},
};

use crate::{
    app::{client::AppState, Settings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    utils::{ErrorFromExt, OrderListExt},
};
use crate::{
    enums::TradeMode, live_scraper::*, send_event, types::*, utils::modules::states,
    utils::SubTypeExt, DATABASE,
};

static COMPONENT: &str = "LiveScraper:Item:";
static LOG_FILE: &str = "live_scraper_item.log";

#[derive(Debug)]
pub struct ItemModule {
    client: Weak<LiveScraperState>,
}

impl ItemModule {
    /**
     * Creates a new `ItemModule` with an empty item list.
     * The `client` parameter is an `Arc<LiveScraperState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
<<<<<<< HEAD
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
    pub fn send_stock_update(&self) {
        let notify = states::notify_client().expect("Failed to get notify client");
        notify.gui().send_event(UIEvent::RefreshStockItems, None);
    }
    pub fn send_wish_list_update(&self) {
        let notify = states::notify_client().expect("Failed to get notify client");
        notify.gui().send_event(UIEvent::RefreshWishListItems, None);
    }
    pub fn send_order_update(&self, operation: UIOperationEvent, value: serde_json::Value) {
        let notify = states::notify_client().expect("Failed to get notify client");
        notify
            .gui()
            .send_event_update(UIEvent::UpdateOrders, operation, Some(value));
    }

    pub async fn check_stock(&mut self, my_orders: &mut Orders) -> Result<(), AppError> {
        logger::info(
            &self.component,
            "Running Item Stock Check",
            LoggerOptions::default(),
=======
    fn send_event(&self, i18nKey: impl Into<String>, values: Option<serde_json::Value>) {
        send_event!(
            UIEvent::SendLiveScraperMessage,
            json!({"i18nKey": format!("item.{}", i18nKey.into()), "values": values})
>>>>>>> better-backend
        );
    }

    async fn delete_unwanted_orders(
        &self,
        app: &AppState,
        settings: &Settings,
        my_orders: &OrderList<Order>,
    ) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Failed to upgrade client");
        if !settings.live_scraper.should_delete_other_types && !settings.live_scraper.auto_delete {
            return Ok(()); // Nothing to delete
        }

<<<<<<< HEAD
        // Get interesting items to sell from the stock items if the order mode is sell or both.
        if settings_state.has_trade_mode(TradeMode::Sell) {
            let sell_list = StockItemQuery::get_all_stock_items(conn, 0)
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            for item in sell_list {
                // Use the entry API for modification or insertion
                interesting_items
                    .entry(item.uuid())
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
                            item.wfm_id.clone(),
                            item.sub_type.clone(),
                            1,
                            0,
                            item.owned,
                            vec!["Sell".to_string()],
                            "closed",
                        )
                    });
=======
        // Determine which order IDs to delete
        let order_ids = match (
            settings.live_scraper.has_trade_mode(TradeMode::Buy),
            settings.live_scraper.has_trade_mode(TradeMode::Sell),
            settings.live_scraper.has_trade_mode(TradeMode::WishList),
            settings.live_scraper.auto_delete,
            client.just_started.load(Ordering::SeqCst),
        ) {
            // If auto delete is enabled → delete all orders if just started
            (_, _, _, true, true) => my_orders
                .order_ids(OrderType::Buy)
                .into_iter()
                .chain(my_orders.order_ids(OrderType::Sell))
                .collect(),
            // Buy + Wishlist mode → delete Sell orders
            (true, false, true, false, _) => my_orders.order_ids(OrderType::Sell),
            // Sell only mode → delete Buy orders
            (false, true, false, false, _) => my_orders.order_ids(OrderType::Buy),
            // Everything else → delete nothing
            _ => vec![],
        };

        // Delete each unwanted order
        let mut current_index = order_ids.len();
        let total = order_ids.len();
        for id in order_ids.iter() {
            // Stop if client stopped running or user is banned
            if !client.is_running.load(Ordering::SeqCst) || app.user.is_banned() {
                warning(
                    format!("{}Delete", COMPONENT),
                    "Live Scraper is not running or user is banned, stopping deletion.",
                    &&LoggerOptions::default(),
                );
                break;
>>>>>>> better-backend
            }
            match app.wfm_client.order().delete(id).await {
                Ok(_) => {
                    info(
                        format!("{}Delete", COMPONENT),
                        &format!("Deleted order with ID: {} {}/{}", id, current_index, total),
                        &&LoggerOptions::default(),
                    );
                    self.send_event(
                        "deleted",
                        Some(json!({
                            "current": current_index,
                            "total": total,
                            "id": id
                        })),
                    );
                }
                Err(e) => error(
                    format!("{}Delete", COMPONENT),
                    &format!("Failed to delete order with ID {}: {}", id, e),
                    &&LoggerOptions::default().set_file(LOG_FILE),
                ),
            }
            current_index -= 1;
        }

<<<<<<< HEAD
        // Get Wishlist items to buy from the wishlist if the order mode is buy or both.
        if settings_state.has_trade_mode(TradeMode::WishList) {
            let wish_list = WishListQuery::get_all(conn)
                .await
                .map_err(|e| AppError::new(&self.component, eyre::eyre!(e)))?;
            for item in wish_list {
                interesting_items
                    .entry(item.uuid())
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
                            item.wfm_id.clone(),
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

        // Handle Delete Orders based on the trade mode.
        let order_ids = if delete_other_types {
            let buy = settings_state.has_trade_mode(TradeMode::Buy);
            let sell = settings_state.has_trade_mode(TradeMode::Sell);
            let wish = settings_state.has_trade_mode(TradeMode::WishList);
            // Delete orders that are not in the interesting items list.
            match (buy, sell, wish) {
                // Buy and Sell
                (true, false, true) => {
                    my_orders.get_orders_ids2(OrderType::Sell, blacklist_items.clone())
                }
                // Sell
                (false, true, false) => {
                    my_orders.get_orders_ids2(OrderType::Buy, blacklist_items.clone())
                }
                _ => vec![],
            }
        } else {
            vec![]
        };
        for id in &order_ids {
            // Send GUI Update.
            wfm.orders().delete(&id).await?;
            self.send_order_update(UIOperationEvent::Delete, json!({"id": id}));
        }
=======
        Ok(())
    }

    pub async fn check(&self) -> Result<(), Error> {
        let app = states::app_state()?;
        info(
            COMPONENT,
            "Checking Item items...",
            &&LoggerOptions::default(),
        );

        // Get My Orders from Warframe Market.
        let my_orders = app.wfm_client.order().cache_orders();

        // Delete unwanted orders
        self.delete_unwanted_orders(&app, &app.settings, &my_orders)
            .await?;
>>>>>>> better-backend

        // Collect interesting items
        let interesting_items = collect_interesting_items(COMPONENT, &app.settings).await?;

        // Process interesting items
        self.process_items(interesting_items, &app).await?;
        Ok(())
    }
    async fn process_items(
        &self,
        mut interesting_items: Vec<ItemEntry>,
        app: &AppState,
    ) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let client = self.client.upgrade().expect("Client should not be dropped");
        let mut current_index = 1;

        // Sort by priority (highest first)
        interesting_items.sort_by(|a, b| b.priority.cmp(&a.priority));
        let total = interesting_items.len();

<<<<<<< HEAD
        logger::log_json(
            "interesting_items.json",
            &json!({
                "ToDelete": order_ids,
                "settings": settings.stock_item,
                "interesting_items": interesting_items.clone(),
            }),
        )?;
        // Loop through all interesting items
        for item_entry in interesting_items.clone() {
            if auth.qf_banned || auth.wfm_banned {
                self.client.stop_loop();
=======
        for item_entry in interesting_items {
            // Stop if client stopped running or user is banned
            if !client.is_running.load(Ordering::SeqCst) || app.user.is_banned() {
                warning(
                    format!("{}ProcessItem", COMPONENT),
                    "Live Scraper is not running or user is banned, stopping processing.",
                    &&LoggerOptions::default(),
                );
>>>>>>> better-backend
                break;
            }

            // Get tradable item info from cache
            let item_info = match cache.tradable_item().get_by(&item_entry.wfm_url) {
                Ok(item) => item,
                Err(e) => {
                    e.set_component(format!("{}ProcessItem", COMPONENT))
                        .log(LOG_FILE);
                    continue;
                }
            };

            // Get item price from cache
            let item_price = cache
                .item_price()
                .find_by(&item_entry.wfm_url, item_entry.sub_type.clone())?
                .unwrap_or_default();

            // GUI event for progress
            self.send_event(
                "checking",
                Some(json!({
                    "current": current_index,
                    "total": total,
                    "name": item_info.name,
                    "sub_type": item_entry.sub_type,
                    "price": item_price
                })),
            );

            // Fetch live orders from API
            let mut orders = match app
                .wfm_client
                .order()
                .get_orders_by_item(&item_entry.wfm_url)
                .await
            {
                Ok(o) => o,
                Err(e) => {
                    let log_level = match e {
                        ApiError::RequestError(_) => LogLevel::Error,
                        _ => LogLevel::Critical,
                    };
                    return Err(Error::from_wfm(
                        format!("{}ProcessItem", COMPONENT),
                        &format!("Failed to get live orders for item {}", item_entry.wfm_url),
                        e,
                        get_location!(),
                    )
                    .set_log_level(log_level));
                }
            };

            // Apply filters to orders
            orders.filter_by_sub_type(
                wf_market::types::SubType::from_entity(item_entry.sub_type.clone()),
                false,
            );
            orders.filter_username(&app.user.wfm_username, true);
            orders.filter_user_status(StatusType::InGame, false);
            orders.sort_by_platinum();

            info(
                format!("{}ProcessItem", COMPONENT),
                &format!(
                    "Processing {}: {} buy orders, {} sell orders",
                    item_entry.uuid(),
                    orders.buy_orders.len(),
                    orders.sell_orders.len()
                ),
                &&LoggerOptions::default(),
            );

<<<<<<< HEAD
            // Get the item stats from the price scraper
            let price = match cache
                .item_price()
                .get_item_price2(&item_entry.wfm_id, item_entry.sub_type.clone())
            {
                Ok(p) => p,
                Err(_) => {
                    logger::warning(
                        &self.get_component("CheckStock"),
                        format!(
                            "Item Price Info for {} not found in cache",
                            item_info.name.clone()
                        )
                        .as_str(),
                        LoggerOptions::default(),
                    );
                    ItemPriceInfo::default()
                }
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
                        live_orders.clone(),
                        my_orders,
                    )
=======
            // Process buying logic
            if item_entry.operation.contains(&"Buy".to_string())
                && !item_entry.operation.contains(&"WishList".to_string())
            {
                if let Err(e) = self
                    .progress_buying(&item_info, &item_entry, &item_price, &orders)
>>>>>>> better-backend
                    .await
                {
                    return Err(e.with_location(get_location!()));
                }
<<<<<<< HEAD
            }
            // Only check if the order mode is buy or both and if the item is in stock items
            if item_entry.operation.contains(&"WishList".to_string()) {
                match self
                    .progress_wish_list(
                        &item_info,
                        &item_entry,
                        &price,
                        live_orders.clone(),
                        my_orders,
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
                        live_orders.clone(),
                        my_orders,
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

    pub async fn delete_all_orders(&mut self, modes: Vec<TradeMode>) -> Result<(), AppError> {
        let conn = DATABASE.get().unwrap();
        let wfm = states::wfm_client()?;
        let _notify = states::notify_client()?;
        let settings = states::settings()?.live_scraper;
        let blacklist = settings.stock_item.blacklist.clone();
        let mut current_orders = wfm.orders().get_my_orders().await?;

        match StockItemMutation::update_all(conn, StockStatus::Pending, None).await {
            Ok(_) => self.send_stock_update(),
            Err(e) => {
                error::create_log_file(
                    self.client.log_file,
                    &AppError::new(&self.component, eyre::eyre!(e)),
=======

                info(
                    format!("{}ProgressBuying", COMPONENT),
                    &format!(
                        "Successfully processed buying for item: {}",
                        item_entry.wfm_url
                    ),
                    &&LoggerOptions::default(),
>>>>>>> better-backend
                );
            }

<<<<<<< HEAD
        let mut orders = vec![];

        if modes.contains(&TradeMode::Buy) {
            orders.append(&mut current_orders.buy_orders);
        }
        if modes.contains(&TradeMode::Sell) {
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
            if blacklist.contains(&order.info.wfm_url) {
                continue;
            }
            match wfm.orders().delete(&order.id).await {
                Ok(_) => {
                    // Send GUI Update.
                    self.send_order_update(UIOperationEvent::Delete, json!({"id": order.id}));
                }
                Err(e) => {
                    error::create_log_file(self.client.log_file, &e);
                    logger::warning(
                        &self.get_component("DeleteAllOrders"),
                        format!("Error trying to delete order: {:?}", e).as_str(),
                        LoggerOptions::default(),
                    );
                }
            };
        }
        Ok(())
    }

    pub async fn get_interesting_items(&self) -> Result<Vec<ItemEntry>, AppError> {
        let settings = states::settings()?.live_scraper;
        let cache = states::cache()?;
        let volume_threshold = settings.stock_item.volume_threshold;
        let avg_price_cap = settings.stock_item.avg_price_cap;
        let trading_tax_cap = settings.stock_item.trading_tax_cap;
        let profit = settings.stock_item.range_threshold;
        let profit_margin = settings.stock_item.min_wtb_profit_margin;
        let price_shift_threshold = settings.stock_item.price_shift_threshold;
        let black_list = settings.stock_item.blacklist.clone();
        let buy_quantity = settings.stock_item.buy_quantity;

        // Create a query uuid.
        let query_id = format!(
            "Volume:{:?}Range:{:?}AvgPrice{:?}Tax{:?}PriceShift:{:?}BlackList:{:?}:StockMode:{:?}:BuyQuantity:{:?}:ProfitMargin:{:?}",
            volume_threshold.clone(),
            profit.clone(),
            avg_price_cap.clone(),
            trading_tax_cap.clone(),
            price_shift_threshold.clone(),
            black_list.clone(),
            settings.stock_mode.clone(),
            buy_quantity.clone(),
            profit_margin.clone()
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

        // Dynamic filter using closures

        let profit_margin_filter =
            |item: &ItemPriceInfo| profit_margin <= 0 || item.profit_margin >= profit_margin as f64;

        let volume_filter =
            |item: &ItemPriceInfo| volume_threshold <= 0 || item.volume > volume_threshold as f64;

        let profit_filter = |item: &ItemPriceInfo| profit <= 0 || item.profit > profit as f64;

        let avg_price_filter =
            |item: &ItemPriceInfo| avg_price_cap <= 0 || item.avg_price <= avg_price_cap as f64;

        let week_price_shift_filter =
            |item: &ItemPriceInfo| item.week_price_shift >= price_shift_threshold as f64;

        let trading_tax_cap_filter =
            |item: &ItemPriceInfo| trading_tax_cap <= 0 || item.trading_tax < trading_tax_cap;

        let black_list_filter = |item: &ItemPriceInfo| !black_list.contains(&item.wfm_url);

        // Combine multiple filters dynamically
        let combined_filter = |item: &ItemPriceInfo| {
            volume_filter(item)
                && profit_filter(item)
                && avg_price_filter(item)
                && week_price_shift_filter(item)
                && trading_tax_cap_filter(item)
                && black_list_filter(item)
                && profit_margin_filter(item)
        };

        let filtered_items = cache.item_price().get_by_filter(combined_filter);

        // Convert to ItemEntry vector
        let entries = filtered_items
            .iter()
            .map(|item| {
                ItemEntry::new(
                    None,
                    None,
                    item.wfm_url.clone(),
                    item.wfm_id.clone(),
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
        let w_max = max_weight as usize;

        // dp[w] = best value achievable with capacity w
        let mut dp = vec![0.0; w_max + 1];

        // choice[i][w] = true if item i is chosen when capacity is w
        let mut choice = vec![vec![false; w_max + 1]; n];

        for (i, item) in items.iter().enumerate() {
            let weight = item.0 as usize;
            let value = item.1;

            // iterate backwards for 1D DP
            for w in (weight..=w_max).rev() {
                let new_val = dp[w - weight] + value;
                if new_val > dp[w] {
                    dp[w] = new_val;
                    choice[i][w] = true;
                }
            }
        }

        // reconstruct chosen items
        let mut selected_items = Vec::new();
        let mut unselected_items = Vec::new();
        let mut w = w_max;

        for i in (0..n).rev() {
            let weight = items[i].0 as usize;
            if w >= weight && choice[i][w] {
                selected_items.push(items[i].clone());
                w -= weight;
            } else {
                unselected_items.push(items[i].clone());
            }
        }

        selected_items.reverse();
        unselected_items.reverse();

        Ok((selected_items, unselected_items))
    }
    pub async fn progress_wish_list(
        &mut self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        live_orders: Orders,
        my_orders: &mut Orders,
    ) -> Result<Option<Vec<Order>>, AppError> {
        // Load Managers.
        let conn = DATABASE.get().unwrap();
        let settings = states::settings()?.live_scraper;
        let wfm = states::wfm_client()?;
        let blacklist = settings.stock_item.blacklist.clone();

        // Check if the item is in the blacklist and skip if it is
        if blacklist.contains(&item_info.wfm_url_name) {
            return Ok(None);
        }

        // Get the stock item from the stock item if it exists.
        let wish_list_item = match entry.wish_list_id {
            Some(stock_id) => match WishListQuery::get_by_id(conn, stock_id).await {
                Ok(stock_item) => stock_item,
                Err(e) => {
                    error::create_log_file(
                        self.client.log_file,
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
            &entry.wfm_id,
            OrderType::Buy,
            entry.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
            None => Order::default(),
        };
        let per_trade = if item_info.bulk_tradable {
            Some(1)
        } else {
            None
        };
        // If the order is visible and the item is hidden, delete the order.
        if wish_list_item.is_hidden {
            wish_list_item.set_status(StockStatus::InActive);
            wish_list_item.set_list_price(None);
            if user_order.visible {
                wfm.orders().delete(&user_order.id).await?;
                self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
            }

            // Send GUI Update.
            self.send_msg("is_hidden", Some(json!({ "name": item_info.name.clone()})));
            if wish_list_item.is_dirty {
                WishListMutation::update_by_id(conn, wish_list_item.id, wish_list_item.clone())
=======
            // Process wishlist logic (future expansion)
            if item_entry.operation.contains(&"WishList".to_string()) {
                if let Err(e) = self
                    .progress_wish_list(&item_info, &item_entry, &item_price, &orders)
>>>>>>> better-backend
                    .await
                {
                    return Err(e.with_location(get_location!()));
                }

                info(
                    format!("{}ProgressWishList", COMPONENT),
                    &format!(
                        "Successfully processed wishlist for item: {}",
                        item_entry.wfm_url
                    ),
                    &&LoggerOptions::default(),
                );
            }

<<<<<<< HEAD
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

        post_price = std::cmp::max(post_price, 1);

        // Get/Create Order Info
        let price_history =
            PriceHistory::new(chrono::Local::now().naive_local().to_string(), post_price);

        // Update the order info with the current price history
        user_order.info.set_highest_price(highest_price);
        user_order
            .info
            .set_lowest_price(live_orders.lowest_price(OrderType::Buy));
        user_order.info.set_name(item_info.name.clone());
        user_order.info.set_image(item_info.image_url.clone());
        user_order
            .info
            .set_total_buyers(live_orders.buy_orders.len() as i64);
        user_order.info.set_orders(live_orders.buy_orders.clone());

        if user_order.id != "N/A" {
            user_order.operation.push("Updated".to_string());
        } else {
            user_order.operation.push("Created".to_string());
        }

        // Set the tags for the order info used for Buying.
        if !user_order.info.tags.contains(&"WishList".to_string()) {
            user_order.info.tags.push("WishList".to_string());
        }

        // Update Price History
        wish_list_item.add_price_history(price_history.clone());
        user_order.info.price_history = wish_list_item.price_history.0.clone().into();

        // Create/Update Order based on the operation.
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
                    per_trade,
                    entry.sub_type.clone(),
                    Some(user_order.info.clone()),
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
                Ok((_, Some(order))) => {
                    self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(order.clone()));
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
            };
            logger::info(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} Created", item_info.name).as_str(),
                LoggerOptions::default(),
            );
        } else if user_order.operation.contains(&"Updated".to_string()) {
            match wfm
                .orders()
                .update(
                    &user_order.id,
                    post_price,
                    entry.buy_quantity,
                    user_order.visible,
                    Some(user_order.info.clone()),
                )
                .await
            {
                Ok(_) => {
                    wish_list_item.set_status(StockStatus::Live);
                    wish_list_item.set_list_price(Some(post_price));
                    if user_order.info.is_dirty {
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
                    }
                }
                Err(e) => {
                    return Err(e);
=======
            // Process selling logic (future expansion)
            if item_entry.operation.contains(&"Sell".to_string()) && item_entry.stock_id.is_some() {
                if let Err(e) = self
                    .progress_selling(&item_info, &item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
>>>>>>> better-backend
                }

                info(
                    format!("{}ProgressBuying", COMPONENT),
                    &format!(
                        "Successfully processed buying for item: {}",
                        item_entry.wfm_url
                    ),
                    &&LoggerOptions::default(),
                );
            }

            current_index += 1;
        }

        Ok(())
    }

    pub async fn progress_buying(
        &self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
<<<<<<< HEAD
        live_orders: Orders,
        my_orders: &mut Orders,
    ) -> Result<Option<Vec<Order>>, AppError> {
        // Load Managers.
        let settings = states::settings()?.live_scraper;
        let wfm = states::wfm_client()?;
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
            &item_info.wfm_id,
            OrderType::Buy,
            entry.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
            None => Order::default(),
        };
        if user_order.id != "N/A" {
            user_order.operation.push("Updated".to_string());
        } else {
            user_order.operation.push("Created".to_string());
        }

        // Probably don't want to be looking at this item right now if there's literally nobody interested in selling it.
        if live_orders.sell_orders.len() <= 0 {
            logger::info(
                &self.get_component("CompareOrdersWhenBuying"),
                format!("Item {} has no sellers. Skipping.", item_info.name).as_str(),
                LoggerOptions::default(),
=======
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let log_options = &LoggerOptions::default()
            .set_show_component(false)
            .set_show_time(false);
        let component = format!("{}Buying:", COMPONENT);
        info(
            &component,
            &format!("Starting buying process for item: {}", item_info.name),
            &log_options
                .set_centered(true)
                .set_width(180)
                .set_enable(true),
        );
        let settings = states::get_settings()?.live_scraper.stock_item;

        // Check if item is blacklisted for buying
        if settings.is_item_blacklisted(&item_info.wfm_id, &TradeMode::Buy) {
            info(
                format!("{}Blacklisted", COMPONENT),
                &format!(
                    "Item {} is blacklisted for buying. Skipping.",
                    item_info.name
                ),
                &log_options,
>>>>>>> better-backend
            );
            return Ok(());
        }

<<<<<<< HEAD
        let per_trade = if item_info.bulk_tradable {
            Some(0)
=======
        let wfm_client = states::app_state()?.wfm_client;
        // Skip if no relevant market activity
        let (should_skip, _operation) = skip_if_no_market_activity(live_orders);
        if should_skip {
            return Ok(());
        }

        let avg_price_cap = settings.avg_price_cap;
        let max_total_price_cap = settings.max_total_price_cap; // currently unused
        let profit_threshold = settings.profit_threshold;
        let closed_avg = price.moving_avg.unwrap_or(0.0);
        let per_trade = if item_info.bulk_tradable {
            Some(settings.quantity_per_trade as u32)
>>>>>>> better-backend
        } else {
            None
        };

<<<<<<< HEAD
        // Get The highest buy order returns 0 if there are no buy orders.
=======
        let mut order_info = get_order_info(item_info, entry, &wfm_client, OrderType::Buy);

>>>>>>> better-backend
        let highest_price = live_orders.highest_price(OrderType::Buy);
        let price_range = live_orders.price_range(OrderType::Buy);

        // Determine post price
        let mut post_price = highest_price;
        if post_price == 0 && closed_avg > 25.0 {
            post_price = (price_range - 40).max(price_range / 3 - 1);
        }
        post_price = post_price.max(1);

        let closed_avg_metric = (closed_avg - post_price as f64) as i64;
        let potential_profit = closed_avg_metric - 1;

        // Check Max Buy Price for Item
        let item_max_price = settings.get_item_max_price(&item_info.wfm_id);
        if post_price as i64 > item_max_price && item_max_price > 0 {
            order_info.add_operation("AboveMaxBuyPrice");
            post_price = item_max_price;
            warning(
                format!("{}AboveMaxBuyPrice", component),
                &format!(
                    "Item {} post price {} is above max buy price {}.",
                    item_info.name, post_price, item_max_price
                ),
                &log_options,
            );
        }

        // Log overpriced warning
        if !is_disabled(avg_price_cap) && (post_price as i64) > avg_price_cap {
            order_info.add_operation("AboveAvgPrice");
            order_info.add_operation("Delete");
            warning(
                format!("{}OverpricedCheck", component),
                &format!("Item {} is above average price cap.", item_info.name),
                &log_options,
            );
        }

        // Update Order Info
        order_info = order_info.set_profit(price_range as f64);
        order_info = order_info.set_closed_avg(closed_avg);
        order_info = order_info.set_highest_price(highest_price);
        order_info = order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
        order_info.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));

<<<<<<< HEAD
        // Update the order info with the current price history
        user_order.info.set_highest_price(highest_price);
        user_order
            .info
            .set_lowest_price(live_orders.lowest_price(OrderType::Buy));
        user_order.info.set_range(price_range);
        user_order
            .info
            .set_total_buyers(live_orders.buy_orders.len() as i64);
        user_order.info.set_orders(live_orders.buy_orders.clone());
        user_order.info.set_moving_avg(closed_avg as i64);
        user_order.info.set_name(item_info.name.clone());
        user_order.info.set_image(item_info.image_url.clone());
        user_order.info.set_profit(potential_profit as f64);
        user_order.info.add_price_history(price_history.clone());

        // If the post price is not equal to the platinum price of the order, update it.
        if user_order.platinum != post_price {
            user_order.platinum = post_price;
            user_order.info.is_dirty = true;
        }

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
                        .filter(|order| !order.info.tags.contains(&"WishList".to_string()))
                        .map(|order| {
                            let platinum = order.platinum;
                            let profit = order.info.profit.unwrap();
                            let url_name = order.info.wfm_url.clone();
                            let id = order.id.clone();
                            (platinum, profit, url_name, id)
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
                            logger::warning(
                                &self.get_component("CompareOrdersWhenBuying"),
                                format!(
                                    "Item {} order id {} is unselected. Deleted order.",
                                    unselected_item.2.as_str(),
                                    unselected_item.3.as_str()
                                )
                                .as_str(),
                                LoggerOptions::default(),
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

        // Create/Update/Delete the order based on the operation.
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
                    per_trade,
                    entry.sub_type.clone(),
                    Some(user_order.info.clone()),
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
                    order.info.set_profit(potential_profit as f64);
                    my_orders.add_order(OrderType::Buy, order.clone());
                    self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(order.clone()));
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
=======
        if closed_avg_metric >= 0
            && price_range >= profit_threshold
            && order_info.has_operation("Create")
            && !wfm_client.order().cache_orders().buy_orders.is_empty()
            && !is_disabled(max_total_price_cap)
        {
            let buy_orders_list = {
                let mut list = wfm_client
                    .order()
                    .cache_orders()
                    .extract_order_summary(OrderType::Buy);
                list.push((
                    post_price,
                    potential_profit as f64,
                    item_info.wfm_id.clone(),
                    String::new(),
                ));
                list
>>>>>>> better-backend
            };

            let (selected_buy_orders, unselected_buy_orders) =
                knapsack(buy_orders_list.clone(), max_total_price_cap);

            info(
                format!("{}KnapsackResult", component),
                &format!(
                    "Selected {} buy orders out of {} for item {} based on knapsack algorithm.",
                    selected_buy_orders.len(),
                    buy_orders_list.len(),
                    item_info.name
                ),
                &log_options,
            );
<<<<<<< HEAD
        } else if user_order.operation.contains(&"Updated".to_string())
            && !user_order.operation.contains(&"Deleted".to_string())
        {
            match wfm
                .orders()
                .update(
                    &user_order.id,
                    post_price,
                    entry.buy_quantity,
                    user_order.visible,
                    Some(user_order.info.clone()),
                )
                .await
            {
                Ok(_) => {
                    if user_order.info.is_dirty {
                        user_order.info.set_profit(potential_profit as f64);
                        my_orders.add_order(OrderType::Buy, user_order.clone());
                        self.send_order_update(UIOperationEvent::CreateOrUpdate, json!(user_order));
=======

            let selected_ids: HashSet<_> = selected_buy_orders.iter().map(|o| &o.2).collect();

            if selected_ids.contains(&item_info.wfm_id) {
                for un_item in &unselected_buy_orders {
                    if let Err(e) = wfm_client.order().delete(&un_item.3).await {
                        error(
                            format!("{}KnapsackDeleteFail", component),
                            &format!("Failed to delete unselected item {}: {}", un_item.3, e),
                            &log_options,
                        );
                        continue;
                    }

                    info(
                        format!("{}KnapsackDeleteSuccess", component),
                        &format!("Deleted unselected item {}: {}", un_item.3, un_item.0),
                        &log_options,
                    );

                    if order_info.order_id == un_item.3 {
                        order_info.add_operation("Skip");
>>>>>>> better-backend
                    }
                }
            } else {
                info(
                    format!("{}KnapsackNotSelected", component),
                    &format!("Item {} not selected for buying.", item_info.name),
                    &log_options,
                );
                order_info.add_operation("Skip");
                order_info.add_operation("Delete");
            }
<<<<<<< HEAD
        } else if user_order.operation.contains(&"Updated".to_string())
            && user_order.operation.contains(&"Deleted".to_string())
        {
            match wfm.orders().delete(&user_order.id).await {
                Ok(_) => {
                    self.send_order_update(UIOperationEvent::Delete, json!({"id": user_order.id}));
                    logger::info(
                        &self.get_component("CompareOrdersWhenBuying"),
                        format!("Item {} Deleted", item_info.name).as_str(),
                        LoggerOptions::default(),
                    );
                    my_orders.delete_order_by_id(OrderType::Buy, &user_order.id);
                }
                Err(e) => {
                    self.client.stop_loop();
                    return Err(e);
                }
            }
        } else if user_order.operation.contains(&"NotInRange".to_string()) {
            logger::info(
                &self.get_component("ProgressBuying"),
                format!(
                    "Item {} is not in range. Skipping, Range: {}, Threshold: {}",
                    item_info.name, price_range, min_range_threshold
                )
                .as_str(),
                LoggerOptions::default(),
            );
        } else if user_order.operation.contains(&"NotOptimal".to_string()) {
            logger::info(
                &self.get_component("ProgressBuying"),
                format!("Item {} is not optimal. Skipping.", item_info.name).as_str(),
                LoggerOptions::default(),
            );
        } else {
            logger::info(
                &self.get_component("ProgressBuying"),
                format!("Item {} is not profitable. Skipping.", item_info.name).as_str(),
                LoggerOptions::default(),
            );
=======
        } else if closed_avg_metric < 0 && !is_disabled(max_total_price_cap) {
            order_info.add_operation("Delete");
            order_info.add_operation("Overpriced");
        } else if price_range < profit_threshold && !is_disabled(max_total_price_cap) {
            order_info.add_operation("Delete");
            order_info.add_operation("Underpriced");
>>>>>>> better-backend
        }

        // Summary log
        info(
            format!("{}Summary", component),
            format!(
                "Item {}: PostPrice: {} | ClosedAvg: {} | PriceRange: {} | PotentialProfit: {} | ClosedAvgMetric: {} | ProfitThreshold: {} | HighestPrice: {} | {}",
                item_info.name, post_price, closed_avg, price_range, potential_profit, closed_avg_metric, profit_threshold, highest_price, order_info
            ),
            &log_options,
        );

        // Create/Update/Delete
        match progress_order(
            &component,
            &wfm_client,
            &order_info,
            OrderType::Buy,
            post_price as u32,
            per_trade,
            log_options,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .with_location(get_location!())
                    .with_context(entry.to_json()))
            }
        }
        Ok(())
    }

    pub async fn progress_selling(
        &self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
<<<<<<< HEAD
        live_orders: Orders,
        my_orders: &mut Orders,
    ) -> Result<(), AppError> {
        // Load Managers.
=======
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
>>>>>>> better-backend
        let conn = DATABASE.get().unwrap();
        let log_options = &LoggerOptions::default();
        let component = format!("{}Selling:", COMPONENT);
        info(
            &component,
            &format!("Starting selling process for item: {}", item_info.name),
            &log_options
                .set_centered(true)
                .set_width(180)
                .set_enable(false),
        );
        // Get Settings.
        let settings = states::get_settings()?.live_scraper.stock_item;

        // Check if item is blacklisted for selling
        if settings.is_item_blacklisted(&item_info.wfm_id, &TradeMode::Sell) {
            info(
                format!("{}Blacklisted", COMPONENT),
                &format!(
                    "Item {} is blacklisted for selling. Skipping.",
                    item_info.name
                ),
                &log_options,
            );
            return Ok(());
        }

<<<<<<< HEAD
        // Get my order if it exists, otherwise empty values.
        let mut user_order = match my_orders.find_order_by_url_sub_type(
            &item_info.wfm_id,
            OrderType::Sell,
            stock_item.sub_type.as_ref(),
        ) {
            Some(mut order) => {
                order.operation = vec![];
                order
            }
            None => Order::default(),
=======
        let wfm_client = states::app_state()?.wfm_client;
        let per_trade = if item_info.bulk_tradable {
            Some(1)
        } else {
            None
>>>>>>> better-backend
        };
        let closed_avg = price.moving_avg.unwrap_or(0.0) as i64;

        let mut stock_item = entry.get_stock_item(conn).await.map_err(|e| {
            e.with_location(get_location!())
                .with_context(entry.to_json())
        })?;

        let mut order_info = get_order_info(item_info, entry, &wfm_client, OrderType::Sell);
        if stock_item.is_hidden && stock_item.status == StockStatus::InActive {
            info(
                format!("{}Skip", COMPONENT),
                &format!(
                    "Item {} is marked as hidden and inactive. Skipping.",
                    item_info.name
                ),
                &log_options.set_enable(false),
            );
            return Ok(());
        } else if stock_item.is_hidden && stock_item.status != StockStatus::InActive {
            stock_item.set_status(StockStatus::InActive);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            order_info.add_operation("Delete");
        }

<<<<<<< HEAD
        // Get the price the item was bought for.
        let bought_price = stock_item.bought as i64;

        let per_trade = if item_info.bulk_tradable {
            Some(0)
        } else {
            None
        };

        // Get the quantity of owned item.
        let quantity = entry.sell_quantity;

        // Get the minimum price of the item.
        let minimum_price = stock_item.minimum_price;

=======
>>>>>>> better-backend
        // Get the lowest sell order price from the DataFrame of live sell orders
        let lowest_price = if live_orders.sell_orders.len() > 2 {
            live_orders.lowest_price(OrderType::Sell)
        } else if stock_item.minimum_price.is_none() {
            order_info.add_operation("Delete");
            order_info.add_operation("NoSellers");
            stock_item.set_status(StockStatus::NoSellers);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            0
        } else {
            0
        };

        // Get the price the item was bought for.
        let bought_price = stock_item.bought as i64;

        // Then Price the order will be posted for.
        let mut post_price = lowest_price;

        // Handle Minimum Price Limit
        if let Some(minimum_price) = stock_item.minimum_price {
            let capped_price = post_price.max(minimum_price);
            if capped_price != post_price {
                post_price = capped_price;
                order_info.add_operation("MinimumPrice");
            }
        }

        // Handle SMA Threshold Global/Item Specific
        let minimum_sma = if stock_item.minimum_sma.is_some() {
            stock_item.minimum_sma.unwrap()
        } else {
            settings.min_sma
        };

        // Handle SMA Limit
        if !is_disabled(minimum_sma)
            && post_price < (closed_avg - minimum_sma)
            && lowest_price > bought_price
        {
            post_price = closed_avg;
            order_info.add_operation("SMALimit");
            stock_item.set_list_price(Some(post_price));
            stock_item.set_status(StockStatus::SMALimit);
            stock_item.locked = true;
        }

        // Calculate the profit from the post price
        let mut profit = post_price - bought_price;

<<<<<<< HEAD
        if profit < minimum_profit && minimum_profit != -1 {
            user_order.operation.push("LowProfit".to_string());
        }

        // Update Order Info
        user_order
            .info
            .set_total_sellers(live_orders.sell_orders.len() as i64);
        user_order.info.set_orders(live_orders.sell_orders.clone());
        user_order.info.set_moving_avg(moving_avg);
        user_order.info.set_name(item_info.name.clone());
        user_order.info.set_image(item_info.image_url.clone());
        user_order.info.set_highest_price(highest_price);
        user_order
            .info
            .set_lowest_price(live_orders.lowest_price(OrderType::Sell));
        user_order
            .info
            .set_orders(live_orders.sell_orders.into_iter().take(10).collect());
        user_order.info.set_profit(profit as f64);
        if user_order.id != "N/A" {
            user_order.operation.push("Updated".to_string());
        } else {
            user_order.operation.push("Created".to_string());
        }

        // If the post price is not equal to the platinum price of the order, update it.
        if user_order.platinum != post_price {
            user_order.platinum = post_price;
            user_order.info.is_dirty = true;
        }

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
                    per_trade,
                    stock_item.sub_type.clone(),
                    Some(user_order.info.clone()),
                )
                .await
            {
                Ok((rep, None)) => {
                    if &rep == "order_limit_reached" {
                        stock_item.set_status(StockStatus::OrderLimit);
                    }
                }
                Ok((_, order)) => {
                    if let Some(mut order) = order {
                        order.info = user_order.info.clone();
                        order.operation = user_order.operation.clone();
                        user_order = order.clone();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if user_order.operation.contains(&"LowProfit".to_string()) {
=======
        // Handle Profit Threshold Global/Item Specific
        let minimum_profit = if stock_item.minimum_profit.is_some() {
            stock_item.minimum_profit.unwrap()
        } else {
            settings.min_profit
        };
        // Handle Low Profit
        if !is_disabled(minimum_profit) && profit < minimum_profit {
            post_price += minimum_profit - profit;
>>>>>>> better-backend
            stock_item.set_status(StockStatus::ToLowProfit);
            stock_item.set_list_price(Some(post_price));
            stock_item.locked = true;
            order_info.add_operation("LowProfit");
            profit = post_price - bought_price;
        }

        // Update Order Info & Stock Item
        order_info = order_info.set_profit(profit as f64);
        order_info = order_info.set_closed_avg(closed_avg as f64);
        order_info = order_info.set_highest_price(live_orders.highest_price(OrderType::Sell));
        order_info = order_info.set_lowest_price(live_orders.lowest_price(OrderType::Sell));
        order_info = order_info.set_orders(live_orders.take_top(5, OrderType::Sell));
        order_info.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));
        stock_item.set_list_price(Some(post_price));
        stock_item.set_status(StockStatus::Live);
        if stock_item.status == StockStatus::Live {
            stock_item.add_price_history(PriceHistory::new(
                chrono::Local::now().naive_local().to_string(),
                post_price,
            ));
        }

        // Ensure post price is at least 1 platinum
        post_price = std::cmp::max(post_price, 1);

        // Summary log
        info(
        format!("{}Summary", component),
        format!(
            "Item {}: PostPrice: {} | ClosedAvg: {} | Profit: {} | IsStockDirty: {} | StockStatus: {:?} | StockListPrice: {:?} | StockChanges: {} | {}",
            item_info.name, post_price, closed_avg, profit, stock_item.is_dirty, stock_item.status, stock_item.list_price, stock_item.changes.join(", "), order_info,
        ),
        &log_options,
        );

        // Create/Update/Delete
        match progress_order(
            &component,
            &wfm_client,
            &order_info,
            OrderType::Sell,
            post_price as u32,
            per_trade,
            log_options,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .with_location(get_location!())
                    .with_context(entry.to_json()))
            }
        }

        if stock_item.is_dirty {
            match StockItemMutation::update_by_id(conn, stock_item.to_update()).await {
                Ok(_) => {
                    info(
                        format!("{}StockItemUpdate", component),
                        &format!("Updated stock item: {:?}", entry.stock_id),
                        &log_options,
                    );
                    if stock_item.update_gui() {
                        send_event!(
                            UIEvent::RefreshStockItems,
                            json!({"id": entry.stock_id, "source": component})
                        );
                    }
                }
                Err(e) => return Err(e.with_location(get_location!())),
            }
        }
        Ok(())
    }

    pub async fn progress_wish_list(
        &self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let log_options = &LoggerOptions::default();
        let component = format!("{}WishList:", COMPONENT);
        info(
            &component,
            &format!("Starting wishlist process for item: {}", item_info.name),
            &log_options
                .set_centered(true)
                .set_width(180)
                .set_enable(false),
        );
        let settings = states::get_settings()?.live_scraper.stock_item;
        // Check if item is blacklisted for wishlist
        if settings.is_item_blacklisted(&item_info.wfm_id, &TradeMode::WishList) {
            info(
                format!("{}Blacklisted", COMPONENT),
                &format!(
                    "Item {} is blacklisted for wishlist. Skipping.",
                    item_info.name
                ),
                &log_options.set_enable(false),
            );
            return Ok(());
        }
        let wfm_client = states::app_state()?.wfm_client;
        let per_trade = if item_info.bulk_tradable {
            Some(1)
        } else {
            None
        };

        let mut wishlist_item = entry.get_wish_list_item(conn).await.map_err(|e| {
            e.with_location(get_location!())
                .with_context(entry.to_json())
        })?;

        let mut order_info = get_order_info(item_info, entry, &wfm_client, OrderType::Buy);
        if wishlist_item.is_hidden && wishlist_item.status == StockStatus::InActive {
            return Ok(());
        } else if wishlist_item.is_hidden && wishlist_item.status != StockStatus::InActive {
            wishlist_item.set_status(StockStatus::InActive);
            wishlist_item.set_list_price(None);
            wishlist_item.locked = true;
            order_info.add_operation("Delete");
        }
        // Get The highest buy order returns 0 if there are no buy orders.
        let highest_price = live_orders.highest_price(OrderType::Buy);

        // Set the post price to the highest price.
        let mut post_price = highest_price;

        // Get Maximum and Minimum Price from Wishlist Item
        let maximum_price = wishlist_item.maximum_price.unwrap_or(0);
        let minimum_price = wishlist_item.minimum_price.unwrap_or(0);

        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            order_info.add_operation("NoBuyers");
            post_price = price.avg_price as i64;
            wishlist_item.set_status(StockStatus::NoBuyers);
            wishlist_item.set_list_price(Some(post_price));
        }
        // Check if the price is higher than the max price
        if post_price > maximum_price && maximum_price > 0 {
            post_price = maximum_price;
            order_info.add_operation("MaxPrice");
        }
        post_price = std::cmp::max(post_price, 1);

        if post_price < minimum_price && minimum_price > 0 {
            post_price = minimum_price;
            order_info.add_operation("MinPrice");
        }

        // Update Order Info
        order_info = order_info.set_highest_price(highest_price);
        order_info = order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
        order_info = order_info.set_orders(live_orders.take_top(10, OrderType::Buy));
        order_info.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));
        // Summary log
        info(
        format!("{}Summary", component),
        format!(
            "Item {}: PostPrice: {} | IsWishlistDirty: {} | WishlistStatus: {:?} | WishlistListPrice: {:?} | {}",
            item_info.name, post_price, wishlist_item.is_dirty, wishlist_item.status, wishlist_item.list_price, order_info
        ),
        &log_options,
        );

        // Create/Update/Delete
        match progress_order(
            &component,
            &wfm_client,
            &order_info,
            OrderType::Buy,
            post_price as u32,
            per_trade,
            log_options,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .with_location(get_location!())
                    .with_context(entry.to_json()))
            }
        }
        wishlist_item.set_list_price(Some(post_price));
        wishlist_item.set_status(StockStatus::Live);
        if wishlist_item.status == StockStatus::Live {
            wishlist_item.add_price_history(PriceHistory::new(
                chrono::Local::now().naive_local().to_string(),
                post_price,
            ));
        }
        if wishlist_item.is_dirty {
            match WishListMutation::update_by_id(conn, wishlist_item.to_update()).await {
                Ok(_) => {
                    info(
                        format!("{}Update", component),
                        &format!("Updated wishlist item: {:?}", entry.wish_list_id),
                        &log_options,
                    );
                    send_event!(
                        UIEvent::RefreshWishListItems,
                        json!({"id": entry.wish_list_id, "source": component})
                    );
                }
                Err(e) => {
                    return Err(e
                        .with_location(get_location!())
                        .with_context(entry.to_json()))
                }
            }
        }
        Ok(())
    }
}
