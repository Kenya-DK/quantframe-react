use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use entity::{dto::*, enums::*};
use serde_json::{json, Value};
use utils::{filters_by, get_location, group_by, Error};
use wf_market::{enums::OrderType, types::Order};

use crate::{
    add_metric,
    app::client::AppState,
    cache::client::CacheState,
    enums::*,
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
                if !o.get_details().item_name.to_lowercase().contains(&q) {
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
                        .map(|item| (item.get_details().profit * item.quantity as f64) as f64)
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

// #[tauri::command]
// pub async fn get_stock_item_financial_report(
//     query: StockItemPaginationQueryDto,
// ) -> Result<FinancialReport, Error> {
//     let items = get_stock_item_pagination(query).await?;
//     Ok(FinancialReport::from(&items.results))
// }

#[tauri::command]
pub async fn order_delete_all(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    live_scraper.stop();
    let orders = match app.wfm_client.order().my_orders().await {
        Ok(orders) => orders,
        Err(e) => {
            let err = Error::from_wfm("OrderDeleteAll", "Failed to get orders", e, get_location!());
            err.log("order_delete_all.log");
            return Err(err);
        }
    };
    let total = orders.total_orders();
    let mut current = total;
    for order in orders.to_vec() {
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
            json!({"source": "order_delete_all", "current": current, "total": total})
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
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let order = app.wfm_client.order().cache_orders().get_by_id(&id);
    if order.is_none() {
        return Err(Error::new(
            "Command::GetWfmOrderById",
            "Order not found",
            get_location!(),
        ));
    }
    let order = order.unwrap();
    let (payload, _, _) = helper::get_item_details(
        FindByType::Id,
        &order.item_id,
        order.subtype.to_entity(),
        order.order_type.clone(),
    )
    .await?;

    Ok(payload)
}
