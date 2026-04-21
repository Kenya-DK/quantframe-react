use entity::{dto::*, enums::*, stock_item::*};
use serde::{Deserialize, Serialize};
use service::StockItemMutation;
use utils::{get_location, info, warning, Error};
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
    pub flags: Vec<String>,
}

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------
fn log(
    component: &str,
    item: &CreateStockItem,
    updated_item: &Option<Model>,
    status: &str,
    flags: &OperationSet,
    operations: &OperationSet,
) {
    let log_opts = utils::LoggerOptions::default();
    let sub_component = if operations.contains("ItemSell_") {
        "SoldByUrlAndSubType"
    } else if operations.contains("ItemBuy_") {
        "BoughtByUrlAndSubType"
    } else {
        "StockItemOperation"
    };
    match (status, updated_item) {
        ("NotFound", _) => info(
            format!("{component}:{sub_component}"),
            &format!("Stock item not found for URL: {} | Operations: {:?} | Flags: {:?}", item.wfm_url, operations.operations, flags.operations),
            &log_opts.set_enable(!flags.has("DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Sold stock item {} | Owned: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                updated.item_name, updated.owned, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Deleted stock item {} | Quantity: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, item.quantity, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Updated stock item: {} | Quantity: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, item.quantity, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Created", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Created stock item: {} | Quantity: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, item.quantity, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCreatedLog")),
        ),

        ("Complete", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Completed stock item: {} | Quantity: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, item.quantity, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCompleteLog")),
        ),
        _ => {
            warning(
                format!("{component}:{sub_component}"),
                &format!(
                    "Unhandled status: {} for stock item: {} | Operations: {:?} | Flags: {:?}",
                    status, item.item_name, operations.operations, flags.operations
                ),
                &log_opts,
            );
        }
    }
}
fn should_run_wfm(flags: &OperationSet, operations: &OperationSet) -> bool {
    if let Some(value) = flags.get_value_after("SkipWFMCheck") {
        !operations.has(value)
    } else {
        true
    }
}

pub async fn handle_item_by_entity(
    mut item: CreateStockItem,
    user_name: impl Into<String>,
    order_type: OrderType,
    flags: OperationSet,
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
    match order_type {
        OrderType::Sell => {
            let (s_operation, updated_item) = StockItemMutation::sold_by_url_and_sub_type(
                con,
                &item.wfm_url,
                item.sub_type.clone(),
                item.quantity,
            )
            .await
            .map_err(|e| e.with_location(get_location!()).log(file))?;

            operations.add(format!("ItemSell_{s_operation}"));
            log(
                component,
                &item,
                &updated_item,
                &s_operation,
                &flags,
                &operations,
            );

            if let Some(updated) = updated_item {
                model = updated;
            }
        }

        OrderType::Buy => {
            let (s_operation, created_item) = StockItemMutation::add_item(con, model)
                .await
                .map_err(|e| e.with_location(get_location!()).log(file))?;

            model = created_item;
            operations.add(format!("ItemBuy_{s_operation}"));
            log(component, &item, &None, &s_operation, &flags, &operations);
        }
    }

    // --------------------------------------------------
    // WFM sync
    // --------------------------------------------------
    if should_run_wfm(&flags, &operations) {
        let status = handle_wfm_item(
            &item.wfm_id,
            &item.sub_type,
            item.quantity,
            order_type,
            OperationSet::new(),
        )
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

        operations.add(format!("WFMItem_{status}"));
    } else {
        operations.add("SkippedWFMCheck");
    }

    // --------------------------------------------------
    // Transaction
    // --------------------------------------------------
    if item.bought.unwrap_or(0) <= 0 {
        operations.add("PriceZeroNoTransaction");
        log(component, &item, &None, "Complete", &flags, &operations);
        return Ok((operations, model));
    }

    let mut tx = item.to_transaction(user_name).map_err(|e| {
        Error::new(
            "{component}:ToTransaction",
            format!("Failed to create transaction: {e}"),
            get_location!(),
        )
        .log(file)
    })?;

    if order_type == OrderType::Sell {
        tx.transaction_type = TransactionType::Sale;
    }

    handle_transaction(tx, &flags)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;
    log(component, &item, &None, "Complete", &flags, &operations);
    Ok((operations, model))
}

pub async fn handle_item(
    wfm_url: impl Into<String>,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    user_name: impl Into<String>,
    order_type: OrderType,
    flags: OperationSet,
) -> Result<(OperationSet, Model), Error> {
    handle_item_by_entity(
        CreateStockItem::new(wfm_url, sub_type.clone(), quantity).set_bought(price),
        user_name,
        order_type,
        flags,
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
            OperationSet::from(item.flags.clone()),
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
