use serde_json::Value;
use utils::{Error, ZipOptions};

use crate::{helper, APP};

#[tauri::command]
pub async fn log_export() -> Result<String, Error> {
    let app = APP.get().expect("App handle not found");
    let info = app.package_info().clone();
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();

    let version = info.version.to_string();
    let app_path = helper::get_app_storage_path();

    utils::export_cached_logs(&app_path)?;

    let zip_path =
        helper::get_desktop_path().join(format!("{} v{} {} Logs.zip", info.name, version, date));
    let path = ZipOptions::new()
        .exclude_patterns(&["EBWebView/"])
        .mask_properties(&["check_code", "qf_token", "wfm_token", "webhook"])
        .create_zip(app_path, zip_path)?;
    Ok(path)
}

#[tauri::command]
pub async fn log(
    cause: String,
    component: String,
    location: String,
    log_level: String,
    message: String,
    context: Option<Value>,
) -> Result<(), Error> {
    let error = Error::new(component, message, location)
        .with_context(context.unwrap_or_default())
        .with_cause(cause)
        .set_log_level(utils::LogLevel::from_str(&log_level));

    error.log("log.log");
    Ok(())
}
