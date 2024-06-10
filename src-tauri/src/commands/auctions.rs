use serde_json::json;

use crate::{
    notification::client::NotifyClient, utils::{enums::ui_events::{UIEvent, UIOperationEvent}, modules::error::{self, AppError}}, wfm_client::client::WFMClient
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
