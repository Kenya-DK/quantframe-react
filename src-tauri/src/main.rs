// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use directories::BaseDirs;
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::Manager;
mod whisper_scraper;

use whisper_scraper::WhisperScraper;

mod live_scraper;

use live_scraper::LiveScraper;

#[tauri::command]
fn toggle_whisper_scraper(whisper_scraper: tauri::State<'_, Arc<Mutex<WhisperScraper>>>) {
    let mut whisper_scraper = whisper_scraper.lock().unwrap();

    if whisper_scraper.is_running() {
        whisper_scraper.stop_loop();
    } else {
        whisper_scraper.start_loop();
    }
}
#[tauri::command]
fn toggle_live_scraper(live_scraper: tauri::State<'_, Arc<Mutex<LiveScraper>>>) {
    let mut live_scraper = live_scraper.lock().unwrap();

    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        live_scraper.start_loop();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            // Get the 'main' window
            let window = app.get_window("main").unwrap().clone();

            if let Some(base_dirs) = BaseDirs::new() {
                let local_path = Path::new(base_dirs.data_local_dir());
                let warfream_path = local_path.join("Warframe");
                println!("{:?}", warfream_path);
                let log_path = warfream_path.join("EE.log");
                println!("Path Warfream EE.log{:?}", log_path);
                // Create an instance of WhisperScraper
                let whisper_scraper =
                    Arc::new(Mutex::new(WhisperScraper::new(window.clone(), log_path)));
                app.manage(whisper_scraper);
            }

            // Create an instance of LiveScraper
            let live_scraper = Arc::new(Mutex::new(LiveScraper::new(window)));

            // Manage the state
            app.manage(live_scraper);

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            toggle_whisper_scraper,
            toggle_live_scraper
        ])
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
