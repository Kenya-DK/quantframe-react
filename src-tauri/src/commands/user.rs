use std::sync::Mutex;

use serde_json::json;
use utils::{get_location, info, Error, LoggerOptions};

use crate::app::client::AppState;

#[tauri::command]
pub async fn user_set_status(
    status: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app_state = app.lock().unwrap().clone();
    if app_state.wfm_socket.is_none() {
        return Err(Error::new(
            "User:SetStatus",
            "WebSocket is not connected, please login first.",
            get_location!(),
        ));
    }
    let wfm_socket = app_state.wfm_socket.as_ref().unwrap();
    match wfm_socket.send_request("@WS/USER/SET_STATUS", json!(status)) {
        Ok(_) => {
            info(
                "Commands:UserSetStatus",
                &format!("User status set to {}", status),
                &LoggerOptions::default(),
            );
        }
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
