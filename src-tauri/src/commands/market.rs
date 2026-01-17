use std::sync::Mutex;

use qf_api::types::*;
use serde_json::Value;
use utils::{get_location, Error};

use crate::{
    app::client::AppState,
    utils::ErrorFromExt,
};

#[tauri::command]
pub async fn get_user_activity(
    query: UserActivityQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Value, Error> {
    let app_state = app.lock().unwrap().clone();
    match app_state.qf_client.market().get_user_activity(query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_qf(
                "UserActivityLookup",
                "Failed to lookup user activity: {}",
                e,
                get_location!(),
            )
            .log("user_activity_lookup.log");
            return Err(error);
        }
    };
}
