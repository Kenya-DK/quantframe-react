// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::structs::Settings;
use directories::{BaseDirs, ProjectDirs};
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::Manager;
mod structs;
mod whisper_scraper;
use whisper_scraper::WhisperScraper; // add this line
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
fn toggle_live_scraper(
    live_scraper: tauri::State<'_, Arc<Mutex<LiveScraper>>>,
    token: String,
    settings: Settings,
) {
    let mut live_scraper = live_scraper.lock().unwrap();

    println!("token: {}", token);
    println!("obj: {:?}", settings.field1);
    println!("obj: {:?}", settings.field2);

    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        live_scraper.start_loop(token, settings);
    }
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            // Get the 'main' window
            let window = app.get_window("main").unwrap().clone();

            if let Some(base_dirs) = BaseDirs::new() {
                // Get the path of Warframe log file
                let local_path = Path::new(base_dirs.data_local_dir());
                let warfream_path = local_path.join("Warframe");
                let log_path = warfream_path.join("EE.log");
                // Create an instance of WhisperScraper
                let whisper_scraper =
                    Arc::new(Mutex::new(WhisperScraper::new(window.clone(), log_path)));
                app.manage(whisper_scraper);

                // App path for csv file
                let roaming_path = Path::new(base_dirs.data_dir());
                let app_path = roaming_path.join("quantframe");
                let csv_path = app_path.join("pricehistory.csv");
                let csv_backop_path = app_path.join("pricehistoryBackop.csv");
                // Create an instance of LiveScraper
                let live_scraper = Arc::new(Mutex::new(LiveScraper::new(
                    window,
                    String::from(""),
                    csv_path.to_str().unwrap().to_string(),
                    csv_backop_path.to_str().unwrap().to_string(),
                )));

                // Manage the state
                app.manage(live_scraper);
            }
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
