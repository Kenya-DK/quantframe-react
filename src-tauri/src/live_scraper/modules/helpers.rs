use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{atomic::Ordering, OnceLock},
};

use entity::{
    dto::{add_price_history, PriceHistory},
    stock_item::*,
    wish_list::*,
};
use serde_json::json;
use service::*;
use utils::*;
use wf_market::{
    enums::OrderType,
    types::{CreateOrderParams, Order, OrderList, OrderWithUser, UpdateOrderParams},
};

use crate::{
    app::{Settings, StockItemSettings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    enums::*,
    live_scraper::*,
    send_event,
    types::*,
    utils::{modules::states, ErrorFromExt, OrderExt, OrderListExt, SubTypeExt},
    DATABASE,
};

pub static INTERESTING_ITEMS: OnceLock<HashMap<String, Vec<ItemPriceInfo>>> = OnceLock::new();

pub fn is_disabled(value: i64) -> bool {
    value <= -1
}

pub fn get_interesting_items(settings: &StockItemSettings) -> Vec<ItemPriceInfo> {
    if let Some(items) = INTERESTING_ITEMS.get() {
        if let Some(interesting_items) = items.get(&settings.get_query_id()) {
            return interesting_items.clone();
        }
    }
    let cache = states::cache_client().expect("Failed to get cache client");

    let volume_threshold = settings.volume_threshold;
    let avg_price_cap = settings.avg_price_cap;
    let trading_tax_cap = settings.trading_tax_cap;
    let profit = settings.profit_threshold;
    let profit_margin = settings.min_wtb_profit_margin;
    let price_shift_threshold = settings.price_shift_threshold;

    // Dynamic filter using closures

    let profit_margin_filter = |item: &ItemPriceInfo| {
        is_disabled(profit_margin) || item.profit_margin >= profit_margin as f64
    };

    let volume_filter = |item: &ItemPriceInfo| {
        is_disabled(volume_threshold) || item.volume > volume_threshold as f64
    };

    let profit_filter = |item: &ItemPriceInfo| is_disabled(profit) || item.profit > profit as f64;

    let avg_price_filter =
        |item: &ItemPriceInfo| is_disabled(avg_price_cap) || item.avg_price <= avg_price_cap as f64;

    let week_price_shift_filter = |item: &ItemPriceInfo| {
        is_disabled(price_shift_threshold) || item.week_price_shift >= price_shift_threshold as f64
    };

    let trading_tax_cap_filter =
        |item: &ItemPriceInfo| is_disabled(trading_tax_cap) || item.trading_tax < trading_tax_cap;

    // Combine multiple filters dynamically
    let combined_filter = |item: &ItemPriceInfo| {
        volume_filter(item)
            && profit_filter(item)
            && avg_price_filter(item)
            && week_price_shift_filter(item)
            && trading_tax_cap_filter(item)
            && profit_margin_filter(item)
    };

    let items = cache.item_price().get_by_filter(combined_filter);
    if items.is_empty() {
        info(
            "LiveScraper:Helpers:GetInterestingItems",
            &format!(
                "No interesting items found for settings: {}",
                settings.get_query_id()
            ),
            &LoggerOptions::default(),
        );
        return vec![];
    }
    items
}

pub fn knapsack(
    items: Vec<(i64, f64, String, String)>,
    max_weight: i64,
) -> (
    Vec<(i64, f64, String, String)>,
    Vec<(i64, f64, String, String)>,
) {
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

    (selected_items, unselected_items)
}

pub fn skip_if_no_market_activity(live_orders: &OrderList<OrderWithUser>) -> (bool, String) {
    let sell_count = live_orders.sell_orders.len();
    let buy_count = live_orders.buy_orders.len();

    if sell_count == 0 || buy_count == 0 {
        let operation = if sell_count == 0 { "selling" } else { "buying" };
        return (true, operation.to_string());
    }
    (false, "".to_string())
}

pub async fn collect_interesting_items(
    component: impl Into<String>,
    settings: &Settings,
) -> Result<Vec<ItemEntry>, Error> {
    let component = component.into();
    let conn = DATABASE.get().unwrap();
    // Variables.
    let stock_item_settings = &settings.live_scraper.stock_item;
    let mut interesting_items: HashMap<String, ItemEntry> = HashMap::new();

    // -- Debugging Mode --
    if !settings.debugging.live_scraper.entries.is_empty() {
        debug(
            format!("{}Debug", component),
            "Debugging enabled for live scraper will use predefined entries",
            &LoggerOptions::default(),
        );
        return Ok(settings.debugging.live_scraper.entries.clone());
    }

    // --- Buy Mode ---
    if settings.live_scraper.has_trade_mode(TradeMode::Buy) {
        let buy_list = get_interesting_items(&settings.live_scraper.stock_item);
        for item in buy_list {
            let item_entry = ItemEntry::from(&item).set_quantity(
                OrderType::Buy,
                settings.live_scraper.stock_item.buy_quantity,
            );
            if !stock_item_settings.is_item_blacklisted(&item.wfm_id, &TradeMode::Buy) {
                interesting_items.insert(item_entry.uuid().clone(), item_entry);
            }
        }
    }

    // --- Sell Mode ---
    if settings.live_scraper.has_trade_mode(TradeMode::Sell) {
        let stock_items = StockItemQuery::get_all(conn, StockItemPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        for item in stock_items.results {
            if !stock_item_settings.is_item_blacklisted(&item.wfm_id, &TradeMode::Sell) {
                interesting_items
                    .entry(item.uuid())
                    .and_modify(|entry| {
                        entry.priority = 1;
                        entry.sell_quantity = item.owned;
                        entry.stock_id = Some(item.id);
                        entry.operation.add("Sell".to_string());
                    })
                    .or_insert_with(|| {
                        ItemEntry::from(&item).set_quantity(OrderType::Sell, item.owned)
                    });
            }
        }
    }

    // --- WishList Mode ---
    if settings.live_scraper.has_trade_mode(TradeMode::WishList) {
        let wish_items = WishListQuery::get_all(conn, WishListPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        for item in wish_items.results {
            if !stock_item_settings.is_item_blacklisted(&item.wfm_id, &TradeMode::WishList) {
                interesting_items
                    .entry(item.uuid())
                    .and_modify(|entry| {
                        entry.priority = 2;
                        entry.buy_quantity = item.quantity;
                        entry.wish_list_id = Some(item.id);
                        entry.operation.add("WishList".to_string());
                    })
                    .or_insert_with(|| ItemEntry::from(&item));
            }
        }
    }
    Ok(interesting_items.into_values().collect())
}

pub fn get_order_info(
    entry: &ItemEntry,
    order_type: OrderType,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
) -> wf_market::types::Properties {
    wfm_client
        .order()
        .cache_orders()
        .find_order(
            &entry.wfm_id,
            &SubTypeExt::from_entity(entry.sub_type.clone()),
            order_type,
        )
        .map(|order| {
            let mut properties = order.properties;
            properties.set_property_value("id", order.id.clone());
            properties
                .set_property_value("original_update_string", format!("p:{}", order.platinum));
            properties.set_property_value("operations", OperationSet::from(vec!["Update"]));
            properties
        })
        .unwrap_or_default()
}
pub fn populate_order_properties(
    properties: &mut wf_market::types::Properties,
    item: &CacheTradableItem,
    entry: &ItemEntry,
) -> (String, OperationSet) {
    properties.set_property_value("wfm_id", item.wfm_id.clone());
    properties.set_property_value("wfm_url", item.wfm_url_name.clone());
    properties.set_property_value("name", item.name.clone());
    properties.set_property_value("sub_type", entry.sub_type.clone());
    properties.set_property_value("image", item.image_url.clone());
    properties.set_property_value("t_type", item.sub_type.clone());
    let order_id = properties.get_property_value("id", String::new());
    let operations =
        properties.get_property_value("operations", OperationSet::from(vec!["Create"]));
    (order_id, operations)
}
pub fn set_order_market_metrics(
    properties: &mut wf_market::types::Properties,
    post_price: i64,
    profit: i64,
    item_price_info: &ItemPriceInfo,
    live_orders: &OrderList<OrderWithUser>,
    order_type: OrderType,
) {
    // Metrics for Highest, Lowest Sell and Buy Prices
    let sell_highest = live_orders.highest_price(OrderType::Sell);
    let sell_lowest = live_orders.lowest_price(OrderType::Sell);
    let buy_highest = live_orders.highest_price(OrderType::Buy);
    let buy_lowest = live_orders.lowest_price(OrderType::Buy);
    properties.set_property_value("update_string", format!("p:{}", post_price));
    properties.set_property_value("closed_avg", item_price_info.avg_price);

    // Use by Items Details Modal
    properties.set_property_value("potential_profit", profit);
    properties.set_property_value("sell_highest_price", sell_highest);
    properties.set_property_value("sell_lowest_price", sell_lowest);
    properties.set_property_value("buy_highest_price", buy_highest);
    properties.set_property_value("buy_lowest_price", buy_lowest);
    properties.set_property_value("supply", live_orders.sell_orders.len());
    properties.set_property_value("demand", live_orders.buy_orders.len());
    let spread = sell_lowest - buy_highest;
    properties.set_property_value("spread", spread);
    let spread_pct = if sell_lowest > 0 {
        spread as f64 / sell_lowest as f64 * 100.0
    } else {
        0.0
    };

    properties.set_property_value("spread_percent", spread_pct);
    properties.set_property_value("orders", live_orders.take_top(5, order_type));

    let mut operations = properties.get_property_value("operations", OperationSet::default());
    operations.add("MarketPopulated");
    properties.set_property_value("operations", operations);
    push_price_history(properties, post_price);
}
pub fn push_price_history(properties: &mut wf_market::types::Properties, price: i64) {
    let mut history = properties.get_property_value::<Vec<PriceHistory>>("price_history", vec![]);

    add_price_history(
        &mut history,
        PriceHistory::new(chrono::Local::now().naive_local().to_string(), price),
    );

    properties.set_property_value("price_history", history);
}
pub fn orders_to_delete(
    settings: &Settings,
    client: &LiveScraperState,
    my_orders: &OrderList<Order>,
) -> Vec<String> {
    if settings.live_scraper.auto_delete && client.just_started.load(Ordering::SeqCst) {
        return my_orders
            .order_ids(OrderType::Buy)
            .into_iter()
            .chain(my_orders.order_ids(OrderType::Sell))
            .collect();
    }

    match (
        settings.live_scraper.has_trade_mode(TradeMode::Buy),
        settings.live_scraper.has_trade_mode(TradeMode::Sell),
        settings.live_scraper.has_trade_mode(TradeMode::WishList),
    ) {
        (true, false, true) => my_orders.order_ids(OrderType::Sell),
        (false, true, false) => my_orders.order_ids(OrderType::Buy),
        _ => vec![],
    }
}
pub async fn load_orders(
    component: &str,
    client: &wf_market::Client<wf_market::Authenticated>,
    item_url: &str,
    fake_path: Option<&Path>,
) -> Result<OrderList<OrderWithUser>, Error> {
    if let Some(path) = fake_path {
        if path.exists() {
            if let Ok(cached) = utils::read_json_file(&path.to_path_buf()) {
                return Ok(cached);
            }
        }
    }

    fetch_and_cache_orders(component, client, item_url, fake_path).await
}
async fn handler_wfm_error(
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    component: &str,
    action: &str,
    message: &str,
    options: &LoggerOptions,
    e: wf_market::errors::ApiError,
) -> utils::Error {
    let log_level = match e {
        wf_market::errors::ApiError::AuctionLimitExceeded(_) => LogLevel::Warning,
        wf_market::errors::ApiError::OrderLimitExceededSamePrice(_)
        | wf_market::errors::ApiError::NotFound(_)
        | wf_market::errors::ApiError::OrderLimitExceeded(_) => {
            wfm_client.order().my_orders().await.ok();
            wfm_client
                .order()
                .cache_orders_mut()
                .apply_trade_info()
                .ok();
            trace(
                format!("{}:{}", component, action),
                "Refreshed cached orders due to order limit exceeded",
                options,
            );
            LogLevel::Warning
        }
        _ => LogLevel::Error,
    };
    let mut err = Error::from_wfm(
        format!("{}:{}", component, action),
        message.to_string(),
        e,
        get_location!(),
    );
    err = err.set_log_level(log_level);
    err
}

pub async fn progress_order(
    component: &str,
    entry: &ItemEntry,
    operations: &OperationSet,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    order_type: OrderType,
    post_price: u32,
    per_trade: Option<i64>,
    log_options: &LoggerOptions,
    properties: &wf_market::types::Properties,
) -> Result<OperationSet, Error> {
    let can_create_order = wfm_client.order().can_create_order();
    let file_name = "progress_order.log";
    let quantity = entry.get_quantity(order_type);
    // Fetch properties data
    let order_id = properties.get_property_value("id", String::new());
    let name = properties.get_property_value("name", String::new());
    let update_string = properties.get_property_value("update_string", String::new());
    let original_update_string =
        properties.get_property_value("original_update_string", String::new());

    if operations.has("Create") && !operations.has("Delete") && can_create_order {
        match wfm_client
            .order()
            .create(
                CreateOrderParams::new_with_subtype(
                    &entry.wfm_id,
                    order_type,
                    post_price,
                    quantity as u32,
                    true,
                    per_trade.map(|pt| pt as u32),
                    SubTypeExt::from_entity(entry.sub_type.clone()),
                )
                .with_properties(json!(properties.properties)),
            )
            .await
        {
            Ok(order) => {
                info(
                    format!("{}CreateSuccess", component),
                    &format!("Created order for item {}: {}", name, order.id),
                    &log_options,
                );
                send_event!(UIEvent::RefreshWfmOrders, json!({"source": component}));
            }
            Err(e) => {
                let err = handler_wfm_error(
                    wfm_client,
                    component,
                    "Create",
                    &format!("Failed to create order for item {}", name),
                    log_options,
                    e,
                )
                .await
                .with_location(get_location!())
                .log_with_options(file_name, &log_options);
                return Err(err);
            }
        }
    } else if operations.has("Update") && !operations.has("Delete") {
        match wfm_client
            .order()
            .update(
                &order_id,
                UpdateOrderParams::new()
                    .with_platinum(post_price)
                    .with_quantity(entry.get_quantity(order_type) as u32)
                    .with_per_trade(per_trade.map(|pt| pt as u32))
                    .with_properties(json!(properties.properties)),
            )
            .await
        {
            Ok(order) => {
                info(
                    format!("{}UpdateSuccess", component),
                    &format!("Updated order for item {}: {}", name, order_id),
                    &log_options,
                );
                if original_update_string != update_string {
                    send_event!(UIEvent::RefreshWfmOrders, json!({"source": component}));
                }
            }
            Err(e) => {
                let err = handler_wfm_error(
                    wfm_client,
                    component,
                    "Update",
                    &format!("Failed to update order for item {}", name),
                    log_options,
                    e,
                )
                .await
                .with_location(get_location!())
                .log_with_options(file_name, &log_options);
                return Err(err);
            }
        }
    } else if operations.has("Update") && operations.has("Delete") {
        match wfm_client.order().delete(&order_id).await {
            Ok(_) => {
                info(
                    format!("{}DeleteSuccess", component),
                    &format!("Deleted order for item {}: {}", name, order_id),
                    &log_options,
                );
                send_event!(UIEvent::RefreshWfmOrders, json!({"source": component}));
            }
            Err(e) => {
                let err = handler_wfm_error(
                    wfm_client,
                    component,
                    "Delete",
                    &format!("Failed to delete order for item {}", name),
                    log_options,
                    e,
                )
                .await
                .with_location(get_location!())
                .log_with_options(file_name, &log_options);
                return Err(err);
            }
        }
    } else if !can_create_order {
        warning(
            format!("{}Skip", component),
            &format!("Item {} has reached the order limit. Skipping.", name),
            &log_options,
        );
    } else {
        warning(
            format!("{}Skip", component),
            &format!("Item {} is not optimal for buying. Skipping.", name),
            &log_options,
        );
    }
    Ok(operations.clone())
}
pub fn log_summary(component: &str, message: impl AsRef<str>, options: &LoggerOptions) {
    info(format!("{}Summary", component), message.as_ref(), options);
}
pub async fn fetch_and_cache_orders(
    component: &str,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    item_url: &str,
    cache_path: Option<&Path>,
) -> Result<OrderList<OrderWithUser>, Error> {
    let orders = wfm_client
        .order()
        .get_orders_by_item(item_url)
        .await
        .map_err(|e| {
            let log_level = match e {
                wf_market::errors::ApiError::RequestError(_) => LogLevel::Error,
                _ => LogLevel::Critical,
            };
            Error::from_wfm(
                format!("{}:FetchAndCacheOrders", component),
                &format!("Failed to get live orders for item {}", item_url),
                e,
                get_location!(),
            )
            .set_log_level(log_level)
        })?;

    if let Some(path) = cache_path {
        utils::write_json_file(path, &orders)?;
    }

    Ok(orders)
}
