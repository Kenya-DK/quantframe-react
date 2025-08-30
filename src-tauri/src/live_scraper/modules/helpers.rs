use std::{collections::HashMap, sync::OnceLock};

use entity::{stock_item::*, wish_list::*};
use serde_json::json;
use service::*;
use utils::*;
use wf_market::{
    enums::OrderType,
    types::{CreateOrderParams, OrderList, OrderWithUser, UpdateOrderParams},
};

use crate::{
    app::{Settings, StockItemSettings},
    cache::types::{CacheTradableItem, ItemPriceInfo},
    enums::*,
    live_scraper::*,
    notification::enums::UIEvent,
    send_event,
    utils::{modules::states, order_ext::OrderDetails, ErrorFromExt, OrderExt, SubTypeExt},
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

    let order_type = |item: &ItemPriceInfo| item.order_type == "closed";

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
        order_type(item)
            && volume_filter(item)
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
            let item_entry = ItemEntry::from(&item)
                .set_buy_quantity(settings.live_scraper.stock_item.buy_quantity);
            interesting_items.insert(item_entry.uuid().clone(), item_entry);
        }
    }

    // --- Sell Mode ---
    if settings.live_scraper.has_trade_mode(TradeMode::Sell) {
        let stock_items = StockItemQuery::get_all(conn, StockItemPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| {
                Error::from_db(
                    "ItemModule::check",
                    "Failed to get stock items: {}",
                    e,
                    get_location!(),
                )
            })?;
        for item in stock_items.results {
            interesting_items
                .entry(item.uuid())
                .and_modify(|entry| {
                    entry.priority = 1;
                    entry.sell_quantity = item.owned;
                    entry.stock_id = Some(item.id);
                    entry.operation.push("Sell".to_string());
                })
                .or_insert_with(|| ItemEntry::from(&item).set_sell_quantity(item.owned));
        }
    }

    // --- WishList Mode ---
    if settings.live_scraper.has_trade_mode(TradeMode::WishList) {
        let wish_items = WishListQuery::get_all(conn, WishListPaginationQueryDto::new(1, -1))
            .await
            .map_err(|e| {
                Error::from_db(
                    "ItemModule::check",
                    "Failed to get wish list items: {}",
                    e,
                    get_location!(),
                )
            })?;
        for item in wish_items.results {
            interesting_items
                .entry(item.uuid())
                .and_modify(|entry| {
                    entry.priority = 2;
                    entry.buy_quantity = item.quantity;
                    entry.wish_list_id = Some(item.id);
                    entry.operation.push("WishList".to_string());
                })
                .or_insert_with(|| ItemEntry::from(&item));
        }
    }
    Ok(interesting_items.into_values().collect())
}

pub fn get_order_info(
    item_info: &CacheTradableItem,
    entry: &ItemEntry,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    order_type: OrderType,
) -> OrderDetails {
    let quantity = if order_type == OrderType::Buy {
        entry.buy_quantity
    } else {
        entry.sell_quantity
    };
    wfm_client
        .order()
        .cache_orders()
        .find_order(
            &item_info.wfm_id,
            &SubTypeExt::from_entity(entry.sub_type.clone()),
            order_type,
        )
        .map(|order| {
            order
                .get_details()
                .set_operation(&["Update"])
                .set_order_id(&order.id)
        })
        .unwrap_or_default()
        .set_item_id(&item_info.wfm_id)
        .set_quantity(quantity as u32)
        .set_sub_type(entry.sub_type.clone())
        .set_info(item_info)
}

pub async fn progress_order(
    component: &str,
    wfm_client: &wf_market::Client<wf_market::Authenticated>,
    order_info: &OrderDetails,
    order_type: OrderType,
    post_price: u32,
    per_trade: Option<u32>,
    log_options: &LoggerOptions,
) -> Result<(), Error> {
    let can_create_order = wfm_client.order().can_create_order();
    if order_info.has_operation("Create") && !order_info.has_operation("Delete") && can_create_order
    {
        match wfm_client
            .order()
            .create(
                CreateOrderParams::new_with_subtype(
                    &order_info.item_id,
                    order_type,
                    post_price,
                    order_info.quantity,
                    true,
                    per_trade,
                    SubTypeExt::from_entity(order_info.sub_type.clone()),
                )
                .with_properties(json!(order_info)),
            )
            .await
        {
            Ok(order) => {
                info(
                    format!("{}CreateSuccess", component),
                    &format!(
                        "Created order for item {}: {}",
                        order_info.item_name, order.id
                    ),
                    &log_options,
                );
                send_event!(UIEvent::RefreshWfmOrders, json!({"source": component}));
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}CreateFail", component),
                    format!("Failed to create order for item {}", order_info.item_name),
                    e,
                    get_location!(),
                ));
            }
        }
    } else if order_info.has_operation("Update") && !order_info.has_operation("Delete") {
        match wfm_client
            .order()
            .update(
                &order_info.order_id,
                UpdateOrderParams::new()
                    .with_platinum(post_price)
                    .with_quantity(order_info.quantity)
                    .with_per_trade(per_trade)
                    .with_properties(json!(order_info)),
            )
            .await
        {
            Ok(_) => {
                info(
                    format!("{}UpdateSuccess", component),
                    &format!(
                        "Updated order for item {}: {}",
                        order_info.item_name, order_info.order_id
                    ),
                    &log_options,
                );
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}UpdateFail", component),
                    format!("Failed to update order for item {}", order_info.item_name),
                    e,
                    get_location!(),
                ));
            }
        }
    } else if order_info.has_operation("Update") && order_info.has_operation("Delete") {
        match wfm_client.order().delete(&order_info.order_id).await {
            Ok(_) => {
                info(
                    format!("{}DeleteSuccess", component),
                    &format!(
                        "Deleted order for item {}: {}",
                        order_info.item_name, order_info.order_id
                    ),
                    &log_options,
                );
                send_event!(UIEvent::RefreshWfmOrders, json!({"source": component}));
            }
            Err(e) => {
                return Err(Error::from_wfm(
                    format!("{}::DeleteFail", component),
                    format!("Failed to delete order for item {}", order_info.item_name),
                    e,
                    get_location!(),
                ));
            }
        }
    } else if !can_create_order {
        warning(
            format!("{}Skip", component),
            &format!(
                "Item {} has reached the order limit. Skipping.",
                order_info.item_name
            ),
            &log_options,
        );
    } else {
        warning(
            format!("{}Skip", component),
            &format!(
                "Item {} is not optimal for buying. Skipping.",
                order_info.item_name
            ),
            &log_options,
        );
    }
    Ok(())
}
