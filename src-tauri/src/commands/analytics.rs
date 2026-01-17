use std::sync::Mutex;

use utils::Error;

use crate::app::client::AppState;

#[tauri::command]
pub fn analytics_set_last_user_activity(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?;
    app.qf_client.analytics().set_last_user_activity();
    Ok(())
}

#[tauri::command]
pub fn analytics_add_metric(
    key: String,
    value: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    app.qf_client.analytics().add_metric(&key, &value);
    Ok(())
}
