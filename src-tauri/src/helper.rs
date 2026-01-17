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

pub fn get_sounds_path() -> PathBuf {
    let sounds_path = get_app_storage_path().join("sounds");
    if !sounds_path.exists() {
        fs::create_dir_all(&sounds_path).unwrap()
    }
    sounds_path
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
<<<<<<< HEAD

pub fn loop_through_properties(data: &mut Map<String, Value>, properties: Vec<String>) {
    // Iterate over each key-value pair in the JSON object
    for (key, value) in data.iter_mut() {
        // Perform actions based on the property key or type
        match value {
            Value::Object(sub_object) => {
                // If the value is another object, recursively loop through its properties
                loop_through_properties(sub_object, properties.clone());
            }
            _ => {
                if properties.contains(&key.to_string()) {
                    *value = json!("***");
                }
            }
        }
    }
}

pub fn open_json_and_replace(path: &str, properties: Vec<String>) -> Result<Value, AppError> {
    match std::fs::File::open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let mut data: serde_json::Map<String, Value> = serde_json::from_reader(reader)
                .map_err(|e| AppError::new("Logger", eyre!(e.to_string())))
                .expect("Could not read auth.json");
            loop_through_properties(&mut data, properties.clone());
            Ok(json!(data))
        }
        Err(_) => Err(AppError::new(
            "Logger",
            eyre!("Could not open file at path: {}", path),
        )),
    }
}

pub async fn progress_wfm_order(
    wfm_id: &str,
    sub_type: Option<SubType>,
    quantity: i64,
    operation: OrderType,
    from: &str,
) -> Result<(String, Option<Order>), AppError> {
    let wfm = states::wfm_client()?;
    let notify = states::notify_client()?;
    // Process the order on WFM
    match wfm
        .orders()
        .progress_order(&wfm_id, sub_type.clone(), quantity, operation.clone())
        .await
    {
        Ok((operation, order)) => {
            if operation == "Deleted" && order.is_some() {
                add_metric("WFM_OrderDeleted", from);
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": order.clone().unwrap().id })),
                );
            } else if operation == "Updated" {
                add_metric("WFM_OrderUpdated", from);
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(order)),
                );
=======
pub async fn get_item_details(
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
        .get_by(&item_id)
        .map_err(|e| e.with_location(get_location!()))?;

    payload["item_info"] = json!(item_info);
    match cache.all_items().get_by(&item_info.unique_name) {
        Ok(mut full_item) => {
            for component in full_item.components.iter_mut() {
                component.name = format!("{} {}", full_item.name, component.name);
>>>>>>> better-backend
            }
            payload["item_info"]["components"] = json!(full_item.components);
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
<<<<<<< HEAD
    {
        Ok((operation, _)) => {
            response.push(format!("WishItem_{}", operation));
            if operation == "NotFound" {
                if !options.contains(&"WishContinueOnError".to_string()) {
                    return Err(AppError::new(
                        "WishItemCreate",
                        eyre!(format!(
                            "Wish Item not found: {} {:?}",
                            entity.wfm_url, entity.sub_type
                        )),
                    ));
                }
            }
            add_metric("Wish_ItemBought", from);
            response.push("WishItem_Bought".to_string());
        }
        Err(e) => {
            response.push("WishItemDbError".to_string());
            return Err(AppError::new("WishItemCreate", eyre!(e)));
        }
    }
    // Send Refresh Event to GUI
    notify.gui().send_event(UIEvent::RefreshWishListItems, None);
    // Process the order on WFM
    match progress_wfm_order(
        &entity.wfm_id.as_str(),
        entity.sub_type.clone(),
        entity.quantity,
        OrderType::Buy,
        from,
    )
    .await
    {
        Ok((operation, _)) => {
            response.push(format!("WFM_{}", operation));
        }
        Err(e) => {
            response.push("WFMOrderError".to_string());
            if !options.contains(&"WFMContinueOnError".to_string()) {
                return Err(e);
            }
        }
    }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((wish_item, response));
    }

    // Add Transaction to the database
    let mut transaction = wish_item.to_transaction(
        user_name,
        entity.tags.clone(),
        entity.quantity,
        entity.bought.unwrap_or(0),
        TransactionType::Purchase,
    );

    match progress_transaction(&mut transaction, from).await {
        Ok(_) => {}
        Err(e) => {
            response.push("TransactionDbError".to_string());
            return Err(e);
        }
    };
    return Ok((wish_item, response));
}

pub async fn progress_stock_item(
    entity: &mut CreateStockItem,
    validate_by: &str,
    user_name: &str,
    operation: OrderType,
    options: Vec<String>,
    progress_wfm: bool,
    from: &str,
) -> Result<(stock_item::Model, Vec<String>), AppError> {
    let conn = DATABASE.get().unwrap();
    let mut response = vec![];
    let cache = states::cache()?;
    let notify = states::notify_client()?;
    // Validate the stock item
    match cache
        .tradable_items()
        .validate_create_item(entity, validate_by)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    //Get stock item from the entity
    let stock = entity.to_model();

    // Progress the stock item based on the operation
    if operation == OrderType::Sell {
        match StockItemMutation::sold_by_url_and_sub_type(
            conn,
            &entity.wfm_url,
            entity.sub_type.clone(),
            entity.quantity,
        )
        .await
        {
            Ok((operation, _)) => {
                response.push(format!("StockItem_{}", operation));
                if operation == "NotFound" {
                    if !options.contains(&"StockContinueOnError".to_string()) {
                        return Err(AppError::new(
                            "StockItemSell",
                            eyre!(format!(
                                "Stock Item not found: {} {:?}",
                                entity.wfm_url, entity.sub_type
                            )),
                        ));
                    }
                }
                add_metric(format!("StockItem_{}", operation).as_str(), from);
            }
            Err(e) => {
                response.push("StockDbError".to_string());
                return Err(AppError::new("StockItemSell", eyre!(e)));
            }
        }
    } else if operation == OrderType::Buy {
        match StockItemMutation::add_item(conn, stock.clone()).await {
            Ok(_) => {
                let rep = "StockItem_Created".to_string();
                response.push(rep.clone());
                add_metric(rep.as_str(), from);
            }
            Err(e) => {
                response.push("StockItem_DbError".to_string());
                return Err(AppError::new("StockItemCreate", eyre!(e)));
            }
        }
    } else {
        return Err(AppError::new(
            "ProgressStockItem",
            eyre!("Invalid operation"),
        ));
    }
    // Send Refresh Event to GUI
    notify.gui().send_event(UIEvent::RefreshStockItems, None);

    // Process the order on WFM
    if progress_wfm {
        match progress_wfm_order(
            &entity.wfm_id.as_str(),
            entity.sub_type.clone(),
            entity.quantity,
            operation.clone(),
            from,
        )
        .await
        {
            Ok((operation, _)) => {
                response.push(format!("WFM_{}", operation));
            }
            Err(e) => {
                response.push("WFM_Error".to_string());
                if !options.contains(&"WFMContinueOnError".to_string()) {
                    return Err(e);
                }
            }
        }
    }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((stock, response));
    }

    // Add Transaction to the database
    let transaction_type = if operation == OrderType::Buy {
        TransactionType::Purchase
    } else {
        TransactionType::Sale
    };
    let mut transaction = stock.to_transaction(
        user_name,
        entity.tags.clone(),
        entity.quantity,
        entity.bought.unwrap_or(0),
        transaction_type,
    );

    match progress_transaction(&mut transaction, from).await {
        Ok(_) => {}
        Err(e) => {
            response.push("Transaction_DbError".to_string());
            return Err(e);
        }
    };
    return Ok((stock, response));
}

pub async fn progress_stock_riven(
    entity: &mut CreateStockRiven,
    validate_by: &str,
    user_name: &str,
    operation: OrderType,
    from: &str,
) -> Result<(stock_riven::Model, Vec<String>), AppError> {
    let conn = DATABASE.get().unwrap();
    let mut response = vec![];
    let cache = states::cache()?;
    let notify = states::notify_client()?;
    let wfm = states::wfm_client()?;
    // Validate the stock item
    match cache.riven().validate_create_riven(entity, validate_by) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    //Get stock riven from the entity
    let stock = entity.to_model();

    // Progress the stock riven based on the operation
    if operation == OrderType::Sell && entity.stock_id.is_some() {
        // Delete the stock from the database
        match StockRivenMutation::delete(conn, entity.stock_id.unwrap()).await {
            Ok(_) => {
                response.push("StockRiven_Deleted".to_string());
                add_metric("StockRiven_Deleted", from);
            }
            Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
        }
    } else if operation == OrderType::Buy {
        match StockRivenMutation::create(conn, stock.clone()).await {
            Ok(_) => {
                add_metric("StockRiven_Create", from);
                response.push("StockRivenAdd".to_string());
            }
            Err(e) => {
                response.push("StockDbError".to_string());
                let err = AppError::new_db("ProgressStockRiven", e);
                return Err(err);
            }
        }
    } else {
        return Err(AppError::new(
            "ProgressStockRiven",
            eyre!("Invalid operation"),
        ));
    }
    notify.gui().send_event(UIEvent::RefreshStockRivens, None);
    // Process the action on WFM
    if operation == OrderType::Sell && entity.wfm_order_id.is_some() {
        let id = entity.wfm_order_id.clone().unwrap();
        match wfm.auction().delete(&id).await {
            Ok(_) => {
                add_metric("WFM_RivenDeleted", from);
                response.push("WFM_RivenDeleted".to_string());
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
            Err(e) => {
                if e.cause().contains("app.form.not_exist") {
                    response.push("WFMRivenNotExist".to_string());
                }
                response.push("WFMRivenError".to_string());
            }
        }
    }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((stock, response));
    }

    // Add Transaction to the database
    let transaction_type = if operation == OrderType::Buy {
        TransactionType::Purchase
    } else {
        TransactionType::Sale
    };
    let mut transaction =
        stock.to_transaction(user_name, entity.bought.unwrap_or(0), transaction_type);

    match TransactionMutation::create(conn, &transaction).await {
        Ok(inserted) => {
            add_metric("Transaction_RivenCreate", from);
            response.push("TransactionCreated".to_string());
            transaction.id = inserted.id;
        }
        Err(e) => {
            response.push("TransactionDbError".to_string());
            return Err(AppError::new_db("StockItemCreate", e));
        }
    };
    notify.gui().send_event(UIEvent::RefreshTransactions, None);
    return Ok((stock, response));
}

// pub fn read_json_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, AppError> {
//     // Check if the file exists
//     if !path.exists() {
//         return Err(AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("File does not exist: {:?}", path.to_str())),
//         ));
//     }

//     let file = File::open(path).map_err(|e| {
//         AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("Could not open file: {}", e.to_string())),
//         )
//     })?;
//     let reader = io::BufReader::new(file);
//     let data: Value = serde_json::from_reader(reader).map_err(|e| {
//         AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("Could not read file: {}", e.to_string())),
//         )
//     })?;
//     match serde_json::from_value(data.clone()) {
//         Ok(payload) => Ok(payload),
//         Err(e) => {
//             return Err(AppError::new(
//                 "Helper:ReadJsonFile",
//                 eyre!(format!("Could not parse payload: {}", e)),
//             ));
//         }
//     }
// }
pub fn calculate_average_of_top_lowest_prices(
    prices: Vec<i64>, // The list of prices to consider (assumed to be sorted by buyout_price ascending)
    limit_to: i64,    // Limit the number of auctions to consider
    threshold_percentage: f64, // The threshold percentage to filter prices
) -> i64 {
    if prices.is_empty() {
        return -1;
    }

    // Get the top `limit_to` lowest starting prices directly, as prices is sorted
    let mut top_price: Vec<i64> = prices.into_iter().take(limit_to as usize).collect();

    // Ensure we have some prices after taking the limit
    if top_price.is_empty() {
        return -1;
    }

    // Find the minimum price in the top lowest prices.
    // Since top_price is created from a sorted list and takes the lowest,
    // the first element will be the minimum.
    let min_price = *top_price.first().unwrap_or(&0);

    // Calculate the threshold based on the minimum price.
    // Prices should not deviate by more than threshold_percentage from the min_price.
    // For example, if min_price is 100 and threshold_percentage is 0.10 (10%),
    // the threshold will be 100 * (1.0 + 0.10) = 110.
    // Any price above 110 will be filtered out.
    let threshold = min_price as f64 * (1.0 + threshold_percentage);

    // Retain prices that are less than or equal to the calculated threshold.
    top_price.retain(|&price| price <= threshold as i64);

    // Ensure we have valid prices after filtering.
    if top_price.is_empty() {
        return -1;
    }

    // Calculate and return the average price.
    top_price.iter().sum::<i64>() / top_price.len() as i64
}

pub fn group_by_date<T: Clone, F>(
    items: &[T],
    date_extractor: F,
    group_by: Vec<GroupBy>,
) -> HashMap<String, Vec<T>>
where
    F: Fn(&T) -> DateTime<Utc>,
{
    let mut map: HashMap<String, Vec<T>> = HashMap::new();

    for item in items {
        // Convert UTC to local time for grouping
        let dt_local = date_extractor(item)
            .with_timezone(&chrono::Local)
            .naive_local();

        let mut key = String::new();
        // If group_by has day
        if group_by.contains(&GroupBy::Year) {
            if !key.is_empty() {
                key.push(' ');
            }
            key.push_str(&format!("{}", dt_local.year()));
        }
        if group_by.contains(&GroupBy::Month) {
            if !key.is_empty() {
                key.push(' ');
            }
            key.push_str(&format!("{:02}", dt_local.month()));
        }
        if group_by.contains(&GroupBy::Day) {
            if !key.is_empty() {
                key.push(' ');
            }
            key.push_str(&format!("{:02}", dt_local.day()));
        }
        if group_by.contains(&GroupBy::Hour) {
            if !key.is_empty() {
                key.push(' ');
            }
            key.push_str(&format!("{:02}:00", dt_local.hour()));
        }
        map.entry(key).or_insert_with(Vec::new).push(item.clone());
    }

    map
}
pub fn get_start_of(group_by: GroupBy) -> DateTime<Utc> {
    let now = Utc::now().naive_utc();
    let date = match group_by {
        GroupBy::Hour => now
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        GroupBy::Day => now
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        GroupBy::Month => now
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap(),
        GroupBy::Year => NaiveDate::from_ymd_opt(now.year(), 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };
    DateTime::<Utc>::from_utc(date, Utc)
}

pub fn get_end_of(group_by: GroupBy) -> DateTime<Utc> {
    let now = Utc::now().naive_utc();
    let date = match group_by {
        GroupBy::Hour => now
            .with_hour(23)
            .unwrap()
            .with_minute(59)
            .unwrap()
            .with_second(59)
            .unwrap(),
        GroupBy::Day => now
            .with_hour(23)
            .unwrap()
            .with_minute(59)
            .unwrap()
            .with_second(59)
            .unwrap(),
        GroupBy::Month => {
            let last_day = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
                .unwrap()
                .with_day(0)
                .unwrap();
            last_day.and_hms_opt(23, 59, 59).unwrap()
        }
        GroupBy::Year => NaiveDate::from_ymd_opt(now.year(), 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap(),
    };
    DateTime::<Utc>::from_utc(date, Utc)
=======
    .map_err(|e| e.with_location(get_location!()))?;
    payload["report"] = json!(FinancialReport::from(&transaction_paginate.results));
    payload["last_transactions"] = json!(transaction_paginate.take_top(5));
    Ok((payload, Some(item_info), order))
>>>>>>> better-backend
}
