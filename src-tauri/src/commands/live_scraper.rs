use std::sync::{Arc, Mutex};


use serde_json::json;

use crate::{live_scraper::client::LiveScraperClient, notification::client::NotifyClient, utils::{enums::ui_events::UIEvent, modules::error::{self, AppError}}};

#[tauri::command]
pub fn live_scraper_set_running_state(
    enable: bool,
    live_scraper: tauri::State<'_, Arc<std::sync::Mutex<LiveScraperClient>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
) -> Result<(), AppError> {
    let notify = notify.lock()?.clone();
    let mut live_scraper = live_scraper.lock()?;
    if enable  && !live_scraper.is_running() {
        match live_scraper.start_loop() {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file("command.log".to_string(), &e);
                notify.gui().send_event(UIEvent::OnLiveTradingError, Some(json!(e)));
            }
        }
    } else {
        live_scraper.stop_loop();
    }
    notify.gui().send_event(UIEvent::UpdateLiveTradingRunningState, Some(json!(live_scraper.is_running())));
    Ok(())

}
