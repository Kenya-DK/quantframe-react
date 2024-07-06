use std::{process::Command, sync::{Mutex, Arc}};

use crate::{notification::client::NotifyClient, utils::modules::logger};

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
)  {
  let notify = notify.lock().unwrap();
  let app = app.lock().unwrap();
  let path = logger::export_logs(app.get_app_info());
  notify.system().send_notification("Export Logs", &format!("Logs exported to: {}", path), None, None);

}
