use std::{
    f32::consts::E,
    sync::{Arc, Mutex},
};

use polars::export::rayon::string;
use serde_json::Value;

use crate::{
    auth::AuthState,
    error::{self, AppError, GetErrorInfo},
    wfm_client::WFMClientState,
};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
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
