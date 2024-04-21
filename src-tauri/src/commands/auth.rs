use std::sync::{Arc, Mutex};

use eyre::eyre;


use crate::{
    auth::AuthState, logger, utils::modules::error::{self, AppError}, wfm_client::client::WFMClient
};

#[tauri::command]
pub async fn auth_login(
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
                    Some("auth_login.log"),
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
            auth.role = user.role.clone();

            if user.role != "user" {
                auth.order_limit = 999;
                auth.auctions_limit = 999;
            }

            auth.save_to_file()?;
            return Ok(auth.clone());
        }
        Err(e) => {
            error::create_log_file("auth_login.log".to_string(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn auth_set_status (
    status: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");
    auth.status = Some(status);
    auth.save_to_file()?;
    Ok(())
}

#[tauri::command]
pub async fn auth_logout(auth: tauri::State<'_, Arc<Mutex<AuthState>>>) -> Result<(), AppError> {
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");
    auth.access_token = None;
    auth.avatar = None;
    auth.ingame_name = "".to_string();
    auth.id = "".to_string();
    auth.save_to_file()?;
    Ok(())
}
