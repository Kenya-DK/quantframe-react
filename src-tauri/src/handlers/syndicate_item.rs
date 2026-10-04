use std::sync::Mutex;

use entity::{enums::*, syndicate_item::*};
use service::{SyndicateItemMutation, SyndicateItemQuery};
use tauri::Manager;
use utils::SubType;
use utils::{get_location, info, warning, Error, OperationSet};
use wf_market::enums::OrderType;

use crate::app::AppState;
use crate::types::UIEvent;
use crate::{handlers::*, utils::CreateSyndicateItemExt, DATABASE};
use crate::{send_event, APP};

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------
fn log(
    component: &str,
    item: &CreateSyndicateItem,
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
        "SyndicateItemOperation"
    };
    match (status, updated_item) {
        ("NotFound", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Syndicate item not found for URL: {} | Operations: {:?} | Flags: {:?}",
                item.wfm_url, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Sold syndicate item {} | Status: {} | Operations: {:?} | Flags: {:?}",
                updated.item_name, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Deleted syndicate item {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Updated syndicate item: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Created", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Created syndicate item: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCreatedLog")),
        ),

        ("Complete", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Completed syndicate item: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                item.item_name, status, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCompleteLog")),
        ),
        _ => {
            warning(
                format!("{component}:{sub_component}"),
                &format!(
                    "Unhandled status: {} for syndicate item: {} | Operations: {:?} | Flags: {:?}",
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

pub async fn handle_syndicate_item_by_entity(
    mut item: CreateSyndicateItem,
    price: i64,
    user_name: impl Into<String>,
    order_type: OrderType,
    flags: &OperationSet,
) -> Result<(OperationSet, Model), Error> {
    let con = DATABASE.get().unwrap();
    let component = "HandleSyndicateItem";
    let file = "handle_syndicate_item.log";

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
    let mut delete = false; // Placeholder for delete logic if needed in the future
    match order_type {
        OrderType::Sell => {
            if let Some(item) = SyndicateItemQuery::find_by_url_name_and_sub_type(
                con,
                &item.wfm_url,
                item.sub_type.clone(),
            )
            .await
            .map_err(|e| e.with_location(get_location!()).log(file))?
            {
                let app = APP.get().expect("APP not initialized");
                let state = app.state::<Mutex<AppState>>();
                let mut guard = state.lock()?;
                let settings = &mut guard.settings.live_scraper.syndicate.wts;
                settings.deduct_standing(&item.syndicate_unique_name, item.standing_cost)?;
                delete =
                    settings.can_afford_posting(&item.syndicate_unique_name, item.standing_cost)?;
                guard
                    .settings
                    .save()
                    .map_err(|e| e.with_location(get_location!()))?;
                send_event!(UIEvent::RefreshSettings, Some(json!({})));
            }
        }

        OrderType::Buy => {
            let (s_operation, created_item) = SyndicateItemMutation::add_item(con, model)
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
    if should_run_wfm(&flags, &operations) && order_type == OrderType::Sell {
        let operation = if delete {
            "ShouldDelete"
        } else {
            "ShouldClose"
        };
        let status = handle_wfm_item(
            &item.wfm_id,
            &item.sub_type,
            1,
            order_type,
            OperationSet::from(vec![operation]),
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
    if price <= 0 {
        operations.add("PriceZeroNoTransaction");
        log(component, &item, &None, "Complete", &flags, &operations);
        return Ok((operations, model));
    }

    let mut tx = item.to_transaction(user_name, price).map_err(|e| {
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

pub async fn handle_syndicate_item(
    wfm_url: impl Into<String>,
    sub_type: Option<SubType>,
    raw_syndicate: impl Into<String>,
    price: i64,
    user_name: impl Into<String>,
    order_type: OrderType,
    flags: &OperationSet,
) -> Result<(OperationSet, Model), Error> {
    handle_syndicate_item_by_entity(
        CreateSyndicateItem::new(wfm_url, sub_type.clone(), raw_syndicate),
        price,
        user_name,
        order_type,
        flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
