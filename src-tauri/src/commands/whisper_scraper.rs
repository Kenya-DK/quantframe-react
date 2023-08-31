use std::sync::{Arc, Mutex};

use crate::whisper_scraper::WhisperScraper;

#[tauri::command]
pub fn toggle_whisper_scraper(
    whisper_scraper: tauri::State<'_, Arc<std::sync::Mutex<WhisperScraper>>>,
) {
    let mut whisper_scraper = whisper_scraper.lock().unwrap();

    if whisper_scraper.is_running() {
        whisper_scraper.stop_loop();
    } else {
        whisper_scraper.start_loop();
    }
}
