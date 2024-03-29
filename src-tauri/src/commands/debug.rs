use once_cell::sync::Lazy;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::{
    fs::OpenOptions,
    sync::{Arc, Mutex},
};

use crate::cache::modules::item;
use crate::enums::OrderType;
use crate::structs::{Order, Orders};
use crate::{
    debug::DebugClient,
    error::{self, AppError},
};
use crate::{helper, logger};

use super::{auth, live_scraper};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_debug.log".to_string()));

#[tauri::command]
pub async fn import_warframe_algo_trader_data(
    db_path: String,
    import_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    let debug = debug.lock()?.clone();
    match debug
        .import_warframe_algo_trader_data(db_path, import_type)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn reset_data(
    reset_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    let debug = debug.lock()?.clone();
    debug.reset_data(reset_type).await?;
    Ok(())
}
#[tauri::command]
pub async fn get_trades(
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let debug = debug.lock()?.clone();
    debug.get_trades()
}
#[tauri::command]
pub async fn test_method(
    id: String,
    data: Option<Value>,
    // wfm: tauri::State<'_, Arc<Mutex<crate::wfm_client::client::WFMClient>>>,
    // debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
    live_scraper: tauri::State<'_, Arc<Mutex<crate::live_scraper::client::LiveScraperClient>>>,
    // auth: tauri::State<'_, Arc<Mutex<crate::auth::AuthState>>>,
) -> Result<(), AppError> {
    // let auth = auth.lock()?.clone();
    // let wfm = wfm.lock()?.clone();
    // let debug = debug.lock()?.clone();
    let live_scraper = live_scraper.lock()?.clone();
    if id == "compare_live" {       
    } else if id == "test_df" {
    } else if id == "test2" {
    } else {
        return Err(AppError::new(
            "Debug",
            eyre::eyre!("Invalid test method id"),
        ));
    }
    Ok(())
}

#[tauri::command]
pub async fn simulate_trade(mut list: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:/Users/Kenya/AppData/Local/Warframe/EE.log")
        .unwrap();
    list.push("6116.803 Script [Info]: Dialog.lua: Dialog::CreateOk(description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok)".to_string());
    // Loop through the list and write each item to the file
    for item in list {
        if let Err(e) = writeln!(file, "{}", item) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
