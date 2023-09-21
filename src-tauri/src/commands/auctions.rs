use crate::{error::AppError, wfm_client::client::WFMClient};
use std::sync::{Arc, Mutex};

#[tauri::command]
async fn get_auctions(
    auction_type: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    Ok(())
}
