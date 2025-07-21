use std::sync::Mutex;

use qf_api::types::*;

use crate::{
    app::client::AppState,
    utils::modules::{
        error::AppError,
        logger::{log_error, LoggerOptions},
    },
};

#[tauri::command]
pub async fn alert_get_alerts(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<Alert>, AppError> {
    let app = app.lock().unwrap().clone();
    match app.qf_client.alert().get_alerts().await {
        Ok(alerts) => Ok(alerts),
        Err(e) => {
            let err = AppError::from_qf("AlertGetAlerts", "Failed to fetch alerts", e);
            log_error(
                &err,
                LoggerOptions::default().set_file("alert_get_alerts.log"),
            );
            return Err(err);
        }
    }
}
