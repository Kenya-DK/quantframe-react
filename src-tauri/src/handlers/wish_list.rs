use entity::{dto::*, enums::*, wish_list::*};
use service::{sea_orm::DatabaseConnection, WishListMutation};
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{handlers::*, types::*, utils::*, DATABASE};

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------

async fn handle_wishlist_buy(
    conn: &DatabaseConnection,
    item: &CreateWishListItem,
    mut model: Model,
    flags: &[&str],
    operations: &mut OperationSet,
    component: &str,
    file: &str,
) -> Result<Model, Error> {
    let (status, updated_item) = WishListMutation::bought_by_url_and_sub_type(
        conn,
        &item.wfm_url,
        item.sub_type.clone(),
        item.quantity,
    )
    .await
    .map_err(|e| e.with_location(get_location!()).log(file))?;

    log_wishlist_buy(component, item, &updated_item, &status, flags);

    if let Some(updated) = updated_item {
        model = updated;
    }

    operations.add(format!("WishListItemBought_{status}"));
    Ok(model)
}
async fn handle_wishlist_sell(
    conn: &DatabaseConnection,
    mut model: Model,
    flags: &[&str],
    operations: &mut OperationSet,
    component: &str,
    file: &str,
) -> Result<Model, Error> {
    let (status, created_item) = WishListMutation::add_item(conn, model)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    log_wishlist_sell(component, &created_item, &status, flags);

    model = created_item;
    operations.add(format!("WishListItemAdded_{status}"));

    Ok(model)
}
fn log_wishlist_buy(
    component: &str,
    item: &CreateWishListItem,
    updated: &Option<Model>,
    status: &str,
    flags: &[&str],
) {
    let log_opts = utils::LoggerOptions::default();

    match (status, updated) {
        ("NotFound", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!("Wish list item not found for URL: {}", item.wfm_url),
            &log_opts.set_enable(!flags.contains(&"DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Bought wish list item: {}, quantity: {}, status: {}",
                updated.item_name, updated.quantity, status
            ),
            &log_opts.set_enable(!flags.contains(&"DisableBoughtLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Deleted wish list item: {}, quantity: {}, status: {}",
                item.item_name, item.quantity, status
            ),
            &log_opts.set_enable(!flags.contains(&"DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:BoughtByUrlAndSubType"),
            &format!(
                "Updated wish list item: {}, quantity: {}, status: {}",
                item.item_name, item.quantity, status
            ),
            &log_opts.set_enable(!flags.contains(&"DisableUpdatedLog")),
        ),

        _ => {}
    }
}
fn log_wishlist_sell(component: &str, item: &Model, status: &str, flags: &[&str]) {
    let log_opts = utils::LoggerOptions::default();

    match status {
        "Created" => info(
            format!("{component}:AddWishListItem"),
            &format!("Created wish list item: {}", item.item_name),
            &log_opts.set_enable(!flags.contains(&"DisableCreatedLog")),
        ),
        "Updated" => info(
            format!("{component}:UpdateWishListItem"),
            &format!("Updated wish list item: {}", item.item_name),
            &log_opts.set_enable(!flags.contains(&"DisableUpdatedLog")),
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
        false,
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
            model = handle_wishlist_buy(
                conn,
                &item,
                model,
                operation_flags,
                &mut operations,
                component,
                file,
            )
            .await?;
        }

        OrderType::Sell => {
            model = handle_wishlist_sell(
                conn,
                model,
                operation_flags,
                &mut operations,
                component,
                file,
            )
            .await?;
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
        return Ok((operations, model));
    }

    // --------------------------------------------------
    // WFM sync (buy only)
    // --------------------------------------------------
    if operation == OrderType::Buy {
        sync_wfm(&item, operation, &mut operations, file).await?;
    }

    // --------------------------------------------------
    // Transaction
    // --------------------------------------------------

    if item.bought.unwrap_or(0) <= 0 {
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
