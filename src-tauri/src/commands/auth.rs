use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::{auth::AuthState, wfm_client::WFMClientState};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<AuthState, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    let mut auth = auth.lock().expect("Could not lock auth").clone();
    match wfm.login(email, password).await {
        Ok(user) => {
            // user.save_to_file().map_err(|e| e.to_json())?;
            auth.set_user(user.clone());
            return Ok(auth.clone());
        }
        Err(e) => {
            let error = e.to_json();
            error::create_log_file("login".to_string(), &e);
            return Err(error);
        }
    }
}
