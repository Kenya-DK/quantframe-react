use std::vec;

use entity::{dto::*, enums::*, wish_list::*};
use service::WishListMutation;
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{handlers::*, types::*, utils::*, DATABASE};

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------

fn log(
    component: &str,
    item: &CreateWishListItem,
    updated: &Option<Model>,
    status: &str,
    flags: &[&str],
    operations: &OperationSet,
) {
    let log_opts = utils::LoggerOptions::default();
    match (status, updated) {
        ("NotFound", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Wish list item not found for URL: {} | Operations: {:?}",
                item.wfm_url, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Bought wish list item: {} | Quantity: {} | Status: {} | Operations: {:?}",
                updated.item_name, updated.quantity, status, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableBoughtLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Deleted wish list item: {} | Quantity: {} | Status: {} | Operations: {:?}",
                item.item_name, item.quantity, status, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Updated wish list item: {} | Quantity: {} | Status: {} | Operations: {:?}",
                item.item_name, item.quantity, status, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableUpdatedLog")),
        ),

        ("Created", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Created wish list item: {} | Quantity: {} | Status: {} | Operations: {:?}",
                item.item_name, item.quantity, status, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableCreatedLog")),
        ),

        ("Complete", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Completed wish list item: {} | Quantity: {} | Status: {} | Operations: {:?}",
                item.item_name, item.quantity, status, operations.operations
            ),
            &log_opts.set_enable(!flags.contains(&"DisableCompleteLog")),
        ),

        _ => {}
    }
}
async fn sync_wfm(
    item: &CreateWishListItem,
    operation: OrderType,
    operations: &mut OperationSet,
    file: &str,
) -> Result<(), Error> {
    let status = handle_wfm_item(
        &item.wfm_id,
        &item.sub_type,
        item.quantity,
        operation,
        OperationSet::from(vec!["ForceOrderSync"]),
    )
    .await
    .map_err(|e| e.with_location(get_location!()).log(file))?;

    operations.add(format!("WFMItem_{status}"));
    Ok(())
}

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
    // Wishlist mutation (buy / sell)
    // --------------------------------------------------
    match operation {
        OrderType::Buy => {
            let (s_operation, updated_item) = WishListMutation::bought_by_url_and_sub_type(
                conn,
                &item.wfm_url,
                item.sub_type.clone(),
                item.quantity,
            )
            .await
            .map_err(|e| e.with_location(get_location!()).log(file))?;

            log(
                component,
                &item,
                &updated_item,
                &s_operation,
                operation_flags,
                &operations,
            );

            if let Some(updated) = updated_item {
                model = updated;
            }

            operations.add(format!("WishListItemBought_{s_operation}"));
        }

        OrderType::Sell => {
            let (s_operation, created_item) = WishListMutation::add_item(conn, model)
                .await
                .map_err(|e| e.with_location(get_location!()).log(file))?;

            log(
                component,
                &item,
                &None,
                &s_operation,
                operation_flags,
                &operations,
            );

            model = created_item;
            operations.add(format!("WishListItemSold_{s_operation}"));
        }
    }

    // --------------------------------------------------
    // Early return from flags
    // --------------------------------------------------
    if operation_flags
        .iter()
        .find_map(|f| f.strip_prefix("ReturnOn:"))
        .map(|suffix| operations.ends_with(suffix))
        .unwrap_or(false)
    {
        operations.add("EarlyReturnFlagTriggered");
        log(
            component,
            &item,
            &None,
            "Complete",
            operation_flags,
            &operations,
        );
        return Ok((operations, model));
    }

    // --------------------------------------------------
    // WFM sync
    // --------------------------------------------------
    sync_wfm(&item, operation, &mut operations, file).await?;

    // --------------------------------------------------
    // Transaction
    // --------------------------------------------------

    if item.bought.unwrap_or(0) <= 0 {
        operations.add("PriceZeroNoTransaction");
        log(
            component,
            &item,
            &None,
            "Complete",
            operation_flags,
            &operations,
        );
        return Ok((operations, model));
    }
    let mut tx = item.to_transaction(user_name).map_err(|e| {
        Error::new(
            "HandleWishListItem:ToTransaction",
            format!("Failed to create transaction: {e}"),
            get_location!(),
        )
        .log(file)
    })?;

    if operation == OrderType::Sell {
        tx.transaction_type = TransactionType::Sale;
    }

    handle_transaction(tx, true)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    log(
        component,
        &item,
        &None,
        "Complete",
        operation_flags,
        &operations,
    );
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
