use entity::dto::SubType;
use service::TransactionMutation;
use utils::{get_location, info, Error};
use wf_market::{enums::OrderType, types::UpdateOrderParams};

use crate::{
    utils::{modules::states, ErrorFromExt, SubTypeExt},
    DATABASE,
};
/// Handles Warframe Market order operations (close/delete/update)
pub async fn handle_wfm_item(
    wfm_id: impl Into<String>,
    sub_type: &Option<SubType>,
    quantity: i64,
    operation: OrderType,
    delete: bool,
) -> Result<String, Error> {
    let wfm_id = wfm_id.into();
    let app = states::app_state()?;
    let component = "HandleWFMItem";
    let file = "handle_wfm_item.log";
    let mut operation_status = "NoOrder".to_string();
    let wf_sub_type: wf_market::types::SubType = SubTypeExt::from_entity(sub_type.to_owned());

    // Skip if order type is Buy and report to WFM is disabled
    if operation == OrderType::Buy && !app.settings.live_scraper.report_to_wfm {
        return Ok("SkippedBuyWfmReportDisabled".to_string());
    }

    if let Some(mut order) =
        app.wfm_client
            .order()
            .cache_orders()
            .find_order(&wfm_id, &wf_sub_type, operation)
    {
        order.quantity -= quantity as u32;

        if app.settings.live_scraper.report_to_wfm && !delete {
            match app
                .wfm_client
                .order()
                .close(&order.id, quantity as u32)
                .await
            {
                Ok(_) => {
                    info(
                        &format!("{}:Close", component),
                        &format!("Closed WFM order: {:?}, {:?}", order.id, quantity),
                        &utils::LoggerOptions::default(),
                    );
                    operation_status = "Closed".to_string();
                }
                Err(e) => {
                    let error = Error::from_wfm(
                        &format!("{}:Close", component),
                        "Failed to close WFM order",
                        e,
                        get_location!(),
                    );
                    error.log(file);
                    return Err(error);
                }
            }
        } else if order.quantity <= 0 || delete {
            match app.wfm_client.order().delete(&order.id).await {
                Ok(_) => {
                    info(
                        &format!("{}:Delete", component),
                        &format!("Deleted WFM order: {:?}", order.id),
                        &utils::LoggerOptions::default(),
                    );
                    operation_status = "Deleted".to_string();
                }
                Err(e) => {
                    let error = Error::from_wfm(
                        &format!("{}:Delete", component),
                        "Failed to delete WFM order",
                        e,
                        get_location!(),
                    );
                    error.log(file);
                    return Err(error);
                }
            }
        } else {
            match app
                .wfm_client
                .order()
                .update(
                    &order.id,
                    UpdateOrderParams::default().with_quantity(order.quantity),
                )
                .await
            {
                Ok(_) => {
                    info(
                        &format!("{}:Update", component),
                        &format!("Updated WFM order: {:?}, {:?}", order.id, order.quantity),
                        &utils::LoggerOptions::default(),
                    );
                    operation_status = "Updated".to_string();
                }
                Err(e) => {
                    let error = Error::from_wfm(
                        &format!("{}:Update", component),
                        "Failed to update WFM order",
                        e,
                        get_location!(),
                    );
                    error.log(file);
                    return Err(error);
                }
            }
        }
    } else {
        info(
            &format!("{}:NoOrder", component),
            &format!(
                "No WFM order found for WFM ID: {} | SubType: {} | Operation: {:?}",
                wfm_id, wf_sub_type, operation
            ),
            &utils::LoggerOptions::default(),
        );
    }

    Ok(operation_status)
}

/// Handles transaction creation and database persistence
pub async fn handle_transaction(
    transaction: entity::transaction::Model,
) -> Result<entity::transaction::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match TransactionMutation::create(conn, &transaction).await {
        Ok(updated_item) => Ok(updated_item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
