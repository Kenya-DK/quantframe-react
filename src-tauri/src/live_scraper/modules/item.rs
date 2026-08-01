use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{atomic::Ordering, Arc, Weak},
};

use entity::{dto::PriceHistory, enums::stock_status::StockStatus};
use serde_json::json;
use utils::*;
use wf_market::{
    enums::{OrderType, StatusType},
    types::{Order, OrderList, OrderWithUser},
};

use crate::{
    app::{AppState, Settings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    utils::OrderListExt,
};
use crate::{
    enums::TradeMode, live_scraper::*, send_event, types::*, utils::modules::states,
    utils::ErrorFromExt, utils::SubTypeExt, DATABASE,
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
        if !settings.live_scraper.general.delete_conflicting_orders
            && !settings.live_scraper.general.auto_delete
        {
            return Ok(()); // Nothing to delete
        }

        // Determine which order IDs to delete
        let order_ids = orders_to_delete(settings, &client, my_orders);

        // Delete each unwanted order
        let mut current_index = order_ids.len();
        let total = order_ids.len();
        for id in order_ids.iter() {
            // Stop if client stopped running or user is banned
            if Self::should_stop(&client, app) {
                warning(
                    comp("Delete"),
                    "Live Scraper is not running or user is banned, stopping deletion.",
                    &&LoggerOptions::default(),
                );
                break;
            }
            match app.wfm_client.order().delete(id).await {
                Ok(_) => {
                    info(
                        comp("Delete"),
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
                    comp("Delete"),
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
            comp("Check"),
            "Checking Item items...",
            &&LoggerOptions::default(),
        );

        // Get My Orders from Warframe Market.
        let my_orders = app.wfm_client.order().cache_orders();

        // Delete unwanted orders
        self.delete_unwanted_orders(&app, &app.settings, &my_orders)
            .await?;

        // Collect interesting items
        let interesting_items = collect_interesting_items(&app, COMPONENT, &app.settings).await?;

        // Process interesting items
        self.process_items(interesting_items, &app).await?;
        Ok(())
    }
    fn should_stop(client: &LiveScraperState, app: &AppState) -> bool {
        !client.is_running.load(Ordering::SeqCst) || app.user.is_banned()
    }
    /// Processes a list of interesting items, applying buying, selling, syndicate, and wishlist logic as configured in the settings.
    async fn process_items(
        &self,
        mut interesting_items: Vec<ItemEntry>,
        app: &AppState,
    ) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let client = self.client.upgrade().expect("Client should not be dropped");
        let use_fake = app.settings.debugging.live_scraper.fake_orders;
        let mut current_index = 1;

        // Snapshot existing buy order IDs before any processing this cycle
        let existing_buy_order_ids: HashSet<String> = app
            .wfm_client
            .order()
            .cache_orders()
            .buy_orders
            .iter()
            .map(|o| o.id.clone())
            .collect();

        // Sort by priority (highest first)
        interesting_items.sort_by(|a, b| b.priority.cmp(&a.priority));
        let total = interesting_items.len();

        for item_entry in interesting_items.iter_mut() {
            // Stop if client stopped running or user is banned
            if Self::should_stop(&client, &app) {
                warning(
                    comp("ProcessItem"),
                    "Live Scraper is not running or user is banned, stopping processing.",
                    &&LoggerOptions::default(),
                );
                break;
            }

            // Get tradable item info from cache
            let item_info = match cache.tradable_item().get_by(&item_entry.wfm_url) {
                Ok(item) => item,
                Err(e) => {
                    e.set_component(comp("ProcessItem")).log(LOG_FILE);
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

            let order_path = PathBuf::from(utils::get_base_path())
                .join("fake_orders")
                .join(format!("order_{}.json", item_info.wfm_url));

            let mut orders = load_orders(
                &comp("ProcessItem:LoadOrders:"),
                &app.wfm_client,
                &item_entry.wfm_url,
                use_fake.then_some(&order_path),
            )
            .await?;

            // Apply filters to orders
            orders.filter_by_sub_type(
                wf_market::types::SubType::from_entity(item_entry.sub_type.clone()),
                false,
            );
            orders.filter_username(&app.user.wfm_username, true);
            orders.filter_user_status(StatusType::InGame, false);
            orders.sort_by_platinum();

            // Add Market Info to ItemEntry
            item_entry.apply_market_info(&orders);

            info(
                &comp("ProcessItem"),
                &format!(
                    "Processing Item: {} | Buy Orders: {} | Sell Orders: {} | Operations: {:?} | Progress: {}/{}",
                    item_info.name,
                    orders.buy_orders.len(),
                    orders.sell_orders.len(),
                    item_entry.operations.operations,
                    current_index,
                    total
                ),
                &&LoggerOptions::default(),
            );

            if item_entry.operations.has("Buy") && !item_entry.operations.has("WishList") {
                if let Err(e) = self
                    .progress_buying(&item_info, item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
                }

                info(
                    &comp("ProgressBuying"),
                    &format!(
                        "Successfully processed buying for item: {}",
                        item_entry.wfm_url
                    ),
                    &LoggerOptions::default(),
                );
            }

            // Process wishlist logic (future expansion)
            if item_entry.operations.has(&"WishList".to_string()) {
                if let Err(e) = self
                    .progress_wish_list(&item_info, item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
                }

                info(
                    &comp("ProgressWishList"),
                    &format!(
                        "Successfully processed wishlist for item: {}",
                        item_entry.wfm_url
                    ),
                    &LoggerOptions::default(),
                );
            }

            // Process selling logic (future expansion)
            if item_entry.operations.has("Sell") && item_entry.stock_id.is_some() {
                if let Err(e) = self
                    .progress_selling(&item_info, item_entry, &item_price, &orders)
                    .await
                {
                    return Err(e.with_location(get_location!()));
                }

                info(
                    &comp("ProgressSelling"),
                    &format!(
                        "Successfully processed selling for item: {}",
                        item_entry.wfm_url
                    ),
                    &LoggerOptions::default(),
                );
            }

            // Process syndicate logic (future expansion) DISABLED FOR NOW WIP
            if item_entry.operations.has("Syndicate") {
                // if let Err(e) = self
                //     .progress_syndicate(&item_info, item_entry, &item_price, &orders)
                //     .await
                // {
                //     return Err(e.with_location(get_location!()));
                // }

                // info(
                //     &comp("ProgressSyndicate"),
                //     &format!(
                //         "Successfully processed syndicate for item: {}",
                //         item_entry.wfm_url
                //     ),
                //     &LoggerOptions::default(),
                // );
            }
            current_index += 1;
        }

        // Global knapsack pass: run once on all cached orders after all items processed
        let all_buy_orders = app
            .wfm_client
            .order()
            .cache_orders()
            .extract_order_summary(OrderType::Buy);

        let max_total_price_cap = app.settings.live_scraper.items.wtb.max_total_price_cap;
        if all_buy_orders.len() > 1 && !is_disabled(max_total_price_cap) {
            info(
                &comp("GlobalKnapsack"),
                &format!(
                    "Running global knapsack check: {} buy orders | Cap: {}",
                    all_buy_orders.len(),
                    max_total_price_cap
                ),
                &&LoggerOptions::default(),
            );
            let (_, unselected) = knapsack(all_buy_orders, max_total_price_cap);
            if !unselected.is_empty() {
                let component = comp("GlobalKnapsack");
                for order in &unselected {
                    if order.3.is_empty() || !existing_buy_order_ids.contains(&order.3) {
                        // Skip orders created this cycle — let them survive until next check
                        continue;
                    }
                    if let Err(err) = app.wfm_client.order().delete(&order.3).await {
                        error(
                            &component,
                            &format!("Failed to delete {}: {}", order.3, err),
                            &&LoggerOptions::default().set_file(LOG_FILE),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Progresses the buying workflow for a single item.
    pub async fn progress_buying(
        &self,
        item_info: &CacheTradableItem,
        entry: &mut ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let log_options = &LoggerOptions::default().set_enable(true);
        let component = comp("Buying");
        let settings = states::get_settings()?.live_scraper.items;

        // Helper function to log messages with the component prefix
        let log = |msg: &str| info(&component, msg, &log_options);

        // Skip if item is blacklisted for buying
        if is_blacklisted(&settings, item_info, entry, &TradeMode::Buy) {
            log(&format!(
                "Item {} is blacklisted for buying. Skipping.",
                item_info.name
            ));
            return Ok(());
        }
        // Get the Warframe Market client from the application state
        let wfm_client = states::app_state()?.wfm_client;

        // Get the per-trade quantity for this item, based on its type and settings
        let per_trade = get_per_trade(item_info);

        // Get item Price Info from cache, including moving average and other metrics
        let closed_avg = price.moving_avg.unwrap_or(0.0);

        // WTB (Want to Buy) configuration used for price calculations
        let max_stock_quantity = settings.wtb.max_stock_quantity;
        let avg_price_cap = settings.wtb.avg_price_cap;
        let max_total_price_cap = settings.wtb.max_total_price_cap;
        let profit_threshold = settings.wtb.profit_threshold;

        // Current market snapshot for this item's buy orders
        let market_info = &entry.buy_market_info;

        // Determine the initial post price based on market conditions
        let mut post_price = market_info.highest_price;

        // Existing Warframe Market order state and metadata
        let (order_id, current_order_price, mut properties, mut trade_operations) =
            get_order_info(entry, OrderType::Buy, &wfm_client);

        // Conditions
        if entry.buy_market_info.volume == 0 || entry.sell_market_info.volume == 0 {
            log(&format!(
                "Item {} has no market volume. Skipping WTB order creation.",
                item_info.name
            ));
            return Ok(());
        }

        // Handle Max Stock Quantity threshold: if we already own enough of this item, skip creating a new WTB order. and delete any existing WTB order for it.
        if !is_disabled(max_stock_quantity) && entry.stock_id.is_some() {
            let stock_item = entry.get_stock_item_or_error(conn).await?;
            if stock_item.owned >= max_stock_quantity {
                log(&format!(
                    "Item {} already has {} units in stock (max: {}). Skipping WTB order creation.",
                    item_info.name, stock_item.owned, max_stock_quantity
                ));
                if let Err(e) = delete_order(
                    &component,
                    entry,
                    OrderType::Buy,
                    &states::app_state()?.wfm_client,
                )
                .await
                {
                    return Err(e
                        .with_location(get_location!())
                        .with_context(entry.to_json()));
                }
                log(&format!(
                    "Deleted WTB order for item {} due to max stock quantity.",
                    item_info.name
                ));
                return Ok(());
            }
        }

        // Handle Max Price Drop & Min Listings Below
        if let Some(reason) = should_apply_max_price_drop(
            settings.wtb.max_price_drop,
            settings.wtb.min_listings_below,
            current_order_price,
            post_price,
            live_orders.get_price_list(OrderType::Buy, None),
            OrderType::Buy,
        ) {
            post_price = current_order_price;
            trade_operations.add(reason);
        }

        // How far above (positive) or below (negative) our post price is from the market average.
        // Used to gauge whether we're overpaying relative to recent trades.
        let closed_avg_metric = closed_avg as i64 - post_price;

        // Rough expected profit: the margin between our buy price and the average sell price, minus 1 plat buffer.
        let potential_profit = closed_avg_metric - 1;

        // Per-item max price cap — if this item has a specific max configured, clamp the post price.
        let item_max_price = settings.general.get_item_max_price(&item_info.wfm_id);
        if item_max_price > 0 && post_price > item_max_price {
            trade_operations.add("AboveMaxBuyPrice");
            post_price = item_max_price;
        }

        // Global average price cap — if post price exceeds it, mark for deletion regardless of other checks.
        if !is_disabled(avg_price_cap) && post_price > avg_price_cap {
            trade_operations.add("AboveAvgPrice");
            trade_operations.add("Delete");

            log(&format!(
                "Item {} is above the average price cap.",
                item_info.name
            ));
        }

        // Knapsack solver: enforces a total platinum budget across all buy orders.
        // Given the cap and all existing orders + this item, it selects the most profitable subset.
        // Items not selected get deferred deletion (cleaned up by the global pass in process_items).
        if !is_disabled(max_total_price_cap) {
            let mut all_orders = wfm_client
                .order()
                .cache_orders()
                .extract_order_summary(OrderType::Buy);
            if !all_orders.iter().any(|i| i.2 == item_info.wfm_id) {
                all_orders.push((
                    post_price,
                    potential_profit as f64,
                    item_info.wfm_id.clone(),
                    String::new(),
                ));
            }

            log(&format!(
                "Knapsack Input: {} | Buy orders | Cap: {}",
                all_orders.len(),
                max_total_price_cap
            ));
            let (selected, unselected) = knapsack(all_orders, max_total_price_cap);
            log(&format!(
                "Knapsack selected {}/{} buy orders for {}.",
                selected.len(),
                selected.len() + unselected.len(),
                item_info.name
            ));

            let selected_ids: HashSet<_> = selected.iter().map(|o| &o.2).collect();

            if !selected_ids.contains(&item_info.wfm_id) {
                log(&format!("{} was not selected.", item_info.name));
                trade_operations.add("Skip");
                trade_operations.add("Delete");
                return Ok(());
            }

            for order in &unselected {
                log(&format!(
                    "Deferred deletion of unselected order {} for item {} (will delete after all items processed).",
                    order.3, order.2
                ));
            }
        }

        // If our post price is higher than the market average, flag as overpriced.
        if closed_avg_metric < 0 {
            trade_operations.add("Delete");
            trade_operations.add("Overpriced");
        }

        // If the spread between lowest and highest buy orders is too narrow, not worth competing.
        if market_info.price_range < profit_threshold {
            trade_operations.add("Delete");
            trade_operations.add("Underpriced");
        }

        log_summary(
            &component,
            format!(
                "Item {} | Post: {} | CurOrder: {} ({}) \
                 | Market: {} \
                 | Price: ClosedAvg: {} | MovingAvg: {} | Avg: {} | Min: {} | Max: {} \
                 | Profit: Metric: {} | Potential: {} | Threshold: {} \
                 | Ops: {:?}",
                item_info.name,
                post_price,
                current_order_price,
                order_id,
                market_info,
                closed_avg,
                price.moving_avg.unwrap_or(0.0),
                price.avg_price,
                price.min_price,
                price.max_price,
                closed_avg_metric,
                potential_profit,
                profit_threshold,
                trade_operations.operations
            ),
            log_options,
        );

        // Attach trade-operation metadata to the order properties
        populate_order_properties(&mut properties, item_info, entry, &trade_operations);

        // Attach market-metrics metadata (volume, velocity, etc.)
        set_order_market_metrics(
            &mut properties,
            post_price,
            potential_profit,
            price,
            live_orders,
            OrderType::Buy,
        );
        // Submit the order to Warframe Market (create, update, or delete)
        match progress_order(
            &component,
            entry,
            &wfm_client,
            OrderType::Buy,
            post_price as u32,
            per_trade,
            log_options,
            &mut properties,
            &trade_operations,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .with_location(get_location!())
                    .with_context(entry.to_json()));
            }
        }
        Ok(())
    }

    /// Progresses the selling workflow for a single item.
    pub async fn progress_selling(
        &self,
        item_info: &CacheTradableItem,
        entry: &mut ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let log_options = &LoggerOptions::default();
        let component = comp("Selling");
        let settings = states::get_settings()?.live_scraper.items;

        let log = |msg: &str| info(&component, msg, &log_options);

        // Skip if item is blacklisted for selling
        if is_blacklisted(&settings, item_info, entry, &TradeMode::Sell) {
            log(&format!(
                "Item {} is blacklisted for selling. Skipping.",
                item_info.name
            ));
            return Ok(());
        }

        // Get the Warframe Market client from the application state
        let wfm_client = states::app_state()?.wfm_client;

        // Get the per-trade quantity for this item, based on its type and settings
        let per_trade = get_per_trade(item_info);

        // Use the moving average as the baseline closed-average price
        let closed_avg = price.moving_avg.unwrap_or(0.0) as i64;
        let mut stock_item = entry.get_stock_item_or_error(conn).await?;
        let bought_price = stock_item.bought;
        let market_info = &entry.sell_market_info;

        // Fetch existing order details and prepare mutable state
        let (_, current_order_price, mut properties, mut trade_operations) =
            get_order_info(&entry, OrderType::Sell, &wfm_client);

        // Per-item overrides stored on the stock item (optional)
        let (min_price, min_profit, min_sma) = (
            stock_item
                .properties
                .get_property_value("min_price", None::<i64>),
            stock_item
                .properties
                .get_property_value("min_profit", None::<i64>),
            stock_item
                .properties
                .get_property_value("min_sma", None::<i64>),
        );

        log(&format!(
            "Item {}: Overrides — min_price={:?}, min_profit={:?}, min_sma={:?}",
            item_info.name, min_price, min_profit, min_sma
        ));

        // Hidden + inactive → nothing to do; hidden + active → deactivate and delete order
        if stock_item.is_hidden && stock_item.status == StockStatus::InActive {
            log(&format!(
                "Item {} is marked as hidden and inactive. Skipping.",
                item_info.name
            ));
            return Ok(());
        } else if stock_item.is_hidden && stock_item.status != StockStatus::InActive {
            log(&format!(
                "Item {} is hidden and active. Setting to inactive and deleting order.",
                item_info.name
            ));
            stock_item.set_status(StockStatus::InActive);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            trade_operations.add("Delete");
        }

        // Determine lowest competitor price. Fall back to 0 if no sellers exist and no min_price override is set.
        let lowest_price = if market_info.volume >= 2 {
            market_info.lowest_price
        } else if min_price.is_none() {
            log(&format!(
                "Item {} has no sellers and no minimum price. Skipping.",
                item_info.name
            ));
            trade_operations.add("Delete");
            trade_operations.add("NoSellers");
            stock_item.set_status(StockStatus::NoSellers);
            stock_item.set_list_price(None);
            stock_item.locked = true;
            0
        } else {
            log(&format!(
                "Item {} has no sellers but has min_price override. Using bought price as fallback.",
                item_info.name
            ));
            0
        };

        // Start with the lowest competitor price as the initial post price
        let mut post_price = lowest_price;

        // Clamp to per-item minimum price if set
        if let Some(min_price) = min_price {
            let capped_price = post_price.max(min_price);
            if capped_price != post_price {
                log(&format!(
                    "Item {} price capped to minimum price {}.",
                    item_info.name, capped_price
                ));
                post_price = capped_price;
                trade_operations.add("MinimumPrice");
            }
        }

        // Prevent prices from dropping too fast relative to existing order
        if let Some(reason) = should_apply_max_price_drop(
            settings.wtb.max_price_drop,
            settings.wtb.min_listings_below,
            current_order_price,
            post_price,
            live_orders.get_price_list(OrderType::Sell, None),
            OrderType::Sell,
        ) {
            log(&format!(
                "Item {} max price drop applied ({}).",
                item_info.name, reason
            ));
            post_price = current_order_price;
            trade_operations.add(reason);
        }

        let minimum_sma = min_sma.unwrap_or(settings.wts.min_sma);

        // Enforce SMA floor: don't sell below (closed average - min_sma) if there's competition
        if !is_disabled(minimum_sma)
            && post_price < (closed_avg - minimum_sma)
            && lowest_price > bought_price
        {
            log(&format!(
                "Item {} price adjusted to SMA limit (closed avg: {}, min_sma: {}).",
                item_info.name, closed_avg, minimum_sma
            ));
            post_price = closed_avg;
            trade_operations.add("SMALimit");
            stock_item.set_list_price(Some(post_price));
            stock_item.set_status(StockStatus::SMALimit);
            stock_item.locked = true;
        }

        // Ensure profit meets the minimum threshold by raising the price if needed
        let mut profit = post_price - bought_price;
        let minimum_profit = min_profit.unwrap_or(settings.wts.min_profit);

        if !is_disabled(minimum_profit) && profit < minimum_profit {
            let adjustment = minimum_profit - profit;
            log(&format!(
                "Item {} profit too low ({}) < min ({}), adjusting by {}.",
                item_info.name, profit, minimum_profit, adjustment
            ));
            post_price += adjustment;
            stock_item.set_status(StockStatus::ToLowProfit);
            stock_item.set_list_price(Some(post_price));
            stock_item.locked = true;
            trade_operations.add("LowProfit");
            profit = post_price - bought_price;
        }

        // Persist final price, mark as live, and record price history
        stock_item.set_list_price(Some(post_price));
        stock_item.set_status(StockStatus::Live);
        stock_item.add_price_history(PriceHistory::new(
            chrono::Local::now().naive_local().to_string(),
            post_price,
        ));

        post_price = std::cmp::max(post_price, 1);

        log_summary(
            &component,
            format!(
                "Item {} | Post: {} | CurOrder: {} | Lowest: {} | ClosedAvg: {} | Bought: {} | Profit: {} \
                 | Market: {} \
                 | Price: AVG: {} | Min: {} | Max: {} | Median: {} | WeekShift: {} \
                 | MinSMA: {} | MinProfit: {} | MinPrice: {:?} \
                 | Hidden: {} | Locked: {} | Status: {:?} | Ops: {:?}",
                item_info.name,
                post_price,
                current_order_price,
                lowest_price,
                closed_avg,
                bought_price,
                profit,
                market_info,
                price.avg_price,
                price.min_price,
                price.max_price,
                price.median,
                price.week_price_shift,
                minimum_sma,
                minimum_profit,
                min_price,
                stock_item.is_hidden,
                stock_item.locked,
                stock_item.status,
                trade_operations.operations,
            ),
            log_options,
        );

        // Attach trade-operation metadata to the order properties
        populate_order_properties(&mut properties, &item_info, &entry, &trade_operations);

        // Attach market-metrics metadata (volume, velocity, etc.)
        set_order_market_metrics(
            &mut properties,
            post_price,
            profit,
            price,
            live_orders,
            OrderType::Sell,
        );

        // Submit the order to Warframe Market (create, update, or delete)
        match progress_order(
            &component,
            entry,
            &wfm_client,
            OrderType::Sell,
            post_price as u32,
            per_trade,
            log_options,
            &mut properties,
            &trade_operations,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(e
                    .with_location(get_location!())
                    .with_context(entry.to_json()));
            }
        }

        // Flush stock-item changes to the database
        entry
            .finalize_stock_item(conn, &component, &mut stock_item, log_options)
            .await?;
        Ok(())
    }

    /// Progresses the wishlist workflow for a single item.
    pub async fn progress_wish_list(
        &self,
        item_info: &CacheTradableItem,
        entry: &mut ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let conn = DATABASE.get().unwrap();
        let component = comp("WishList:");
        let log_options = LoggerOptions::default();
        let settings = states::get_settings()?.live_scraper.items;
        let wfm_client = states::app_state()?.wfm_client;

        let log = |msg: &str| info(&component, msg, &log_options);

        // Skip items that are not allowed to be bought.
        if is_blacklisted(&settings, item_info, entry, &TradeMode::WishList) {
            log(&format!(
                "Item {} is blacklisted for wishlist buying. Skipping.",
                item_info.name
            ));
            return Ok(());
        }

        let market = &entry.buy_market_info;
        let per_trade = get_per_trade(item_info);

        // Load the persisted wishlist item.
        let mut wishlist_item = entry.get_wishlist_item_or_error(conn).await?;

        // Retrieve the current order state and metadata.
        let (_, _, mut properties, mut trade_operations) =
            get_order_info(entry, OrderType::Buy, &wfm_client);

        // Load optional price limits configured on the wishlist item.
        let min_price = wishlist_item
            .properties
            .get_property_value("min_price", 0i64);

        let max_price = wishlist_item
            .properties
            .get_property_value("max_price", 0i64);

        // Hidden items are either ignored or scheduled for deletion,
        // depending on their current status.
        if wishlist_item.is_hidden {
            if wishlist_item.status == StockStatus::InActive {
                log(&format!(
                    "Item {} is marked as hidden and inactive. Skipping.",
                    item_info.name
                ));
                return Ok(());
            }

            wishlist_item.set_status(StockStatus::InActive);
            wishlist_item.set_list_price(None);
            wishlist_item.locked = true;
            trade_operations.add("Delete");
        }

        // Determine the desired buy price from the current market.
        let mut post_price = if market.volume == 0 {
            trade_operations.add("NoBuyers");
            wishlist_item.set_status(StockStatus::NoBuyers);
            price.avg_price as i64
        } else {
            market.highest_price
        };

        // Apply user-defined maximum price.
        if max_price > 0 && post_price > max_price {
            post_price = max_price;
            trade_operations.add("MaxPrice");
        }

        // Apply user-defined minimum price.
        if min_price > 0 && post_price < min_price {
            post_price = min_price;
            trade_operations.add("MinPrice");
        }

        // Warframe Market prices cannot be below 1 platinum.
        post_price = post_price.max(1);

        // Update the wishlist with the calculated price and state.
        wishlist_item.set_list_price(Some(post_price));
        wishlist_item.set_status(StockStatus::Live);

        // Record the latest calculated price for historical tracking.
        if wishlist_item.status == StockStatus::Live {
            wishlist_item.add_price_history(PriceHistory::new(
                chrono::Local::now().naive_local().to_string(),
                post_price,
            ));
        }

        // Log a summary of the calculated state before progressing the order.
        log_summary(
            &component,
            format!(
                "Item {} | Post: {} | Market: {} \
                 | Price: Avg: {} | Min: {} | Max: {} | MovingAvg: {} | Median: {} \
                 | MinPrice: {} | MaxPrice: {} \
                 | Hidden: {} | Locked: {} | Status: {:?} | Ops: {:?}",
                item_info.name,
                post_price,
                market,
                price.avg_price,
                price.min_price,
                price.max_price,
                price.moving_avg.unwrap_or(0.0),
                price.median,
                min_price,
                max_price,
                wishlist_item.is_hidden,
                wishlist_item.locked,
                wishlist_item.status,
                trade_operations.operations,
            ),
            &log_options,
        );

        // Populate metadata used by the order manager.
        populate_order_properties(&mut properties, item_info, entry, &trade_operations);

        // Store the latest market metrics alongside the order.
        set_order_market_metrics(
            &mut properties,
            post_price,
            0,
            price,
            live_orders,
            OrderType::Buy,
        );

        // Create, update, or remove the live market order.
        progress_order(
            &component,
            entry,
            &wfm_client,
            OrderType::Buy,
            post_price as u32,
            per_trade,
            &log_options,
            &mut properties,
            &trade_operations,
        )
        .await
        .map_err(|e| {
            e.with_location(get_location!())
                .with_context(entry.to_json())
        })?;

        // Persist any changes made to the wishlist item.
        entry
            .finalize_wishlist_item(conn, &component, &mut wishlist_item, &log_options)
            .await?;

        Ok(())
    }

    /// Progresses the syndicate workflow for a single item.
    pub async fn progress_syndicate(
        &self,
        item_info: &CacheTradableItem,
        entry: &mut ItemEntry,
        price: &ItemPriceInfo,
        live_orders: &OrderList<OrderWithUser>,
    ) -> Result<(), Error> {
        let component = comp("Syndicate:");
        let log_options = LoggerOptions::default();
        let settings = states::get_settings()?.live_scraper;
        let syndicate_settings = &settings.syndicate;
        let wfm_client = states::app_state()?.wfm_client;

        let log = |msg: &str| info(&component, msg, &log_options);

        // Skip items that are not allowed to be sold.
        if is_blacklisted(&settings.items, item_info, entry, &TradeMode::Syndicate) {
            log(&format!(
                "Item {} is blacklisted for syndicate selling. Skipping.",
                item_info.name
            ));
            return Ok(());
        }

        let market = &entry.sell_market_info;
        let per_trade = get_per_trade(item_info);

        // Retrieve the current order state and metadata.
        let (_, current_order_price, mut properties, mut trade_operations) =
            get_order_info(entry, OrderType::Sell, &wfm_client);

        // Merge any existing properties from the entry into the order properties.
        properties.merge_properties(entry.properties.properties.clone(), true, true);

        // Start by matching the lowest active market price.
        let mut post_price = market.lowest_price;

        // Prevent large price drops that would undercut the existing order too aggressively.
        if let Some(reason) = should_apply_max_price_drop(
            syndicate_settings.wts.max_price_drop,
            syndicate_settings.wts.min_listings_below,
            current_order_price,
            post_price,
            live_orders.get_price_list(OrderType::Sell, None),
            OrderType::Sell,
        ) {
            log(&format!(
                "Item {} max price drop applied ({}).",
                item_info.name, reason
            ));

            post_price = current_order_price;
            trade_operations.add(reason);
        }

        // Warframe Market prices cannot be below 1 platinum.
        post_price = post_price.max(1);

        // Log the calculated order state before submitting it.
        log_summary(
            &component,
            format!(
                "Item {} | Post: {} | CurOrder: {} \
                 | Market: {} \
                 | Price: Avg: {} | Min: {} | Max: {} | MovingAvg: {} | Median: {} \
                 | Ops: {:?}",
                item_info.name,
                post_price,
                current_order_price,
                market,
                price.avg_price,
                price.min_price,
                price.max_price,
                price.moving_avg.unwrap_or(0.0),
                price.median,
                trade_operations.operations,
            ),
            &log_options,
        );

        // Populate metadata used by the order manager.
        populate_order_properties(&mut properties, item_info, entry, &trade_operations);

        // Store the latest market metrics alongside the order.
        set_order_market_metrics(
            &mut properties,
            post_price,
            post_price,
            price,
            live_orders,
            OrderType::Sell,
        );

        // Create, update, or remove the live market order.
        progress_order(
            &component,
            entry,
            &wfm_client,
            OrderType::Sell,
            post_price as u32,
            per_trade,
            &log_options,
            &mut properties,
            &trade_operations,
        )
        .await
        .map_err(|e| {
            e.with_location(get_location!())
                .with_context(entry.to_json())
        })?;

        Ok(())
    }
}
fn comp(suffix: &str) -> String {
    return format!("{}{}", COMPONENT, suffix);
}
