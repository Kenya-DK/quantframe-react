use std::sync::{Arc, Mutex};

use eyre::eyre;
use once_cell::sync::Lazy;

use crate::{
    auth::AuthState,
    error::{self, AppError},
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
) -> Result<AuthState, AppError> {
    let wfm = wfm.lock()?.clone();
    let qf = qf.lock()?.clone();
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth").clone();

    let mut created_at = Some(chrono::Utc::now().timestamp());

    let mut wfm_user = None;

    match wfm.auth().login(email.clone(), password.clone()).await {
        Ok(user) => {
            wfm_user = Some(user.clone());
            // Check if user is banned from WarframeMarket
            if user.banned {
                let error = AppError::new(
                    "WarframeMarket",
                    eyre!(
                        "User is banned from WarframeMarket reason: {:?}",
                        user.ban_reason
                    ),
                );
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &error);
                return Err(error);
            }
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
    let wfm_user_un = wfm_user.unwrap();

    // Check if first time starting app
    if auth.created_at.is_some() {
        created_at = auth.created_at;
    } else {
        let password = created_at.unwrap().to_string();
        match qf
            .auth()
            .registration(
                wfm_user_un.id.clone(),
                wfm_user_un.avatar,
                wfm_user_un.ingame_name,
                wfm_user_un.locale,
                wfm_user_un.platform,
                wfm_user_un.region,
                password.clone(),
                password.clone(),
            )
            .await
        {
            Ok(user) => {
                logger::info_con("QuantFrame", "User registered");
            }
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e);
            }
        }
    }

    let password = created_at.unwrap().to_string();

    
    // Validate user in QuantFrame api
    match qf.auth().login(wfm_user_un.id, password).await {
        Ok(user) => {
            auth.ingame_name = user.ingame_name;
            auth.locale = user.locale;
            auth.region = user.region;
            auth.wfm_access_token = wfm_user_un.access_token;
            auth.qf_access_token = user.qf_access_token;
            auth.save_to_file().map_err(|e| e.to_json()).map_err(|e| AppError::new("Auth", eyre!(e.to_string())))?;
            auth.send_to_window();
            return Ok(auth.clone());
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}

pub async fn validate(auth: AuthState, wfm: WFMClient, qf: QFClient) -> Result<bool, AppError> {
    // Check WarframeMarket Credentials are valid

    let is_validate = match wfm.auth().validate().await {
        Ok(is_validate) => is_validate,
        Err(e) => {
            logger::warning_con(
                "WarframeMarket",
                format!("WarframeMarket Credentials are invalid").as_str(),
            );
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    Ok(is_validate)
}