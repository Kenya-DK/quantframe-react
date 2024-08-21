use serde_json::json;
use service::{StockRivenMutation, StockRivenQuery, TransactionMutation};

use crate::{
    app::client::AppState,
    cache::{client::CacheClient, types::item_price_info::StockRiven},
    helper,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType, types::auction::Auction},
};
use std::sync::{Arc, Mutex};

// Create a static variable to store the log file name

#[tauri::command]
pub async fn auction_refresh(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();
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
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let app = app.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();
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
                helper::add_metric("Auction_Delete", "manual");
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
                qf.analytics()
                    .add_metric("Stock_RivenUpdate", "manual_auction_delete");
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
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<i64, AppError> {
    let app = app.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let qf = qf.lock()?.clone();

    let current_auctions = match wfm.auction().get_my_auctions().await {
        Ok(auctions) => auctions,
        Err(e) => {
            error::create_log_file("command_auctions.log".to_string(), &e);
            return Err(e);
        }
    };
    let total = current_auctions.len() as i64;
    helper::add_metric("Auction_DeleteAll", "manual");
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
#[tauri::command]
pub async fn auction_import(
    auction: Auction<String>,
    bought: i64,
    app: tauri::State<'_, Arc<Mutex<AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    cache: tauri::State<'_, Arc<Mutex<CacheClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<entity::stock::riven::stock_riven::Model, AppError> {
    let app = app.lock()?.clone();
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
        app,
        cache,
        notify,
        wfm,
        qf,
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
