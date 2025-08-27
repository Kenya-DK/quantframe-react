use entity::{dto::*, enums::*, wish_list::*};
use service::WishListMutation;
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{enums::*, handlers::*, utils::*, DATABASE};

/// Handles wish list operations (add/remove) with WFM integration
pub async fn handle_wish_list_by_entity(
    mut item: CreateWishListItem,
    user_name: impl Into<String>,
    operation: OrderType,
) -> Result<(Vec<String>, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleWishListItem";
    let file = "handle_wish_list_item.log";
    let mut operations: Vec<String> = vec![];

    item.validate(FindByType::Url).map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(Some(file));
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
                        &utils::LoggerOptions::default(),
                    );
                } else if let Some(item) = updated_item {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Bought wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default(),
                    );
                    model = item;
                } else if s_operation == "Deleted" {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Deleted wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default(),
                    );
                } else if s_operation == "Updated" {
                    info(
                        format!("{}:BoughtByUrlAndSubType", component),
                        &format!(
                            "Updated wish list item: {}, quantity: {}, status: {}",
                            item.item_name, item.quantity, s_operation
                        ),
                        &utils::LoggerOptions::default(),
                    );
                }
                operations.push(format!("WishListItemBought_{}", s_operation));
            }
            Err(e) => {
                let error = Error::from_db(
                    format!("{}:BoughtByUrlAndSubType", component),
                    format!("Failed to handle buy operation with item: {}", item),
                    e,
                    get_location!(),
                );
                error.log(Some(file));
                return Err(error);
            }
        }
    } else if operation == OrderType::Sell {
        // Handle sell operation
        match WishListMutation::add_item(conn, model).await {
            Ok((s_operation, created_item)) => {
                if s_operation == "Created" {
                    info(
                        format!("{}:AddWishListItem", component),
                        &format!("Created wish list item: {}", created_item.item_name),
                        &utils::LoggerOptions::default(),
                    );
                } else if s_operation == "Updated" {
                    info(
                        format!("{}:UpdateWishListItem", component),
                        &format!("Updated wish list item: {}", created_item.item_name),
                        &utils::LoggerOptions::default(),
                    );
                }
                model = created_item;
                operations.push(format!("WishListItemAdded_{}", s_operation));
            }
            Err(e) => {
                let error = Error::from_db(
                    format!("{}:AddWishListItem", component),
                    format!(
                        "Failed to handle sell operation with wish list item: {}",
                        item
                    ),
                    e,
                    get_location!(),
                );
                error.log(Some(file));
                return Err(error);
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
                operations.push(format!("WFMItem_{}", operation_status));
            }
            Err(e) => {
                return Err(e.with_location(get_location!()).log(Some(file)));
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
        .log(Some(file)));
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
        .log(Some(file))
    })?;
    if operation == OrderType::Sell {
        transaction.transaction_type = TransactionType::Sale;
    }
    handle_transaction(transaction)
        .await
        .map_err(|e| e.with_location(get_location!()).log(Some(file)))?;

    Ok((operations, model))
}
/// Handles wish list operations (add/remove) with WFM integration
pub async fn handle_wish_list(
    wfm_url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
    user_name: impl Into<String>,
    operation: OrderType,
) -> Result<(Vec<String>, Model), Error> {
    handle_wish_list_by_entity(
        CreateWishListItem::new(wfm_url, sub_type.clone(), quantity).set_bought(price),
        user_name,
        operation,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
