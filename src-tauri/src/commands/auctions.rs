use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery};

use crate::{
    app::client::AppState,
    cache::client::CacheClient,
    helper,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::{
            error::{self, AppError},
            logger,
        },
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType, types::auction::Auction},
    DATABASE,
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
    helper::add_metric("Auction_Refresh", "manual");
    notify.gui().send_event_update(
        UIEvent::UpdateAuction,
        UIOperationEvent::Set,
        Some(json!(current_auctions.auctions)),
    );

    Ok(())
}
#[tauri::command]
pub async fn auction_delete(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let conn = DATABASE.get().unwrap();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    // Get the my auctions from the WFM
    let auction = match wfm.auction().get_auction(&id).await {
        Ok(auction) => auction,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };

    if auction.is_none() {
        logger::warning_con(
            "Command:AuctionDelete",
            format!("Auction not found: {}", id).as_str(),
        );
        return Err(AppError::new(
            "Auction not found",
            eyre::eyre!("Auction not found"),
        ));
    }

    let auction = auction.unwrap();

    // Delete the auction form the WFM if it exists
    match wfm.auction().delete(&auction.id).await {
        Ok(_) => {
            notify.gui().send_event_update(
                UIEvent::UpdateAuction,
                UIOperationEvent::Delete,
                Some(json!({ "id": auction.id })),
            );
            helper::add_metric("Auction_Delete", "manual");
        }
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    }

    //Update the StockRiven
    match StockRivenMutation::clear_order_id(conn, &id).await {
        Ok(res) => {
            if res.is_some() {
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(res)),
                );
            }
        }
        Err(e) => {
            let err = AppError::new_db("Command:AuctionDelete", e);
            error::create_log_file("command_auctions.log".to_string(), &err);
            return Err(err);
        }
    };
    Ok(())
}
#[tauri::command]
pub async fn auction_delete_all(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<i64, AppError> {
    let conn = DATABASE.get().unwrap();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();

    let current_auctions = match wfm.auction().get_my_auctions().await {
        Ok(auctions) => auctions,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };
    let total = current_auctions.total();
    helper::add_metric("Auction_DeleteAll", "manual");
    for auction in current_auctions.auctions {
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
    match StockRivenQuery::clear_all_order_id(conn).await {
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
#[tauri::command]
pub async fn auction_import(
    auction: Auction<String>,
    bought: i64,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<entity::stock::riven::stock_riven::Model, AppError> {
    let notify = notify.lock()?.clone();
    let cache = cache.lock()?.clone();
    let qf = qf.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    let mut riven_entry = match auction.convert_to_create_stock(bought) {
        Ok(stock) => stock,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };

    match helper::progress_stock_riven(
        &mut riven_entry,
        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
        "",
        OrderType::Buy,
        "manual_auction_import",
        &cache,
        &notify,
        &wfm,
        &qf,
    )
    .await
    {
        Ok((stock, _)) => {
            return Ok(stock);
        }
        Err(e) => {
            error::create_log_file("command_stock_riven_create.log".to_string(), &e);
            return Err(e);
        }
    }
}
