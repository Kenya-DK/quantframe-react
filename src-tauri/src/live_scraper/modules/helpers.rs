use crate::live_scraper::types::cooldown::CooldownInfo;
use std::{
    collections::HashMap,
    path::Path,
    sync::{atomic::Ordering, Mutex, OnceLock},
    time::Duration,
    vec,
};

use chrono::{DateTime, Utc};
use entity::{
    dto::{add_price_history, PriceHistory},
    enums::StockStatus,
    stock_item::*,
    syndicate_item::SyndicateItemPaginationQueryDto,
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
    app::{AppState, ItemSettings, Settings, SyndicateSettings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    enums::*,
    live_scraper::*,
    send_event,
    types::*,
    utils::{modules::states, ErrorFromExt, OrderListExt, SubTypeExt},
    DATABASE,
};

pub static INTERESTING_ITEMS: OnceLock<HashMap<String, Vec<ItemPriceInfo>>> = OnceLock::new();

const ORDER_COOLDOWN_ENABLED: bool = false; // Set to true to enable order cooldowns, false to disable
static ORDER_COOL_DOWNS: OnceLock<Mutex<HashMap<String, CooldownInfo>>> = OnceLock::new();
const ORDER_SAME_PRICE_COOL_DOWN: Duration = Duration::from_secs(20 * 60);
const ORDER_PRICE_CHANGE_COOL_DOWN: Duration = Duration::from_secs(5 * 60);

pub fn is_disabled(value: i64) -> bool {
    value <= -1
}

pub fn get_interesting_items(settings: &ItemSettings) -> Vec<ItemPriceInfo> {
    if let Some(items) = INTERESTING_ITEMS.get() {
        if let Some(interesting_items) = items.get(&settings.get_query_id()) {
            return interesting_items.clone();
        }
    }
    let cache = states::cache_client().expect("Failed to get cache client");

    let volume_threshold = settings.wtb.volume_threshold;
    let avg_price_cap = settings.wtb.avg_price_cap;
    let trading_tax_cap = settings.wtb.trading_tax_cap;
    let profit = settings.wtb.profit_threshold;
    let profit_margin = settings.wtb.min_wtb_profit_margin;
    let price_shift_threshold = settings.wtb.price_shift_threshold;

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

pub async fn collect_interesting_items(
    app: &AppState,
    component: impl Into<String>,
    settings: &Settings,
) -> Result<Vec<ItemEntry>, Error> {
    let component = component.into();
    let conn = DATABASE.get().unwrap();
    // Variables.
    let stock_item_settings = &settings.live_scraper.items;
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
        let buy_list = get_interesting_items(&settings.live_scraper.items);
        for item in buy_list {
            let item_entry = ItemEntry::from(&item)
                .set_quantity(OrderType::Buy, settings.live_scraper.items.wtb.buy_quantity);
            if !stock_item_settings.general.is_item_blacklisted(
                &item.wfm_id,
                &item.sub_type,
                &TradeMode::Buy,
            ) {
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
            if !stock_item_settings.general.is_item_blacklisted(
                &item.wfm_id,
                &item.sub_type,
                &TradeMode::Sell,
            ) {
                interesting_items
                    .entry(item.uuid())
                    .and_modify(|entry| {
                        entry.priority = 1;
                        entry.sell_quantity = item.owned;
                        entry.stock_id = Some(item.id);
                        entry.operations.add("Sell".to_string());
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
            if !stock_item_settings.general.is_item_blacklisted(
                &item.wfm_id,
                &item.sub_type,
                &TradeMode::WishList,
            ) {
                interesting_items
                    .entry(item.uuid())
                    .and_modify(|entry| {
                        entry.priority = 2;
                        entry.buy_quantity = item.quantity;
                        entry.wish_list_id = Some(item.id);
                        entry.operations.add("WishList".to_string());
                    })
                    .or_insert_with(|| ItemEntry::from(&item));
            }
        }
    }

    // --- Syndicate Mode --- Disabled for now, WIP
    if settings.live_scraper.has_trade_mode(TradeMode::Syndicate) {
        let Ok(_) = app
            .user
            .has_permission(PermissionsFlags::from_str("syndicate_prices_search"))
        else {
            return Ok(interesting_items.into_values().collect());
        };
        let items = SyndicateItemQuery::get_all(conn, SyndicateItemPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        for item in items.results {
            interesting_items
                .entry(item.uuid())
                .and_modify(|entry| {
                    if entry.sell_quantity == 0 {
                        entry.sell_quantity = 1;
                    }
                    entry.syndicate_id = Some(item.id);
                    entry.operations.add("Syndicate".to_string());
                })
                .or_insert_with(|| ItemEntry::from(&item).set_quantity(OrderType::Sell, 1));
        }

        // let items = get_syndicate_interesting_items(&app, &settings.live_scraper.syndicate)
        //     .await
        //     .map_err(|e| e.with_location(get_location!()))?;

        // for item in items.into_iter().filter(|item| {
        //     !stock_item_settings.general.is_item_blacklisted(
        //         &item.wfm_id,
        //         &item.sub_type,
        //         &TradeMode::Syndicate,
        //     )
        // }) {
        //     interesting_items
        //         .entry(item.uuid.clone())
        //         .and_modify(|entry| entry.operations.add("Syndicate".to_string()))
        //         .or_insert_with(|| ItemEntry::from(&item));
        // }
    }
    Ok(interesting_items.into_values().collect())
}

pub fn get_order_info(
    entry: &ItemEntry,
    order_type: OrderType,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
) -> (String, i64, wf_market::types::Properties, OperationSet) {
    let order = wfm_client.order().cache_orders().find_order(
        &entry.wfm_id,
        &SubTypeExt::from_entity(entry.sub_type.clone()),
        order_type,
    );
    if order.is_none() {
        (
            String::new(),
            0,
            wf_market::types::Properties::default(),
            OperationSet::from(vec!["Create"]),
        )
    } else {
        let order = order.unwrap();
        let mut properties = order.properties;
        properties.set_property_value("id", order.id.clone());
        properties.set_property_value("old_price", order.platinum);
        properties.set_property_value("original_update_string", format!("p:{}", order.platinum));
        (
            order.id.clone(),
            i64::from(order.platinum),
            properties,
            OperationSet::from(vec!["Update"]),
        )
    }
}
pub fn populate_order_properties(
    properties: &mut wf_market::types::Properties,
    item: &CacheTradableItem,
    entry: &ItemEntry,
    trade_operations: &OperationSet,
) {
    properties.set_property_value("wfm_id", item.wfm_id.clone());
    properties.set_property_value("wfm_url", item.wfm_url.clone());
    properties.set_property_value("name", item.name.clone());
    properties.set_property_value("sub_type", entry.sub_type.clone());
    properties.set_property_value("image", item.icon.clone());
    properties.set_property_value("t_type", item.sub_type.clone());
    let mut operations = entry.operations.clone();
    operations.merge(trade_operations);
    properties.set_property_value("operations", operations);
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

    // let mut operations = properties.get_property_value("operations", OperationSet::default());
    // operations.add("MarketPopulated");
    // properties.set_property_value("operations", operations);
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
    if settings.live_scraper.general.auto_delete && client.just_started.load(Ordering::SeqCst) {
        let mut ids = vec![];
        for item in my_orders.to_vec() {
            let mode = match item.order_type {
                OrderType::Buy => TradeMode::Buy,
                OrderType::Sell => TradeMode::Sell,
            };

            if !settings.live_scraper.items.general.is_item_blacklisted(
                &item.item_id,
                &SubTypeExt::to_entity(&item.subtype),
                &mode,
            ) {
                ids.push(item.id.clone());
            }
        }

        return ids;
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

pub fn get_cooldown(new: &Properties) -> CooldownInfo {
    let current_cooldown = new.get_property_value("cooldown", CooldownInfo::default());
    current_cooldown
}
pub fn set_cooldown(current: &mut Properties, new: &Properties) -> (bool, CooldownInfo) {
    let current_cooldown = get_cooldown(current);
    let new_cooldown = get_cooldown(new);
    if current_cooldown.start_time != new_cooldown.start_time {
        current.set_property_value("cooldown", new_cooldown.clone());
        return (true, new_cooldown);
    }
    (false, current_cooldown)
}
fn is_order_on_cooldown(
    order_id: impl Into<String>,
    old_price: u32,
    new_price: u32,
) -> Option<CooldownInfo> {
    if !ORDER_COOLDOWN_ENABLED {
        return None;
    }
    let order_id = order_id.into();
    let now = Utc::now();

    let (cooldown_type, duration) = if old_price == new_price {
        ("order_same_price", ORDER_SAME_PRICE_COOL_DOWN)
    } else {
        ("order_price_change", ORDER_PRICE_CHANGE_COOL_DOWN)
    };
    // Start a new cooldown
    let cooldown = CooldownInfo::new(order_id, now, cooldown_type, duration);
    let key = cooldown.key();

    let cooldowns = ORDER_COOL_DOWNS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cooldowns = cooldowns.lock().unwrap();

    // Check existing cooldown
    if let Some(cooldown) = cooldowns.get(&key) {
        if cooldown.remaining().is_some() {
            return Some(cooldown.clone());
        }

        // Cooldown has expired
        cooldowns.remove(&key);
    }

    cooldowns.insert(key, cooldown);

    None
}
pub async fn progress_order(
    component: &str,
    entry: &ItemEntry,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    order_type: OrderType,
    post_price: u32,
    per_trade: Option<i64>,
    log_options: &LoggerOptions,
    properties: &mut wf_market::types::Properties,
    trade_operations: &OperationSet,
) -> Result<OperationSet, Error> {
    let can_create_order = wfm_client.order().can_create_order();
    let quantity = entry.get_quantity(order_type);
    // Fetch properties data
    let order_id = properties.get_property_value("id", String::new());
    let old_price = properties.get_property_value("old_price", 0u32);
    let name = properties.get_property_value("name", String::new());
    let update_string = properties.get_property_value("update_string", String::new());
    let original_update_string =
        properties.get_property_value("original_update_string", String::new());

    if trade_operations.has("Create") && !trade_operations.has("Delete") && can_create_order {
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
                .with_location(get_location!());
                return Err(err);
            }
        }
    } else if trade_operations.has("Update") && !trade_operations.has("Delete") {
        if let Some(info) = is_order_on_cooldown(&order_id, old_price, post_price) {
            properties.set_property_value("cooldown", info.clone());
            wfm_client.order().cache_orders_mut().update(
                order_id,
                UpdateOrderParams::new().with_properties(json!(properties.properties)),
            );
            return Err(Error::new(
                format!("{}:Cooldown", component),
                format!(
                    "Order for item {} is on cooldown ({}). Skipping update.",
                    name,
                    info.format_remaining()
                ),
                get_location!(),
            )
            .with_cause("CooldownError")
            .with_context(json!({"cooldown": info})));
        }

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
                    &format!("Updated order for item {}: {}", name, order.id),
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
                .with_location(get_location!());
                return Err(err);
            }
        }
    } else if trade_operations.has("Update") && trade_operations.has("Delete") {
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
                .with_location(get_location!());
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
            &format!("Item {} has no trade operations. Skipping.", name),
            &log_options,
        );
    }
    Ok(OperationSet::default())
}
pub async fn delete_order(
    component: &str,
    entry: &ItemEntry,
    order_type: OrderType,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
) -> Result<OperationSet, Error> {
    progress_order(
        component,
        entry,
        wfm_client,
        order_type,
        1,
        Some(1),
        &LoggerOptions::default(),
        &mut wf_market::types::Properties::default(),
        &OperationSet::default(),
    )
    .await
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
            Error::from_wfm(
                format!("{}:FetchAndCacheOrders", component),
                &format!("Failed to get live orders for item {}", item_url),
                e,
                get_location!(),
            )
        })?;

    if let Some(path) = cache_path {
        utils::write_json_file(path, &orders)?;
    }

    Ok(orders)
}

pub fn get_per_trade(bulk_tradable: bool, quantity: i64, is_bulk: bool) -> Option<i64> {
    if !bulk_tradable {
        return None;
    }

    if !is_bulk {
        return Some(1);
    }

    let max_per_trade = quantity.clamp(1, 6);
    (1..=max_per_trade)
        .rev()
        .find(|per_trade| quantity % per_trade == 0)
}

pub fn is_blacklisted(
    settings: &ItemSettings,
    item_info: &CacheTradableItem,
    entry: &ItemEntry,
    mode: &TradeMode,
) -> bool {
    if settings
        .general
        .is_item_blacklisted(&item_info.wfm_id, &entry.sub_type, mode)
    {
        return true;
    }
    false
}

pub fn should_apply_max_price_drop(
    max_price_drop: i64,
    min_listings_below: i64,
    current_order_price: i64,
    post_price: i64,
    prices: Vec<i64>,
    order_type: OrderType,
) -> Option<String> {
    if is_disabled(max_price_drop) && is_disabled(min_listings_below) {
        return None;
    }
    let (is_price_invalid, price_change, listing_count) = match order_type {
        OrderType::Buy => (
            current_order_price > post_price,
            post_price - current_order_price,
            prices.iter().filter(|&&p| p > current_order_price).count() as i64,
        ),
        OrderType::Sell => (
            current_order_price < post_price,
            current_order_price - post_price,
            prices.iter().filter(|&&p| p < current_order_price).count() as i64,
        ),
    };

    if is_price_invalid {
        return None;
    }

    let should_skip = !is_disabled(max_price_drop)
        && price_change > max_price_drop
        && (is_disabled(min_listings_below) || listing_count <= min_listings_below);

    if should_skip {
        Some("MaxPriceDrop".to_string())
    } else {
        None
    }
}

pub fn minimum_profit_threshold(bought_price: i64, flat_profit: i64, percentage: i64) -> i64 {
    if is_disabled(percentage) {
        flat_profit
    } else {
        flat_profit.max(bought_price.saturating_mul(percentage).saturating_add(99) / 100)
    }
}
