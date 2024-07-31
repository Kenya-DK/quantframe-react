use std::sync::{Arc, Mutex};

use crate::{qf_client::{client::QFClient, modules::analytics::AnalyticsModule}, utils::modules::error::AppError};

#[tauri::command]
pub fn analytics_set_last_user_activity(
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let qf = qf.lock()?;
    qf.analytics().set_last_user_activity();
    Ok(())
}

#[tauri::command]
pub fn analytics_send_metric(
    key: String,
    value: String,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<(), AppError> {
    let qf = qf.lock()?;
    qf.analytics().add_metric(&key, &value);
    Ok(())
}
