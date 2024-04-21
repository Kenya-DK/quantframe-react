use std::sync::{Arc, Mutex};

use serde_json::json;


use crate::{
    notification::client::NotifyClient, utils::{enums::ui_events::{UIEvent, UIOperationEvent}, modules::error::{self, AppError}}, wfm_client::{client::WFMClient, types::{chat_data::ChatData, chat_message::ChatMessage}}
};

#[tauri::command]
pub async fn chat_refresh(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let current_orders = match wfm.orders().get_my_orders().await {
        Ok(mut auctions) => {
            let mut orders = auctions.buy_orders;
            orders.append(&mut auctions.sell_orders);
            orders
        }
        Err(e) => {
            error::create_log_file("command_orders.log".to_string(), &e);
            return Err(e);
        }
    };
    notify.gui().send_event_update(
        UIEvent::UpdateChats,
        UIOperationEvent::Set,
        Some(json!(current_orders)),
    );

    Ok(())
}