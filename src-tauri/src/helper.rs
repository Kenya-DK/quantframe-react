use chrono::{DateTime, Utc};
use entity::{
    dto::{FinancialGraph, FinancialReport, PaginatedResult, SubType},
    transaction::TransactionPaginationQueryDto,
};
use serde_json::json;
use service::TransactionQuery;
use std::{
    fs::{self},
    path::PathBuf,
};
use tauri::Manager;
use utils::*;
use wf_market::{enums::OrderType, types::Order};

use crate::{
    cache::CacheTradableItem,
    enums::{FindBy, FindByType},
    utils::{modules::states, OrderExt, SubTypeExt},
    APP, DATABASE,
};

pub static APP_PATH: &str = "dev.kenya.quantframe";

pub fn get_device_id() -> String {
    let app = APP.get().unwrap();
    let home_dir = match app.path().home_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find home directory");
        }
    };
    let device_name = home_dir.file_name().unwrap().to_str().unwrap();
    device_name.to_string()
}
pub fn get_app_storage_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find app path");
        }
    };

    let app_path = local_path.join(APP_PATH);
    if !app_path.exists() {
        fs::create_dir_all(&app_path).unwrap()
    }
    app_path
}
pub fn get_desktop_path() -> PathBuf {
    let app = APP.get().unwrap();
    let desktop_path = match app.path().desktop_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find desktop path");
        }
    };
    desktop_path
}
pub fn generate_transaction_summary(
    transactions: &Vec<entity::transaction::Model>,
    date: DateTime<Utc>,
    group_by1: GroupByDate,
    group_by2: &[GroupByDate],
    _previous: bool,
) -> (FinancialReport, FinancialGraph<i64>) {
    let (start, end) = get_start_end_of(date, group_by1);
    let transactions = filters_by(transactions, |t| {
        t.created_at >= start && t.created_at <= end
    });

    let mut grouped = group_by_date(&transactions, |t| t.created_at, group_by2);

    fill_missing_date_keys(&mut grouped, start, end, group_by2);

    let graph = FinancialGraph::<i64>::from(&grouped, |group| {
        FinancialReport::from(&group.to_vec()).total_profit
    });
    (FinancialReport::from(&transactions), graph)
}

/// Paginate a vector of items
pub fn paginate<T: Clone>(items: &[T], page: i64, per_page: i64) -> PaginatedResult<T> {
    let total_items = items.len() as i64;

    let start = (page.saturating_sub(1)) * per_page;
    let end = (start + per_page).min(total_items);

    let start_usize = start as usize;
    let end_usize = end as usize;

    let page_items = if start < total_items && end > 0 {
        items[start_usize..end_usize].to_vec()
    } else if per_page == -1 {
        items.to_vec()
    } else {
        Vec::new()
    };
    let total_pages = if per_page == -1 {
        1
    } else {
        (total_items as f64 / per_page as f64).ceil() as i64
    };
    PaginatedResult {
        results: page_items,
        page,
        limit: per_page,
        total: total_items,
        total_pages,
    }
}

pub fn get_local_data_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find local data path");
        }
    };
    local_path
}
pub async fn get_item_details(
    find_by: FindByType,
    raw: impl Into<String>,
    sub_type: Option<SubType>,
    order_type: OrderType,
) -> Result<(serde_json::Value, Option<CacheTradableItem>, Option<Order>), Error> {
    let item_id = raw.into();
    let app = states::app_state()?.clone();
    let cache = states::cache_client()?.clone();
    let conn = DATABASE.get().unwrap();

    let mut payload = json!({});
    // Get item details from cache
    let item_info = cache
        .tradable_item()
        .get_by(FindBy::new(find_by.clone(), &item_id))?;
    if item_info.is_none() {
        warning(
            "Helper::GetItemDetails",
            &format!("Item not found in cache for {:?}: {}", find_by, item_id),
            &LoggerOptions::default().set_file("helper.log"),
        );
    }
    let item_info = item_info.unwrap();
    payload["item_info"] = json!(item_info);
    match cache
        .all_items()
        .get_by(FindBy::new(FindByType::UniqueName, &item_info.unique_name))
    {
        Ok(full_item) => {
            if let Some(mut full_item) = full_item {
                for component in full_item.components.iter_mut() {
                    component.name = format!("{} {}", full_item.name, component.name);
                }
                payload["item_info"]["components"] = json!(full_item.components);
            }
        }
        Err(_) => {
            warning(
                "Command::GetItemDetails",
                &format!(
                    "Full item not found for unique name: {}",
                    item_info.unique_name
                ),
                &LoggerOptions::default(),
            );
        }
    }

    // Get Order Info from WFM
    let order = app.wfm_client.order().cache_orders().find_order(
        &item_info.wfm_id,
        &SubTypeExt::from_entity(sub_type.clone()),
        order_type,
    );
    if let Some(mut order_info) = order.clone() {
        let mut details = order_info.get_details();
        let mut orders = details.orders;
        if !orders.is_empty() {
            for ord in orders.iter_mut() {
                ord.order.apply_item_info(&cache)?;
            }
            details.orders = orders;
            order_info.update_details(details);
        }
        payload["order_info"] = json!(order_info);
    }

    // Get Transaction Summary
    let transaction_paginate = TransactionQuery::get_all(
        conn,
        TransactionPaginationQueryDto::new(1, -1)
            .set_wfm_id(&item_info.wfm_id)
            .set_sub_type(sub_type.clone()),
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    payload["report"] = json!(FinancialReport::from(&transaction_paginate.results));
    payload["last_transactions"] = json!(transaction_paginate.take_top(5));
    Ok((payload, Some(item_info), order))
}
