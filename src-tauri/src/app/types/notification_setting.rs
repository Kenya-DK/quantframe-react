use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utils::{get_location, info, Error, LoggerOptions};

use crate::{play_sound, APP};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemNotify {
    pub enabled: bool,
    pub title: String,
    pub content: String,
    pub sound_file: String,
    pub volume: f32,
}
impl SystemNotify {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        sound_file: impl Into<String>,
        volume: f32,
    ) -> Self {
        Self {
            enabled: true,
            title: title.into(),
            content: content.into(),
            sound_file: sound_file.into(),
            volume,
        }
    }
    pub fn send(&self, variables: &HashMap<String, String>) {
        let mut title = self.title.clone();
        let mut content = self.content.clone();
        for (k, v) in variables.iter() {
            title = title.replace(&format!("{}", k), v);
            content = content.replace(&format!("{}", k), v);
        }
        if !self.sound_file.is_empty() && self.sound_file != "none" {
            play_sound!(self.sound_file.clone(), self.volume);
        }
        #[cfg(target_os = "windows")]
        {
            use crate::send_system_notification;
            send_system_notification!(&title, &content, None, None);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscordNotify {
    pub enabled: bool,
    pub content: String,
    pub webhook: String,
    pub user_ids: Vec<String>,
}
impl DiscordNotify {
    pub fn new(
        content: impl Into<String>,
        webhook: impl Into<String>,
        user_ids: Vec<String>,
    ) -> Self {
        Self {
            enabled: true,
            content: content.into(),
            webhook: webhook.into(),
            user_ids,
        }
    }
    pub fn send(&self, variables: &HashMap<String, String>) {
        let variables = &mut variables.clone();
        if self.webhook.is_empty() {
            return;
        }
        let mut tags = Vec::new();
        for user_id in &self.user_ids {
            if !user_id.is_empty() {
                tags.push(format!("<@{}>", user_id));
            }
        }
        if !tags.is_empty() {
            variables.insert("<MENTION>".to_string(), tags.join(" ").to_string());
        } else {
            variables.insert("<MENTION>".to_string(), "".to_string());
        }
        let mut content = self.content.clone();
        for (k, v) in variables.iter() {
            content = content.replace(&format!("{}", k), v);
        }
        let webhook = self.webhook.clone();
        let tauri_app = APP.get().expect("App handle not found");
        let app_info = tauri_app.package_info().clone();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();
            // Get the current timestamp formatted 2024-09-04T22:00:00.000Z
            let timestamp = chrono::Local::now()
                .to_utc()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string();
            // Create a new Discord notification JSON object
            let body = json!({
                "content": content,
                "embeds": [
                    {
                        "description": "",
                        "color": 5814783,
                        "footer": {
                            "text": format!("{} v{} ({})",app_info.name, app_info.version, app_info.authors),
                            "icon_url": "https://raw.githubusercontent.com/Kenya-DK/quantframe-react/refs/heads/main/app-icon.png"
                        },
                        "timestamp": timestamp
                    }
                ]
            });
            let res = client.post(webhook).json(&body).send().await;
            match res {
                Ok(_) => {
                    info(
                        "Helper",
                        "Message sent to discord",
                        &LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    let err = Error::new(
                        "DiscordNotificationError",
                        &format!("{:?}", e),
                        get_location!(),
                    );
                    err.log("discord_notification.log");
                }
            }
        });
    }
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSetting {
    system_notify: SystemNotify,
    discord_notify: DiscordNotify,
    webhook_notify: WebHookNotify,
}
impl NotificationSetting {
    pub fn new(
        discord_notify: DiscordNotify,
        system_notify: SystemNotify,
        webhook_notify: WebHookNotify,
    ) -> Self {
        Self {
            discord_notify,
            system_notify,
            webhook_notify,
        }
    }
}

impl NotificationSetting {
    pub fn send(&self, variables: &HashMap<String, String>, value: Option<Value>) {
        if self.system_notify.enabled {
            self.system_notify.send(variables);
        }
        if self.discord_notify.enabled {
            self.discord_notify.send(variables);
        }
        if self.webhook_notify.enabled && value.is_some() {
            self.webhook_notify.send(value.unwrap());
        }
    }
}
