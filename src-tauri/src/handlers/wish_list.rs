use entity::{dto::*, enums::*, wish_list::*};
use service::WishListMutation;
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{handlers::*, types::*, utils::*, DATABASE};
/// Handles wish list operations (add/remove) with WFM integration
pub async fn handle_wish_list_by_entity(
    mut item: CreateWishListItem,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleWishListItem";
    let file = "handle_wish_list_item.log";
    let mut operations = OperationSet::new();

    item.validate().map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(file);
        e
    })?;
    let mut model = item.to_model();

    // Handle StockItem creation, deletion, or update
    if operation == OrderType::Buy {
        // Handle buy operation
        match WishListMutation::bought_by_url_and_sub_type(
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
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Wish list item not found for URL: {}, status: {}",
                            item.wfm_url, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableNotFoundLog")),
                    );
                } else if let Some(item) = updated_item {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Bought wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableBoughtLog")),
                    );
                    model = item;
                } else if s_operation == "Deleted" {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Deleted wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableDeletedLog")),
                    );
                } else if s_operation == "Updated" {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Updated wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableUpdatedLog")),
                    );
                }
                operations.add(format!("WishListItemBought_{}", s_operation));
            }
            Err(e) => return Err(e.with_location(get_location!()).log(file)),
        }
    } else if operation == OrderType::Sell {
        // Handle sell operation
        match WishListMutation::add_item(conn, model).await {
            Ok((s_operation, created_item)) => {
                if s_operation == "Created" {
                    info(
                        format!("{}:AddWishListItem", component),
                        &format!("Created wish list item: {}", created_item.item_name),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableCreatedLog")),
                    );
                } else if s_operation == "Updated" {
                    info(
                        format!("{}:UpdateWishListItem", component),
                        &format!("Updated wish list item: {}", created_item.item_name),
                        &utils::LoggerOptions::default()
                            .set_enable(!operation_flags.contains(&"DisableUpdatedLog")),
                    );
                }
                model = created_item;
                operations.add(format!("WishListItemAdded_{}", s_operation));
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

    // If the operation is a sale, we need to check if there's an existing order
    if operation == OrderType::Buy {
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
    }

    // Create a transaction from the item
    if !item.is_validated {
        return Err(Error::new(
            component,
            "Wish list item is not validated yet",
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
            format!("Failed to create transaction from wish list item: {}", e),
            get_location!(),
        )
        .log(file)
    })?;
    if operation == OrderType::Sell {
        transaction.transaction_type = TransactionType::Sale;
    }
    handle_transaction(transaction, true)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    Ok((operations, model))
}
/// Handles wish list operations (add/remove) with WFM integration
pub async fn handle_wish_list(
    wfm_url: impl Into<String>,
    sub_type: &Option<SubType>,
    quantity: i64,
    price: i64,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    handle_wish_list_by_entity(
        CreateWishListItem::new(wfm_url, sub_type.clone(), quantity).set_bought(price),
        user_name,
        operation,
        operation_flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
