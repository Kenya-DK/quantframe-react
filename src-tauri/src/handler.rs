use std::{env, fs};

use tauri::{api::notification::Notification, AppHandle, Window};

#[derive(Clone,Debug)]
pub struct MonitorHandler {
    pub main_window: Window,
    pub app_handler: AppHandle,
}

impl MonitorHandler {
    pub fn new(window: Window, app_handler: AppHandle) -> Self {
        MonitorHandler {
            main_window: window,
            app_handler,
        }
    }
    pub fn show_notification(&self, title: &str, body: &str,icon: Option<&str>, sound: Option<&str>) {
        let sound = match sound {
            Some(s) => s,
            None => "Default",
        };
        let notification = Notification::new(&self.app_handler.config().tauri.bundle.identifier)
            .title(title)
            .body(body)
            .icon(icon.unwrap_or("assets/icons/icon.png"))
            .sound(sound);
        notification.show().unwrap();
    }
}
