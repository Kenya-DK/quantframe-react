use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::{self},
    logger,
    qf_client::client::QFClient,
    wfm_client::client::WFMClient,
};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("commands.log".to_string()));

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<AuthState, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    let qf = qf.lock().expect("Could not lock qf").clone();
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");

    let mut created_at= Some(chrono::Utc::now().timestamp());

    let wfm_user = None;

    match wfm.auth().login(email, password).await {
        Ok(user) => {           
            wfm_user = Some(user.clone());
            // Check if user is banned from WarframeMarket
            if user.banned {
                let error = AppError::new("WarframeMarket", eyre!("User is banned from WarframeMarket reason: {:?}", user.ban_reason));
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &error);
                return Err(error.to_json());
            }
        }
        Err(e) => {
            let error = e.to_json();
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(error);
        }
    }

    // Check if first time starting app
    if auth.created_at.is_some() {
        created_at = auth.created_at;
    } else {
        match qf.auth().registration(
            wfm_user.id,
            wfm_user.avatar,
            wfm_user.ingame_name,
            wfm_user.locale,
            wfm_user.platform,
            wfm_user.region,
            created_at.to_string()
            created_at.to_string()).await {
            Ok(user) => {
                logger::log(
                    LogLevel::Info,
                    &format!("User registered with QuantFrame: {:?}", user),
                );
            }
            Err(e) => {
                let error = e.to_json();
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(error);
            }
        }
    }

    // auth.save_to_file().map_err(|e| e.to_json())?;
    // auth.send_to_window();
    // return Ok(auth.clone());


    if auth.access_token.is_none() {
        return Ok(None);
    }

    // Validate user in QuantFrame api
    match qf.auth().login(wfm_id.to_string(), created_at.to_string()).await {
        Ok(user) => {

        }
        Err(e) => {
            let error = e.to_json();
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(error);
        }
    }

}
