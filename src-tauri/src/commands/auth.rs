use std::{
    sync::{Arc, Mutex},
};

use serde_json::Value;

use crate::{auth::AuthState, wfm_client::WFMClientState};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<AuthState, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    match wfm.login(email, password).await {
        Ok(user) => {
            user.save_to_file().map_err(|e| e.to_json())?;
            return Ok(user.clone());
        }
        Err(e) => {
            return Err(e.to_json());
        }
    }
}
