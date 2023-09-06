use crate::{structs::GlobleError, wfm_client::WFMClientState, error::AppError};
use std::sync::{Arc, Mutex};


#[tauri::command]
async fn get_auctions(auction_type: String, wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}