use std::sync::{Arc, Mutex};

use crate::{
    auth::AuthState,
    qf_client::client::QFClient,
    utils::modules::error::{self, AppError, ErrorApiResponse},
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
    let wfm = wfm.lock().expect("Could not lock wfm").clone();
    let qf = qf.lock().expect("Could not lock qf").clone();

    let mut auth_state = auth.lock()?.clone();

    // Login to Warframe Market
    let (wfm_user, wfm_token) = match wfm.auth().login(&email, &password).await {
        Ok((user, token)) => (user, token),
        Err(e) => {
            error::create_log_file("auth_login.log".to_string(), &e);
            return Err(e);
        }
    };

    if wfm_user.anonymous || wfm_user.banned || !wfm_user.verification {
        return Ok(auth_state);
    }

    qf.analytics()
        .add_metric("Auth_LoginWFM", "manual");
    auth_state.update_from_wfm_user_profile(&wfm_user, wfm_token.clone());
    // Login/Register to Quantframe
    let (qf_user, qf_token) = match qf
        .auth()
        .login_or_register(
            &auth_state.get_username(),
            &auth_state.get_password(),
            wfm_user.ingame_name.clone().unwrap().as_str(),
        )
        .await
    {
        Ok(user) => (Some(user.clone()), user.token),
        Err(e) => {
            let json = e.extra_data()["ApiError"].clone();
            let ex: ErrorApiResponse = serde_json::from_value(json).unwrap();
            let msg = ex.messages.get(0);
            if msg.is_none() {
                error::create_log_file("auth_login.log".to_string(), &e);
                return Err(e);
            }
            let msg = msg.unwrap().to_owned();
            (None, Some(msg))
        }
    };

    match qf.analytics().init() {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file("analytics.log".to_string(), &e);
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
    auth.wfm_access_token = None;
    auth.qf_access_token = None;
    auth.check_code = "".to_string();
    auth.avatar = None;
    auth.ingame_name = "".to_string();
    auth.id = "".to_string();
    auth.save_to_file()?;
    Ok(())
}
