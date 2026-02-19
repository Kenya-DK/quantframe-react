use entity::{dto::SubType, enums::TransactionType, transaction::TransactionPaginationQueryDto};
use service::{TransactionMutation, TransactionQuery};
use utils::{get_location, info, Error, SortDirection};
use wf_market::{enums::OrderType, types::UpdateOrderParams};

use crate::{
    utils::{modules::states, ErrorFromExt, SubTypeExt},
    DATABASE,
};

// Handles Warframe Market order operations (close/delete/update)
pub async fn handle_wfm_item(
    wfm_id: impl Into<String>,
    sub_type: &Option<SubType>,
    quantity: i64,
    operation: OrderType,
    delete: bool,
) -> Result<String, Error> {
    let wfm_id = wfm_id.into();
    let log_options = utils::LoggerOptions::default();
    let app = states::app_state()?;

    let component = "HandleWFMItem";
    let file = "handle_wfm_item.log";

    let wf_sub_type: wf_market::types::SubType = SubTypeExt::from_entity(sub_type.to_owned());

    // Skip buy if reporting disabled
    if operation == OrderType::Buy && !app.settings.live_scraper.report_to_wfm {
        return Ok("SkippedBuyWfmReportDisabled".to_string());
    }

    let Some(mut order) =
        app.wfm_client
            .order()
            .cache_orders()
            .find_order(&wfm_id, &wf_sub_type, operation)
    else {
        info(
            &format!("{component}:NoOrder"),
            &format!(
                "No WFM order found for WFM ID: {} | SubType: {} | Operation: {:?}",
                wfm_id, wf_sub_type, operation
            ),
            &log_options,
        );
        return Ok("NoOrder".to_string());
    };

    // ---- Compute new quantity ----
    order.quantity = (order.quantity as i64 - quantity).max(0) as u32;

    let reporting_enabled = app.settings.live_scraper.report_to_wfm;
    let should_close = reporting_enabled && !delete;
    let should_delete = delete || order.quantity == 0;

    // ---- Helpers ----
    let map_err = |stage: &str, msg: &str, e| {
        let err = Error::from_wfm(&format!("{component}:{stage}"), msg, e, get_location!());
        err.log(file);
        err
    };

    // ---- Perform action ----
    if should_close {
        app.wfm_client
            .order()
            .close(&order.id, quantity as u32)
            .await
            .map_err(|e| map_err("Close", "Failed to close WFM order", e))?;

        info(
            &format!("{component}:Close"),
            &format!("Closed WFM order: {:?}, {:?}", order.id, quantity),
            &log_options,
        );

        return Ok("Closed".to_string());
    }

    if should_delete {
        app.wfm_client
            .order()
            .delete(&order.id)
            .await
            .map_err(|e| map_err("Delete", "Failed to delete WFM order", e))?;

        info(
            &format!("{component}:Delete"),
            &format!("Deleted WFM order: {:?}", order.id),
            &log_options,
        );

        return Ok("Deleted".to_string());
    }

    // ---- Otherwise update ----
    app.wfm_client
        .order()
        .update(
            &order.id,
            UpdateOrderParams::default().with_quantity(order.quantity),
        )
        .await
        .map_err(|e| map_err("Update", "Failed to update WFM order", e))?;

    info(
        &format!("{component}:Update"),
        &format!("Updated WFM order: {:?}, {:?}", order.id, order.quantity),
        &log_options,
    );

    Ok("Updated".to_string())
}

/// Handles transaction creation and database persistence
pub async fn handle_transaction(
    mut transaction: entity::transaction::Model,
    use_current_date: bool,
) -> Result<entity::transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();

    // Find the existing transaction in the database
    if transaction.transaction_type == TransactionType::Sale {
        let existing_transaction = TransactionQuery::get_all(
            conn,
            TransactionPaginationQueryDto::new(1, 1)
                .set_transaction_type(TransactionType::Purchase)
                .set_wfm_id(transaction.wfm_id.clone())
                .set_sub_type(transaction.sub_type.clone())
                .set_sort_by("created_at")
                .set_sort_direction(SortDirection::Desc),
        )
        .await?;
        if let Some(purchase_transaction) = existing_transaction.results.first() {
            let purchase_price_per_unit =
                purchase_transaction.price / purchase_transaction.quantity;

            let sold_price_per_unit = transaction.price / transaction.quantity;

            let total_profit =
                (sold_price_per_unit - purchase_price_per_unit) * transaction.quantity;

            transaction.set_profit(total_profit);
        }
        // Overall credits calculation
        transaction.set_credits(transaction.price * crate::enums::TradeItemType::Platinum.to_tax());
    }

    match TransactionMutation::create(conn, &transaction, use_current_date).await {
        Ok(updated_item) => Ok(updated_item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
