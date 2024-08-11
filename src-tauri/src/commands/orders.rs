use serde_json::json;

use crate::{
    live_scraper::client::LiveScraperClient, notification::client::NotifyClient, settings::SettingsState, utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::error::{self, AppError},
    }, wfm_client::client::WFMClient
};
use std::sync::{Arc, Mutex};

use super::live_scraper;

#[tauri::command]
pub async fn order_refresh(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<i32, AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let current_orders = match wfm.orders().get_my_orders().await {
        Ok(mut auctions) => {
            let mut orders = auctions.buy_orders;
            orders.append(&mut auctions.sell_orders);
            orders
        }
        Err(e) => {
            error::create_log_file("command_order_refresh.log".to_string(), &e);
            return Err(e);
        }
    };
    notify.gui().send_event_update(
        UIEvent::UpdateOrders,
        UIOperationEvent::Set,
        Some(json!(current_orders)),
    );

    Ok(current_orders.len() as i32)
}
#[tauri::command]
pub async fn order_delete(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();

    match wfm.orders().delete(&id).await {
        Ok(_) => {
            notify.gui().send_event_update(
                UIEvent::UpdateOrders,
                UIOperationEvent::Delete,
                Some(json!({"id": id})),
            );
        }
        Err(e) => {
            error::create_log_file("command_order_delete.log".to_string(), &e);
            return Err(e);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn order_delete_all(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
    live_scraper    : tauri::State<'_, Arc<Mutex<LiveScraperClient>>>,
) -> Result<i32, AppError> {
    let wfm = wfm.lock()?.clone();
    let notify = notify.lock()?.clone();
    let settings = settings.lock()?.clone();
    let live_scraper = live_scraper.lock()?.clone();

    live_scraper.stop_loop();
    live_scraper.set_can_run(false);

    let current_orders = match wfm.orders().get_my_orders().await {
        Ok(mut auctions) => {
            let mut orders = auctions.buy_orders;
            orders.append(&mut auctions.sell_orders);
            orders
        }
        Err(e) => {
            error::create_log_file("command_order_delete_all.log".to_string(), &e);
            live_scraper.set_can_run(true);
            return Err(e);
        }
    };
    let mut total = 0;
    for order in current_orders.iter() {
        if settings
            .live_scraper
            .stock_item
            .blacklist
            .contains(&order.item.clone().unwrap().url_name)
        {
            continue;
        }
        if let Err(e) = wfm.orders().delete(&order.id).await {
            live_scraper.set_can_run(true);
            error::create_log_file("command_order_delete_all.log".to_string(), &e);
            return Err(e);
        }
        total += 1;
        notify.gui().send_event_update(
            UIEvent::UpdateOrders,
            UIOperationEvent::Delete,
            Some(json!({"id": order.id})),
        );
    }
    live_scraper.set_can_run(true);
    Ok(total)
}
