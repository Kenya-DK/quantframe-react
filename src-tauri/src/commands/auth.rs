use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde_json::{json, Value};
use eyre::eyre;

use crate::{
    auth::AuthState,
    error::{self, AppError},
    logger,
    wfm_client::client::WFMClient,
};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_auth.log".to_string()));

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<AuthState, AppError> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    match wfm.auth().login(email, password).await {
        Ok(user) => {
            if user.access_token.is_none() {
                logger::critical(
                    "WarframeMarket",
                    "No access token found for user",
                    true,
                    Some(LOG_FILE.lock().unwrap().as_str()),
                );
                return Err(AppError::new(
                    "WarframeMarket",
                    eyre!("No access token found for user"),
                ));
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
            auth.save_to_file()?;
            auth.send_to_window();
            return Ok(auth.clone());
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn update_user_status(
    status: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");
    auth.status = Some(status);
    auth.save_to_file()?;
    auth.send_to_window();
    Ok(())
}
#[tauri::command]
pub async fn logout(
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");
    auth.access_token = None;
    auth.avatar = None;
    auth.ingame_name = "".to_string();
    auth.id = "".to_string();
    auth.save_to_file()?;
    auth.send_to_window();
    Ok(())
}
