use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use serde_json::Value;

use crate::{
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::log_level::LogLevel,
        modules::logger::{self, LoggerOptions},
    },
};

#[tauri::command]
pub fn log_open_folder() {
    Command::new("explorer")
        .args(["/select,", &logger::get_log_folder().to_str().unwrap()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn log_export(
    app: tauri::State<'_, Arc<Mutex<crate::app::client::AppState>>>,
    notify: tauri::State<'_, Arc<Mutex<NotifyClient>>>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) {
    let notify = notify.lock().unwrap();
    let app = app.lock().unwrap();
    let qf = qf.lock().unwrap();
    let path = logger::export_logs(app.get_app_info());

    notify.system().send_notification(
        "Export Logs",
        &format!("Logs exported to: {}", path),
        None,
        None,
    );
    qf.analytics().add_metric("Log_Export", "manual");
}

#[tauri::command]
pub fn log_send(
    component: String,
    msg: String,
    level: LogLevel,
    console: bool,
    file: Option<String>,
) {
    let mut options = LoggerOptions::default().set_console(console);
    if let Some(file) = file {
        options.set_file(&file);
    }
    logger::dolog(
        level,
        format!("GUI:{}", component).as_str(),
        msg.as_str(),
        options,
    );
}
