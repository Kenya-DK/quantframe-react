use entity::{dto::*, enums::*, stock_item::*};
use service::StockItemMutation;
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{enums::*, handlers::*, types::OperationSet, utils::CreateStockItemExt, DATABASE};

pub async fn handle_item_by_entity(
    mut item: CreateStockItem,
    user_name: impl Into<String>,
    operation: OrderType,
    find_by: FindByType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleItem";
    let file = "handle_item.log";
    let mut operations = OperationSet::new();
    // Create and validate the item

    item.validate(FindBy::new(find_by, item.raw.clone()))
        .map_err(|e| {
            let err = e.clone();
            err.with_location(get_location!()).log(file);
            e
        })?;

    let mut model = item.to_model();

    // Handle StockItem creation, deletion, or update
    if operation == OrderType::Sell {
        // Handle sell operation
        match StockItemMutation::sold_by_url_and_sub_type(
            conn,
            &item.wfm_url,
            item.sub_type.clone(),
            item.quantity,
        )
        .await
        {
            Ok((s_operation, updated_item)) => {
                if s_operation == "NotFound" {
                    info(
                        format!("{}:SoldByUrlAndSubType", component),
                        &format!(
                            "Stock item not found for URL: {}, status: {}",
                            item.wfm_url, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableNotFoundLog")),
                    );
                } else if let Some(item) = updated_item {
                    info(
                        format!("{}:SoldByUrlAndSubType", component),
                        &format!(
                            "Sold stock item: {}, owned: {}, status: {}",
                            item.item_name, item.owned, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableUpdatedLog")),
                    );
                    model = item;
                } else if s_operation == "Deleted" {
                    info(
                        format!("{}:SoldByUrlAndSubType", component),
                        &format!(
                            "Deleted stock item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableDeletedLog")),
                    );
                } else if s_operation == "Updated" {
                    info(
                        format!("{}:SoldByUrlAndSubType", component),
                        &format!(
                            "Updated stock item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableUpdatedLog")),
                    );
                }
                operations.add(format!("ItemSell_{}", s_operation));
            }
            Err(e) => return Err(e.with_location(get_location!()).log(file)),
        }
    } else if operation == OrderType::Buy {
        // Handle buy operation
        match StockItemMutation::add_item(conn, model).await {
            Ok((s_operation, created_item)) => {
                if s_operation == "Created" {
                    info(
                        format!("{}:AddItem", component),
                        &format!("Created stock item: {}", created_item.item_name),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableCreatedLog")),
                    );
                } else if s_operation == "Updated" {
                    info(
                        "HandleItem:AddItem",
                        &format!("Updated stock item: {}", created_item.item_name),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableUpdatedLog")),
                    );
                }
                model = created_item;
                operations.add(format!("ItemBuy_{}", s_operation));
            }
            Err(e) => return Err(e.with_location(get_location!()).log(file)),
        }
    }

    if operation_flags.iter().any(|op| op.starts_with("ReturnOn:")) {
        let return_on = operation_flags
            .iter()
            .filter(|op| op.starts_with("ReturnOn:"))
            .cloned()
            .collect::<Vec<_>>();
        if return_on.len() > 0 {
            let return_on = return_on[0].replace("ReturnOn:", "");
            if operations.ends_with(&return_on) {
                return Ok((operations, model));
            }
        }
    }

    match handle_wfm_item(
        &item.wfm_id,
        &item.sub_type,
        item.quantity,
        operation,
        false,
    )
    .await
    {
        Ok(operation_status) => {
            operations.add(format!("WFMItem_{}", operation_status));
        }
        Err(e) => {
            return Err(e.with_location(get_location!()).log(file));
        }
    }

    // Create a transaction from the item
    if !item.is_validated {
        return Err(Error::new(
            component,
            "Stock item is not validated yet",
            get_location!(),
        )
        .log(file));
    }
    if item.bought.unwrap_or(0) <= 0 {
        return Ok((operations, model));
    }

    let mut transaction = item.to_transaction(user_name).map_err(|e| {
        Error::new(
            format!("{}:ToTransaction", component),
            format!("Failed to create transaction from item: {}", e),
            get_location!(),
        )
        .log(file)
    })?;
    if operation == OrderType::Sell {
        transaction.transaction_type = TransactionType::Sale;
    }
    handle_transaction(transaction)
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
    find_by: FindByType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    handle_item_by_entity(
        CreateStockItem::new(wfm_url, sub_type.clone(), quantity).set_bought(price),
        user_name,
        operation,
        find_by,
        operation_flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
