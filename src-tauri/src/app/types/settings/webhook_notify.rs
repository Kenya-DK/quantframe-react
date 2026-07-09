use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::{get_location, info, Error, LoggerOptions};

use crate::APP;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebHookNotify {
    pub enabled: bool,
    pub url: String,
}
impl WebHookNotify {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            enabled: false,
            url: url.into(),
        }
    }
    pub fn send(&self, value: Value) {
        if self.url.is_empty() {
            return;
        }
        let url = self.url.clone();
        let tauri_app = APP.get().expect("App handle not found");
        let app_info = tauri_app.package_info().clone();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();
            let res = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    format!(
                        "{} v{} ({})",
                        app_info.name, app_info.version, app_info.authors
                    ),
                )
                .json(&value)
                .send()
                .await;
            match res {
                Ok(_) => {
                    info(
                        "Helper",
                        &format!("Message sent to webhook: {}", url),
                        &LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    let err = Error::new(
                        "WebhookNotificationError",
                        &format!("{:?}", e),
                        get_location!(),
                    );
                    err.log("webhook_notification.log");
                }
            }
        });
    }
}
