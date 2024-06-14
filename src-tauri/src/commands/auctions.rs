use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::{
    app::client::AppState,
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::client::WFMClient,
};
use std::sync::{Arc, Mutex};

// Create a static variable to store the log file name

#[tauri::command]
pub async fn auction_refresh(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let current_auctions = match wfm.auction().get_my_auctions().await {
        Ok(auctions) => auctions,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };
    notify.gui().send_event_update(
        UIEvent::UpdateAuction,
        UIOperationEvent::Set,
        Some(json!(current_auctions)),
    );

    Ok(())
}
#[tauri::command]
pub async fn auction_delete(
    id: String,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();

    // Get the my auctions from the WFM
    let auction = match wfm.auction().get_auction_by_id(&id).await {
        Ok(auction) => auction,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };

    // Delete the auction form the WFM if it exists
    if let Some(auction) = auction {
        match wfm.auction().delete(&auction.id).await {
            Ok(_) => {
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": auction.id })),
                );
            }
            Err(e) => {
                error::create_log_file("command_auctions.log".to_string(), &e);
                return Err(e);
            }
        }
    }

    //Get StockRiven by WfmOrderId
    let stock = match StockRivenQuery::get_by_order_id(&app.conn, &id).await {
        Ok(stock) => stock,
        Err(e) => {
            let err = AppError::new_db("Command:AuctionDelete", e);
            error::create_log_file("command_auctions.log".to_string(), &err);
            return Err(err);
        }
    };
    if let Some(mut stock) = stock {
        stock.wfm_order_id = None;
        match StockRivenMutation::update_by_id(&app.conn, stock.id, stock.clone()).await {
            Ok(_) => {
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(stock)),
                );
            }
            Err(e) => {
                let err = AppError::new_db("Command:AuctionDelete", e);
                error::create_log_file("command_auctions.log".to_string(), &err);
                return Err(err);
            }
        }
    }

    Ok(())
}
#[tauri::command]
pub async fn auction_delete_all(
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<i64, AppError> {
    let app = app.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let current_auctions = match wfm.auction().get_my_auctions().await {
        Ok(auctions) => auctions,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };
    let total = current_auctions.len() as i64;
    for auction in current_auctions {
        // Delete the auction form the WFM if it exists
        match wfm.auction().delete(&auction.id).await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file("command_auctions.log".to_string(), &e);
                return Err(e);
            }
        }
    }
    notify.gui().send_event_update(
        UIEvent::UpdateAuction,
        UIOperationEvent::Set,
        Some(json!([])),
    );


    // Clear all WfmOrderId in StockRiven
    match StockRivenQuery::clear_all_order_id(&app.conn).await {
        Ok(stock) => {
            notify.gui().send_event_update(
                UIEvent::UpdateStockRivens,
                UIOperationEvent::Set,
                Some(json!(stock)),
            );
        }
        Err(e) => {
            let err = AppError::new_db("Command:AuctionDeleteAll", e);
            error::create_log_file("command_auctions.log".to_string(), &err);
            return Err(err);
        }
    }

    Ok(total)
}
