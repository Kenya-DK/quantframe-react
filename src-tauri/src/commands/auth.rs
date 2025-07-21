use std::sync::Mutex;

use crate::{
    app::{client::AppState, User},
    utils::modules::{
        error::AppError,
        logger::{self, log_error, LoggerOptions},
    },
};

#[tauri::command]
pub async fn auth_me(app: tauri::State<'_, Mutex<AppState>>) -> Result<User, AppError> {
    let app = app.lock().unwrap();
    Ok(app.user.clone())
}
#[tauri::command]
pub async fn auth_login(
    email: String,
    password: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<User, AppError> {
    let app_state = app.lock().unwrap().clone();

    let (qf_client, wfm_client, updated_user, ws) = match app_state.login(&email, &password).await {
        Ok((qf_client, wfm_client, user, ws)) => (qf_client, wfm_client, user, ws),
        Err(e) => {
            log_error(&e, LoggerOptions::default().set_file("auth_login.log"));
            return Err(e);
        }
    };
    // Update The current AuthState
    let mut app = app.lock().expect("Could not lock auth");
    logger::info(
        "Commands:AuthLogin",
        &format!("User {} logged in successfully", updated_user.wfm_username),
        LoggerOptions::default().set_file("auth_login.log"),
    );
    qf_client
        .analytics()
        .add_metric("login", updated_user.wfm_username.as_str());
    qf_client
        .analytics()
        .start()
        .map_err(|e| AppError::from_qf("AppState:Validate", "Failed to start QF analytics", e))?;
    app.wfm_client = wfm_client;
    app.user = updated_user.clone();
    app.qf_client = qf_client;
    app.wfm_socket = Some(ws);
    Ok(updated_user)
}

#[tauri::command]
pub async fn auth_logout(app: tauri::State<'_, Mutex<AppState>>) -> Result<User, AppError> {
    let app_state = app.lock().unwrap().clone();

    // Stop the WebSocket if it exists
    if let Some(ws) = &app_state.wfm_socket {
        match ws.disconnect() {
            Ok(_) => logger::info(
                "Commands:AuthLogout",
                "WebSocket disconnected successfully",
                LoggerOptions::default().set_file("auth_logout.log"),
            ),
            Err(e) => {
                let err = AppError::new(
                    "Commands:AuthLogout",
                    &format!("Failed to close WebSocket: {:?}", e),
                );
                err.log(Some("auth_logout.log"));
                return Err(err);
            }
        }
    }

    app_state
        .qf_client
        .analytics()
        .add_metric("logout", app_state.user.wfm_username.clone());
    match app_state.qf_client.analytics().send_current_metrics().await {
        Ok(_) => logger::info(
            "Commands:AuthLogout",
            "Successfully sent current metrics",
            LoggerOptions::default(),
        ),
        Err(e) => {
            let err = AppError::from_qf("Commands:AuthLogout", "Failed to send current metrics", e);
            err.log(Some("auth_logout.log"));
            return Err(err);
        }
    }
    // Stop QF analytics if it exists
    app_state.qf_client.analytics().stop();

    let new_user = User::default();
    new_user
        .save()
        .map_err(|e| {
            e.log(Some("auth_logout.log"));
        })
        .unwrap();
    // Update The current AuthState
    let mut app = app.lock().expect("Could not lock auth");
    app.user = new_user.clone();
    Ok(new_user)
}
