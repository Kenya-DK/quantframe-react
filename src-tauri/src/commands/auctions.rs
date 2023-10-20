use serde_json::{json, Value};

use crate::{auth::AuthState, error::AppError, helper, wfm_client::client::WFMClient};
use std::sync::{Arc, Mutex};

#[tauri::command]
pub async fn refresh_auctions(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock()?.clone();
    let auctions = wfm.auction().get_my_auctions().await?;
    helper::emit_update(
        "auctions",
        "SET",
        Some(serde_json::to_value(auctions).unwrap()),
    );
    Ok(())
}
