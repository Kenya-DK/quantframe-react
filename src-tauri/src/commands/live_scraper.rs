use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{error, live_scraper::client::LiveScraperClient};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_live_scraper.log".to_string()));

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
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            }
        }
    }
}
