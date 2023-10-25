use crate::{error::{AppError, self}, helper, wfm_client::client::WFMClient};
use std::sync::{Arc, Mutex};

#[tauri::command]
pub async fn refresh_auctions(
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let wfm = wfm.lock()?.clone();
    match wfm.auction().get_my_auctions().await {
        Ok(auctions) => {
            let json  = serde_json::to_value(auctions).unwrap();
            helper::emit_update(
                "auctions",
                "SET",
                Some(json.clone()),
            );
            Ok(json)
        }
        Err(e) => {
            error::create_log_file("refresh_auctions".to_string(), &e);
            return Err(e);
        }
    }
}
