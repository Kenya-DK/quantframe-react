use serde_json::json;

use crate::{
    helper,
    notification::client::NotifyClient,
    settings::SettingsState,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    },
    wfm_client::{client::WFMClient, types::order::Order},
};
use std::sync::{Arc, Mutex};


#[tauri::command]
pub async fn order_refresh(
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
        UIEvent::UpdateOrders,
        UIOperationEvent::Set,
        Some(json!(current_orders)),
    );

    Ok(())
}
