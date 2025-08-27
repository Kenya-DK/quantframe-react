use std::sync::Mutex;

use qf_api::types::*;
use utils::{get_location, Error};

use crate::{app::client::AppState, utils::ErrorFromExt};

#[tauri::command]
pub async fn alert_get_alerts(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<Alert>, Error> {
    let app = app.lock()?.clone();
    match app.qf_client.alert().get_alerts().await {
        Ok(alerts) => Ok(alerts),
        Err(e) => {
            let err = Error::from_qf(
                "AlertGetAlerts",
                "Failed to fetch alerts",
                e,
                get_location!(),
            );
            err.log(Some("alert_get_alerts.log"));
            return Err(err);
        }
    }
}
