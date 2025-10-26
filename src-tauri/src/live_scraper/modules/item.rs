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
    types::{Order, OrderList, OrderWithUser},
    utils::write_json_file,
};

use crate::{
    app::{client::AppState, Settings},
    cache::{
        client::CacheState,
        types::{CacheTradableItem, ItemPriceInfo},
    },
    enums::FindBy,
    utils::{ErrorFromExt, OrderListExt},
};
use crate::{
    enums::TradeMode, live_scraper::*, send_event, types::*, utils::modules::states,
    utils::SubTypeExt, DATABASE,
};

static COMPONENT: &str = "LiveScraper:ItemModule:";
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
    fn send_event(&self, i18nKey: impl Into<String>, values: Option<serde_json::Value>) {
        send_event!(
            UIEvent::SendLiveScraperMessage,
            json!({"i18nKey": format!("item.{}", i18nKey.into()), "values": values})
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
        let mut current_index = interesting_items.len();

        // Sort by priority (highest first)
        interesting_items.sort_by(|a, b| b.priority.cmp(&a.priority));
        let total = interesting_items.len();

        for item_entry in interesting_items {
            // Stop if client stopped running or user is banned
            if !client.is_running.load(Ordering::SeqCst) || app.user.is_banned() {
                warning(
                    format!("{}ProcessItem", COMPONENT),
                    "Live Scraper is not running or user is banned, stopping processing.",
                    &&LoggerOptions::default(),
                );
                break;
            }

            // Get tradable item info from cache
            let Some(item_info) = cache.tradable_item().get_by(FindBy::new(
                crate::enums::FindByType::Url,
                &item_entry.wfm_url,
            ))?
            else {
                error(
                    format!("{}ProcessItem", COMPONENT),
                    &format!("Item not found in tradable items: {}", item_entry.wfm_url),
                    &&LoggerOptions::default(),
                );
                continue;
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
                    return Err(Error::from_wfm(
                        format!("{}ProcessItem", COMPONENT),
                        &format!("Failed to get live orders for item {}", item_entry.wfm_url),
                        e,
                        get_location!(),
                    ))
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

            // Process buying logic
            if item_entry.operation.contains(&"Buy".to_string())
                && !item_entry.operation.contains(&"WishList".to_string())
            {
                if let Err(e) = self
                    .progress_buying(&item_info, &item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
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

            // Process wishlist logic (future expansion)
            if item_entry.operation.contains(&"WishList".to_string()) {
                if let Err(e) = self
                    .progress_wish_list(&item_info, &item_entry, &item_price, &orders)
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

            // Process selling logic (future expansion)
            if item_entry.operation.contains(&"Sell".to_string()) && item_entry.stock_id.is_some() {
                if let Err(e) = self
                    .progress_selling(&item_info, &item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
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

            current_index -= 1;
        }

        Ok(())
    }

    pub async fn progress_buying(
        &self,
        item_info: &CacheTradableItem,
        entry: &ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let log_options = &LoggerOptions::default()
            .set_file("progress_buying.log")
            .set_show_component(false)
            .set_show_time(false);
        let component = format!("{}ProgressBuying:", COMPONENT);
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
            );
            return Ok(());
        }

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
        } else {
            None
        };

        let mut order_info = get_order_info(item_info, entry, &wfm_client, OrderType::Buy);

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

        // Update Order Info
        order_info = order_info.set_profit(price_range as f64);
        order_info = order_info.set_closed_avg(closed_avg);
        order_info = order_info.set_highest_price(highest_price);
        order_info = order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));

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

        if closed_avg_metric >= 0
            && price_range >= profit_threshold
            && order_info.has_operation("Create")
            && !wfm_client.order().cache_orders().buy_orders.is_empty()
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
            };

            let (selected_buy_orders, unselected_buy_orders) =
                knapsack(buy_orders_list.clone(), max_total_price_cap);

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
        } else if closed_avg_metric < 0 {
            order_info.add_operation("Delete");
            order_info.add_operation("Overpriced");
        } else if price_range < profit_threshold {
            order_info.add_operation("Delete");
            order_info.add_operation("Underpriced");
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
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let log_options = &LoggerOptions::default()
            .set_file("progress_selling.log")
            .set_show_component(false)
            .set_show_time(false);
        let component = format!("{}ProgressSelling:", COMPONENT);
        info(
            &component,
            &format!("Starting selling process for item: {}", item_info.name),
            &log_options
                .set_centered(true)
                .set_width(180)
                .set_enable(true),
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

        let wfm_client = states::app_state()?.wfm_client;
        let per_trade = if item_info.bulk_tradable {
            Some(1)
        } else {
            None
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
                &log_options,
            );
            return Ok(());
        } else if stock_item.is_hidden && stock_item.status != StockStatus::InActive {
            stock_item.set_status(StockStatus::InActive);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            order_info.add_operation("Delete");
        }

        // Get the lowest sell order price from the DataFrame of live sell orders
        let lowest_price = if live_orders.sell_orders.len() > 2 {
            live_orders.lowest_price(OrderType::Sell)
        } else {
            order_info.add_operation("Delete");
            order_info.add_operation("NoSellers");
            stock_item.set_status(StockStatus::NoSellers);
            stock_item.set_list_price(None);
            stock_item.locked = true;
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

        // Handle Profit Threshold Global/Item Specific
        let minimum_profit = if stock_item.minimum_profit.is_some() {
            stock_item.minimum_profit.unwrap()
        } else {
            settings.min_profit
        };
        // Handle Low Profit
        if !is_disabled(minimum_profit) && profit < minimum_profit {
            post_price += minimum_profit - profit;
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
                Err(e) => {
                    return Err(Error::from_db(
                        format!("{}StockItemUpdate", component),
                        &format!("Failed to update stock item {:?}", entry.stock_id),
                        e,
                        get_location!(),
                    )
                    .with_context(entry.to_json()));
                }
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
        let log_options = &LoggerOptions::default()
            .set_file("progress_wish_list.log")
            .set_show_component(false)
            .set_show_time(false);
        let component = format!("{}ProgressWishList:", COMPONENT);
        info(
            &component,
            &format!("Starting wishlist process for item: {}", item_info.name),
            &log_options
                .set_centered(true)
                .set_width(180)
                .set_enable(true),
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
                &log_options,
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

        // Get Maximum Price
        let maximum_price = wishlist_item.maximum_price.unwrap_or(0);
        // Return if no buy orders are found.
        if live_orders.buy_orders.len() <= 0 {
            order_info.add_operation("NoBuyers");
            post_price = price.avg_price as i64;
            wishlist_item.set_status(StockStatus::NoBuyers);
            wishlist_item.set_list_price(Some(post_price));
            wishlist_item.locked = true;
        }
        // Check if the price is higher than the max price
        if post_price > maximum_price && maximum_price > 0 {
            post_price = maximum_price;
            order_info.add_operation("MaxPrice");
        }
        post_price = std::cmp::max(post_price, 1);

        // Update Order Info
        order_info = order_info.set_highest_price(highest_price);
        order_info = order_info.set_lowest_price(live_orders.lowest_price(OrderType::Buy));
        order_info = order_info.set_orders(live_orders.take_top(10, OrderType::Buy));

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
                        format!("{}WishListUpdate", component),
                        &format!("Updated wishlist item: {:?}", entry.wish_list_id),
                        &log_options,
                    );
                    send_event!(
                        UIEvent::RefreshWishListItems,
                        json!({"id": entry.wish_list_id, "source": component})
                    );
                }
                Err(e) => {
                    return Err(Error::from_db(
                        format!("{}WishListUpdate", component),
                        &format!("Failed to update wishlist item {:?}", entry.wish_list_id),
                        e,
                        get_location!(),
                    )
                    .with_context(entry.to_json()));
                }
            }
        }
        Ok(())
    }
}
