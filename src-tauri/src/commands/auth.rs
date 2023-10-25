use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{auth::AuthState, error::{self}, wfm_client::client::WFMClient, logger};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<AuthState, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    match wfm.auth().login(email, password).await {
        Ok(user) => {

            if user.access_token.is_none() {
                logger::critical("WarframeMarket", "No access token found for user", true, Some(wfm.log_file.as_str()));
                return Err(json!({"error": "No access token found for user"}));
            }

            let arced_mutex = Arc::clone(&auth);
            let mut auth = arced_mutex.lock().expect("Could not lock auth");
            auth.banned = user.banned;
            auth.id = user.id;
            auth.access_token = user.access_token;
            auth.avatar = user.avatar;
            auth.ingame_name = user.ingame_name;
            auth.locale = user.locale;
            auth.platform = user.platform;
            auth.region = user.region;
            auth.role = user.role;
            auth.save_to_file().map_err(|e| e.to_json())?;
            auth.send_to_window();
            return Ok(auth.clone());
        }
        Err(e) => {
            let error = e.to_json();
            error::create_log_file("login.log".to_string(), &e);
            return Err(error);
        }
    }
}
