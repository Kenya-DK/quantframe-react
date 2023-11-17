use std::{
    f32::consts::E,
    sync::{Arc, Mutex},
};

use eyre::eyre;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::{self, AppError},
    logger,
    qf_client::{client::QFClient, structs::User},
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
) -> Result<User, Value> {
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    let qf = qf.lock().expect("Could not lock qf").clone();
    let auth_1 = auth.lock().expect("Could not lock qf").clone();
    let packageinfo = crate::PACKAGEINFO
        .lock()
        .unwrap()
        .clone()
        .expect("Could not get package info");

    let current_version = packageinfo.version.to_string();

    let wfm_user = match wfm.auth().login(email.clone(), password.clone()).await {
        Ok(user) => {
            let arced_mutex = Arc::clone(&auth);
            let mut auth = arced_mutex.lock().expect("Could not lock auth");
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
                return Err(error.to_json());
            }
            auth.wfm_access_token = user.access_token.clone();
            user
        }
        Err(e) => {
            // Handle data.Contains("app.form.invalid") || data.Contains("app.account.password_invalid") || data.Contains("app.account.email_not_exist")
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e.to_json());
        }
    };
    let password = match auth_1.code {
        Some(created_at) => created_at,
        None => wfm_user.check_code,
    };

    // Try to register user if not registered
    let is_new = match qf
        .auth()
        .registration(
            wfm_user.id.clone(),
            wfm_user.avatar.clone(),
            wfm_user.ingame_name.clone(),
            wfm_user.locale,
            wfm_user.platform,
            wfm_user.region,
            password.clone(),
            password.clone(),
            current_version.clone(),
        )
        .await
    {
        Ok(user) => {
            logger::info_con(
                "QuantFrame",
                format!("User registered {}", user.ingame_name).as_str(),
            );
            true
        }
        Err(e) => {
            if e.log_level() != logger::LogLevel::Warning {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e.to_json());
            }
            false
        }
    };

    // Validate user in QuantFrame api
    let qf_user = match qf.auth().login(wfm_user.id.clone(), password.clone()).await {
        Ok(user) => {
            let arced_mutex = Arc::clone(&auth);
            let mut auth = arced_mutex.lock().expect("Could not lock auth");
            auth.qf_access_token = user.token.clone();
            auth.wfm_user_id = user.wfm_id.clone();
            auth.ingame_name = user.ingame_name.clone();
            auth.locale = user.locale.clone();
            auth.region = user.region.clone();
            auth.platform = user.platform.clone();
            auth.code = Some(password);
            user
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e.to_json());
        }
    };

    // If user is old, update user info
    if !is_new {
        match qf
            .user()
            .update(
                Some(wfm_user.ingame_name.clone()),
                wfm_user.avatar.clone(),
                Some(current_version.clone()),
                None,
                None,
                None,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e.to_json());
            }
        }
    }

    let arced_mutex = Arc::clone(&auth);
    let auth = arced_mutex.lock().expect("Could not lock auth");
    auth.save_to_file().map_err(|e: AppError| e.to_json())?;
    auth.send_to_window();
    Ok(qf_user)
}

pub async fn validate(wfm: WFMClient, qf: QFClient) -> Result<Option<User>, AppError> {
    // Check WarframeMarket Token is valid
    let is_validate = match wfm.auth().validate().await {
        Ok(is_validate) => is_validate,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            false
        }
    };

    if !is_validate {
        logger::warning_con(
            "WarframeMarket",
            format!("WarframeMarket Token is invalid").as_str(),
        );
        return Ok(None);
    }

    // Check QuantFrame Token is valid
    let user = match qf.auth().validate().await {
        Ok(user) => user,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Ok(None);
        }
    };

    if user.is_none() {
        logger::warning_con(
            "QuantFrame",
            format!("QuantFrame Token is invalid").as_str(),
        );
        return Ok(None);
    }

    Ok(user)
}
