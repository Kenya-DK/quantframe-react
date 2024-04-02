use once_cell::sync::Lazy;

use crate::{
    helper, utils::modules::error::{self, AppError}, wfm_client::client::WFMClient
};
use std::sync::{Arc, Mutex};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_auctions.log".to_string()));

#[tauri::command]
pub async fn refresh_auctions(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.auction().get_my_auctions().await {
        Ok(auctions) => {
            let json = serde_json::to_value(auctions).unwrap();
            helper::emit_update("auctions", "SET", Some(json.clone()));
            Ok(json)
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
