use std::sync::Arc;

use crate::{live_scraper::client::LiveScraperClient, logger::error, error};

#[tauri::command]
pub fn toggle_live_scraper(
    live_scraper: tauri::State<'_, Arc<std::sync::Mutex<LiveScraperClient>>>,
) {
    let mut live_scraper = live_scraper.lock().unwrap();
    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        match live_scraper.start_loop() {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file("debug".to_string(), &e);
            }
        }
    }
}

