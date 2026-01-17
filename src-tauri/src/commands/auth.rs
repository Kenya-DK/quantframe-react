use std::sync::{Arc, Mutex};

use serde_json::json;
use tauri::http::status;

use crate::{
    auth::AuthState,
    qf_client::client::QFClient,
    utils::modules::{
        error::{self, AppError, ErrorApiResponse},
        states,
    },
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn auth_login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<AuthState, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();
    let wfm = states::wfm_client().expect("Failed to get WFM client");
    let mut auth_state = auth.lock()?.clone();

    // Login to Warframe Market
    let (wfm_user, wfm_token) = match wfm.auth().login(&email, &password).await {
        Ok((user, token)) => (user, token),
        Err(e) => {
            error::create_log_file("auth_login.log", &e);
            return Err(e);
        }
    };

    if wfm_user.anonymous || wfm_user.banned || !wfm_user.verification {
        return Ok(auth_state);
    }

    qf.analytics().add_metric("Auth_LoginWFM", "manual");
    auth_state.update_from_wfm_user_profile(&wfm_user, wfm_token.clone());
    // Login/Register to Quantframe
    let (qf_user, qf_token) = match qf
        .auth()
        .login_or_register(&auth_state.get_username(), &auth_state.get_password())
        .await
    {
        Ok(user) => (Some(user.clone()), user.token),
        Err(e) => {
            let json = e.extra_data();
            if json.is_none() {
                return Err(e);
            }
            let json = json.unwrap()["ApiError"].clone();
            let ex: ErrorApiResponse = serde_json::from_value(json).unwrap();
            let msg = ex.messages.get(0);
            if msg.is_none() {
                error::create_log_file("auth_login.log", &e);
                return Err(e);
            }
            let msg = msg.unwrap().to_owned();
            (None, Some(msg))
        }
    };

    match qf.analytics().init() {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("analytics.log", &e);
            return Err(e);
        }
    }
    // Setup WebSocket Client
    match wfm
        .auth()
        .setup_websocket(&auth_state.wfm_access_token.clone().unwrap())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("ws_setup.log", &e);
            return Err(e);
        }
    }
    // Update The current AuthState
    let arced_mutex = Arc::clone(&auth);
    let mut auth = arced_mutex.lock().expect("Could not lock auth");
    auth.update_from_wfm_user_profile(&wfm_user, wfm_token.clone());
    auth.update_from_qf_user_profile(&qf_user.unwrap(), qf_token);
    auth.save_to_file()?;
    Ok(auth.clone())
}

#[tauri::command]
pub async fn auth_set_status(
    status: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<(), AppError> {
    let wfm = wfm.lock().expect("Could not lock wfm");
    wfm.auth().set_user_status(status)?;
    Ok(())
}

#[tauri::command]
pub async fn auth_logout(
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();
    let arced_mutex = Arc::clone(&auth);
    let wfm = states::wfm_client().expect("Failed to get WFM client");
    match qf
        .analytics()
        .try_send_analytics("metrics/periodic", 3, json!([{"Auth_Logout": "manual"}]))
        .await
    {
        Ok(_) => {
            wfm.auth().stop_websocket();
            qf.analytics().set_send_metrics(false);
            qf.auth().logout().await?;
            let mut auth = arced_mutex.lock().expect("Could not lock auth");
            auth.reset();
            auth.save_to_file()?;
        }
        Err(_) => {}
    };
    Ok(())
}
