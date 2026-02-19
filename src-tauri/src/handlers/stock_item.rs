use entity::{dto::*, enums::*, stock_item::*};
use serde::{Deserialize, Serialize};
use service::StockItemMutation;
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{handlers::*, types::OperationSet, utils::CreateStockItemExt, DATABASE};
#[derive(Serialize, Deserialize)]
pub struct ItemEntity {
    pub wfm_url: String,
    pub sub_type: Option<SubType>,
    pub quantity: i64,
    pub price: i64,
    pub user_name: String,
    pub order_type: OrderType,
    pub operation_set: Vec<String>,
}

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------
fn log_sell_result(
    component: &str,
    item: &CreateStockItem,
    updated_item: &Option<Model>,
    status: &str,
    flags: &OperationSet,
) {
    let log_opts = utils::LoggerOptions::default();

    match (status, updated_item) {
        ("NotFound", _) => info(
            format!("{component}:SoldByUrlAndSubType"),
            &format!("Stock item not found for URL: {}", item.wfm_url),
            &log_opts.set_enable(!flags.has("DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:SoldByUrlAndSubType"),
            &format!(
                "Sold stock item {} | Owned: {} | Status: {}",
                updated.item_name, updated.owned, status
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:SoldByUrlAndSubType"),
            &format!(
                "Deleted stock item {} | Quantity: {} | Status: {}",
                item.item_name, item.quantity, status
            ),
            &log_opts.set_enable(!flags.has("DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:SoldByUrlAndSubType"),
            &format!(
                "Updated stock item: {} | Quantity: {} | Status: {}",
                item.item_name, item.quantity, status
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        _ => {}
    }
}
fn log_buy_result(component: &str, item: &Model, status: &str, flags: &OperationSet) {
    let log_opts = utils::LoggerOptions::default();

    match status {
        "Created" => info(
            format!("{component}:AddItem"),
            &format!("Created stock item: {}", item.item_name),
            &log_opts.set_enable(!flags.has("DisableCreatedLog")),
        ),
        "Updated" => info(
            format!("{component}:AddItem"),
            &format!("Updated stock item: {}", item.item_name),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),
        _ => {}
    }
}
fn should_run_wfm(flags: &OperationSet, operations: &OperationSet) -> bool {
    if let Some(value) = flags.get_value_after("SkipWFMCheck") {
        !operations.has(value)
    } else {
        true
    }
}
fn create_transaction(
    item: &CreateStockItem,
    user_name: impl Into<String>,
    operation: OrderType,
    flags: &OperationSet,
    component: &str,
    file: &str,
) -> Result<entity::transaction::Model, Error> {
    let mut tx = item.to_transaction(user_name).map_err(|e| {
        Error::new(
            format!("{component}:ToTransaction"),
            format!("Failed to create transaction: {e}"),
            get_location!(),
        )
        .log(file)
    })?;

    if operation == OrderType::Sell {
        tx.transaction_type = TransactionType::Sale;
    }

    // If SetDate flag is present, parse the date and set it on the transaction
    if let Some(date) = flags.get_value_after("SetDate") {
        tx.created_at = chrono::DateTime::parse_from_rfc3339(&date)
            .map_err(|e| {
                Error::new(
                    format!("{component}:ParseDate"),
                    format!("Failed to parse date: {e}"),
                    get_location!(),
                )
                .log(file)
            })?
            .with_timezone(&chrono::Utc);
    }

    Ok(tx)
}

pub async fn handle_item_by_entity(
    mut item: CreateStockItem,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: OperationSet,
) -> Result<(OperationSet, Model), Error> {
    let con = DATABASE.get().unwrap();
    let component = "HandleItem";
    let file = "handle_item.log";

    let mut operations = OperationSet::new();

    // --------------------------------------------------
    // Validate
    // --------------------------------------------------
    item.validate().map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(file);
        e
    })?;

    let mut model = item.to_model();

    // --------------------------------------------------
    // Stock mutation (buy / sell)
    // --------------------------------------------------
    match operation {
        OrderType::Sell => {
            let (s_operation, updated_item) = StockItemMutation::sold_by_url_and_sub_type(
                con,
                &item.wfm_url,
                item.sub_type.clone(),
                item.quantity,
            )
            .await
            .map_err(|e| e.with_location(get_location!()).log(file))?;

            log_sell_result(
                component,
                &item,
                &updated_item,
                &s_operation,
                &operation_flags,
            );

            if let Some(updated) = updated_item {
                model = updated;
            }

            operations.add(format!("ItemSell_{s_operation}"));
        }

        OrderType::Buy => {
            let (s_operation, created_item) = StockItemMutation::add_item(con, model)
                .await
                .map_err(|e| e.with_location(get_location!()).log(file))?;

            log_buy_result(component, &created_item, &s_operation, &operation_flags);

            model = created_item;
            operations.add(format!("ItemBuy_{s_operation}"));
        }
    }

    // --------------------------------------------------
    // WFM sync
    // --------------------------------------------------
    if should_run_wfm(&operation_flags, &operations) {
        let status = handle_wfm_item(
            &item.wfm_id,
            &item.sub_type,
            item.quantity,
            operation,
            false,
        )
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

        operations.add(format!("WFMItem_{status}"));
    } else {
        operations.add("SkippedWFMCheck");
    }

    // --------------------------------------------------
    // Transaction creation
    // --------------------------------------------------
    if item.bought.unwrap_or(0) <= 0 {
        return Ok((operations, model));
    }

    let mut transaction = create_transaction(
        &item,
        user_name,
        operation,
        &operation_flags,
        component,
        file,
    )?;

    handle_transaction(transaction, !operation_flags.has("SetDate"))
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    Ok((operations, model))
}

pub async fn handle_item(
    wfm_url: impl Into<String>,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: OperationSet,
) -> Result<(OperationSet, Model), Error> {
    handle_item_by_entity(
        CreateStockItem::new(wfm_url, sub_type.clone(), quantity).set_bought(price),
        user_name,
        operation,
        operation_flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}

pub async fn handle_items(
    items: Vec<ItemEntity>,
) -> Result<(i32, Vec<(OperationSet, String)>), Error> {
    let mut total = 0;
    let mut processed_items = Vec::new();
    for item in items {
        match handle_item(
            item.wfm_url,
            item.sub_type,
            item.quantity,
            item.price,
            item.user_name,
            item.order_type,
            OperationSet::from(item.operation_set.clone()),
        )
        .await
        {
            Ok((o, updated_item)) => {
                total += 1;
                processed_items.push((o, updated_item.item_name));
            }
            Err(e) => {
                return Err(e.with_location(get_location!()));
            }
        }
    }
    Ok((total, processed_items))
}
