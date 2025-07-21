use std::sync::Mutex;

use serde_json::json;

use crate::{
    app::client::AppState,
    utils::modules::{error::AppError, logger},
};

#[tauri::command]
pub async fn user_set_status(
    status: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let app_state = app.lock().unwrap().clone();
    if app_state.wfm_socket.is_none() {
        return Err(AppError::new(
            "User:SetStatus",
            "WebSocket is not connected, please login first.",
        ));
    }
    let wfm_socket = app_state.wfm_socket.as_ref().unwrap();
    match wfm_socket.send_request("@WS/USER/SET_STATUS", json!(status)) {
        Ok(_) => {
            logger::info(
                "Commands:UserSetStatus",
                &format!("User status set to {}", status),
                logger::LoggerOptions::default(),
            );
        }
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
