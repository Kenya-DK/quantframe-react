use std::sync::{Arc, Mutex};

use crate::{auth::AuthState, wfm_client::WFMClientState, error::{AppError, GetErrorInfo}};

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    auth: tauri::State<'_, Arc<Mutex<AuthState>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<AuthState, AppError> {
    let auth = auth.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    match wfm.login(email, password).await {
        Ok(user) => {
            user.save_to_file()?;
            return Ok(user.clone());
        }
        Err(e) => {
            let component = e.component();
            let cause = e.cause();
            let backtrace = e.backtrace();
            let log_level = e.log_level();
            crate::logger::dolog(
                log_level,
                component.as_str(),
                format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
                true,
                Some(format!("error_{}_{}.log", component, chrono::Local::now().format("%Y-%m-%d")).as_str()),
            );  
        }
    }
    Ok(auth.clone())
}
