use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use entity::{dto::*, enums::*};
use utils::{filters_by, get_location, group_by, sorting::SortDirection, Error, Properties};
use wf_market::{enums::OrderType, types::Order};

use crate::{
    add_metric,
    app::client::AppState,
    cache::client::CacheState,
    helper::{self, paginate},
    live_scraper::LiveScraperState,
    send_event,
    types::*,
    utils::*,
};
#[tauri::command]
pub async fn order_refresh(
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<(), Error> {
    let app_state = app.lock()?.clone();
    let cache_state = cache.lock()?.clone();
    app_state
        .wfm_client
        .order()
        .my_orders()
        .await
        .map_err(|e| {
            let err = Error::from_wfm(
                "OrderRefresh",
                "Failed to refresh orders",
                e,
                get_location!(),
            );
            err.log("order_refresh.log");
            err
        })?;
    app_state
        .wfm_client
        .order()
        .cache_orders_mut()
        .apply_item_info(&cache_state)?;
    Ok(())
}

#[tauri::command]
pub fn get_wfm_orders_pagination(
    query: WfmOrderPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<PaginatedResult<Order>, Error> {
    let app = app.lock()?.clone();

    let mut filtered_orders = filters_by(&app.wfm_client.order().cache_orders().to_vec(), |o| {
        match &query.query {
            FieldChange::Value(q) => {
                let q = q.to_lowercase();
                if !o
                    .properties
                    .get_property_value("name", String::new())
                    .to_lowercase()
                    .contains(&q)
                {
                    return false;
                }
            }
            _ => {}
        }
        match &query.order_type {
            FieldChange::Value(order_type) => {
                if o.order_type != *order_type {
                    return false;
                }
            }
            _ => {}
        }

        true
    });

    match &query.sort_by {
        FieldChange::Value(sort_by) => {
            let dir = match &query.sort_direction {
                FieldChange::Value(dir) => dir,
                _ => &SortDirection::Asc,
            };
            // Only allow sorting by known columns for safety
            match sort_by.as_str() {
                "created_at" => filtered_orders.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.created_at.cmp(&b.created_at),
                    SortDirection::Desc => b.created_at.cmp(&a.created_at),
                }),
                "platinum" => filtered_orders.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.platinum.cmp(&b.platinum),
                    SortDirection::Desc => b.platinum.cmp(&a.platinum),
                }),
                "updated_at" => filtered_orders.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.updated_at.cmp(&b.updated_at),
                    SortDirection::Desc => b.updated_at.cmp(&a.updated_at),
                }),
                "order_type" => filtered_orders.sort_by(|a, b| match dir {
                    SortDirection::Asc => a.order_type.cmp(&b.order_type),
                    SortDirection::Desc => b.order_type.cmp(&a.order_type),
                }),
                _ => {}
            }
        }
        _ => {}
    }
    let p = paginate(
        &filtered_orders,
        query.pagination.page,
        query.pagination.limit,
    );

    Ok(p)
}

#[tauri::command]
pub async fn get_wfm_orders_status_counts(
    query: WfmOrderPaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<HashMap<String, (usize, i64, f64)>, Error> {
    let items = get_wfm_orders_pagination(query, app)?.results;
    let mut grouped = group_by(&items, |item| item.order_type.to_string())
        .iter()
        .map(|(status, items)| {
            (
                status.clone(),
                (
                    items.len(),
                    items
                        .iter()
                        .map(|item| (item.platinum * item.quantity) as i64)
                        .sum(),
                    items
                        .iter()
                        .map(|item| {
                            (item.properties.get_property_value("potential_profit", 0)
                                * item.quantity as i64) as f64
                        })
                        .sum(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    if grouped.get("buy").is_none() {
        grouped.insert("buy".to_string(), (0, 0, 0.0));
    }
    if grouped.get("sell").is_none() {
        grouped.insert("sell".to_string(), (0, 0, 0.0));
    }
    Ok(grouped)
}

#[tauri::command]
pub async fn order_delete_all(
    order_type: Option<OrderType>,
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app_state: tauri::State<'_, Mutex<AppState>>,
    cache_state: tauri::State<'_, Mutex<CacheState>>,
) -> Result<(), Error> {
    let app = app_state.lock()?.clone();
    order_refresh(app_state, cache_state).await?;
    live_scraper.stop();

    let orders = match order_type {
        Some(OrderType::Buy) => app.wfm_client.order().cache_orders().buy_orders,
        Some(OrderType::Sell) => app.wfm_client.order().cache_orders().sell_orders,
        _ => app.wfm_client.order().cache_orders().to_vec(),
    };
    let mut current = orders.len();
    for order in orders.iter() {
        if let Err(e) = app.wfm_client.order().delete(&order.id).await {
            let err = Error::from_wfm(
                "OrderDeleteAll",
                "Failed to delete order",
                e,
                get_location!(),
            );
            err.log("order_delete_all.log");
            return Err(err);
        }
        current -= 1;
        send_event!(
            UIEvent::OnDeleteWfmOrders,
            json!({"source": "order_delete_all", "current": current, "total": orders.len()})
        );
    }
    add_metric!("order_delete_all", "manual");
    Ok(())
}
#[tauri::command]
pub async fn order_delete_by_id(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    let order = app.wfm_client.order().cache_orders().get_by_id(&id);
    if order.is_none() {
        return Err(Error::new(
            "Command::OrderDeleteById",
            "Order not found",
            get_location!(),
        ));
    }
    let order = order.unwrap();
    match app.wfm_client.order().delete(&order.id).await {
        Ok(_) => {}
        Err(e) => {
            let err = Error::from_wfm(
                "Command::OrderDeleteById",
                "Failed to delete order",
                e,
                get_location!(),
            );
            err.log("order_delete_by_id.log");
            return Err(err);
        }
    }
    add_metric!("order_delete_by_id", "manual");
    Ok(())
}
#[tauri::command]
pub async fn get_wfm_order_by_id(
    id: String,
    operations: Option<Vec<String>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Order, Error> {
    let cache = cache.lock()?.clone();
    let app = app.lock()?.clone();
    let order = app.wfm_client.order().cache_orders().get_by_id(&id);
    if order.is_none() {
        return Err(Error::new(
            "Command::GetWfmOrderById",
            "Order not found",
            get_location!(),
        ));
    }
    let mut order = order.unwrap();
    let mut properties = Properties::default();
    helper::populate_item_market_properties(
        &mut properties,
        &order.item_id,
        order.subtype.to_entity(),
        0,
        Some(order.platinum as i64),
        OperationSet::from(
            operations.unwrap_or(
                vec!["MarketInfo", "TransactionInfo", "ProfitabilityInfo"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ),
        order.order_type,
        &cache,
        &app.wfm_client,
    )
    .await?;

    order.properties.set_properties(properties.properties);

    Ok(order)
}
