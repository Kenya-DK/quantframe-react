use std::sync::{Arc, Mutex};

use crate::{auth::AuthState, structs::GlobleError, wfm_client::WFMClientState, error::AppError};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<AuthState, AppError> {
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    match wfm.login(email, password).await {
        Ok(user) => {
            user.save_to_file()?;
            return Ok(user.clone());
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
    Ok(auth.clone())
}
