use std::sync::Arc;

use crate::live_scraper::LiveScraper;

#[tauri::command]
pub fn toggle_live_scraper(live_scraper: tauri::State<'_, Arc<std::sync::Mutex<LiveScraper>>>) {
    let mut live_scraper = live_scraper.lock().unwrap();
    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        match live_scraper.start_loop() {
            Ok(_) => {}
            Err(_e) => {}
        }
    }
}
