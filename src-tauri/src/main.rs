// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::structs::Settings;
use directories::BaseDirs;
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::Manager;
mod structs;
mod whisper_scraper;
use whisper_scraper::WhisperScraper; // add this line
mod live_scraper;
use live_scraper::LiveScraper;

mod database;
mod price_scraper;
mod wfm_client;

use price_scraper::CSV_PATH;
use price_scraper::CSV_BACKOP_PATH;
use price_scraper::WINDOW as PS_WINDOW;

use database::DB_PATH;
use database::WINDOW as DB_WINDOW;

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
    if live_scraper.is_running() {
        live_scraper.stop_loop();
    } else {
        match live_scraper.start_loop(token, settings) {
            Ok(_) => {}
            Err(e) => {
                println!("Error while starting live scraper {:?}", e);
            }
        }
    }
}

#[tauri::command]
async fn generate_price_history(platform: String) {
    tauri::async_runtime::spawn(async move {
        let runner = price_scraper::generate(platform.as_str()).await;
        match runner {
            Ok(_) => {}
            Err(e) => {
                println!("Error while generating price history {:?}", e);
            }
        }
    });
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
                
                *CSV_PATH.lock().unwrap() = csv_path.clone().to_str().unwrap().to_string();
                *CSV_BACKOP_PATH.lock().unwrap() = csv_backop_path.clone().to_str().unwrap().to_string();

                *PS_WINDOW.lock().unwrap() = Some(window.clone());


                // Get database path
                let db_path = app_path.join("quantframe.sqlite");
                *DB_PATH.lock().unwrap() = db_path.clone().to_str().unwrap().to_string();

                *DB_WINDOW.lock().unwrap() = Some(window.clone());

                // Create an instance of LiveScraper
                let live_scraper = Arc::new(Mutex::new(LiveScraper::new(
                    window.clone(),
                    String::from(""),
                    String::from("")
                )));

                // Manage the state
                app.manage(live_scraper);
            }
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            toggle_whisper_scraper,
            toggle_live_scraper,
            generate_price_history
        ])
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
