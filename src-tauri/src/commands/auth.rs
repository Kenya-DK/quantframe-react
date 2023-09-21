use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{auth::AuthState, error, wfm_client::client::WFMClient};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<AuthState, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    let mut auth = auth.lock().expect("Could not lock auth").clone();
    match wfm.auth().login(email, password).await {
        Ok(user) => {
            // user.save_to_file().map_err(|e| e.to_json())?;
            auth.set_user(user.clone());
            auth.send_to_window();
            return Ok(auth.clone());
        }
        Err(e) => {
            let error = e.to_json();
            error::create_log_file("login".to_string(), &e);
            return Err(error);
        }
    }
}
