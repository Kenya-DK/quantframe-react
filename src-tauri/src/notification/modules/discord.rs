use serde_json::json;
use tokio::join;

use crate::{helper, notification::client::NotifyClient, utils::modules::logger};

#[derive(Clone, Debug)]
pub struct DiscordModule {
    client: NotifyClient,
    pub debug_id: String,
    component: String,
}

impl DiscordModule {
    pub fn new(client: NotifyClient) -> Self {
        DiscordModule {
            client,
            debug_id: "DiscordModule".to_string(),
            component: "GUINotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_discord_module(self.clone());
    }
    pub fn create_tag_string(&self, tags: Option<Vec<String>>) -> String {
        if tags.is_none() {
            return "".to_string();
        }
        let tags = tags.unwrap();
        let mut tag_string = String::new();
        for tag in tags {
            tag_string.push_str(&format!("<@{}> ", tag));
        }
        tag_string
    }

    pub fn send_notification(
        &self,
        webhook: &str,
        title: &str,
        content: &str,
        user_ids: Option<Vec<String>>,
    ) {
        let app = self.client.app.lock().unwrap().clone();
        let packageinfo = app.get_app_info();
        let component = self.get_component("SendNotification");
        let tags = self.create_tag_string(user_ids);

        let title = title.to_string();
        let content = content.to_string();
        let webhook = webhook.to_string();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();

            // Create a new Discord notification JSON object
            let mut body = json!({
                "username": "Quantframe",
                "avatar_url": "https://i.imgur.com/bgR6vAd.png",
                "content": "",
                "embeds": [
                    {
                        "title": title,
                        "description": content,
                        "color": 5814783,
                        "footer": {
                            "text": format!("Quantframe v{} BY Kenya-DK", packageinfo.version.to_string()),
                            "timestamp": chrono::Local::now()
                            .naive_utc()
                            .to_string()
                        }
                    }
                ]
            });
            if tags != "" {
                body["content"] = json!(format!("{}", tags).replace("\"", ""));
            }
            let res = client.post(webhook).json(&body).send().await;
            match res {
                Ok(_) => {
                    logger::info_con("Helper", "Message sent to discord");
                }
                Err(e) => {
                    logger::error_con(&component, &format!("Error: {:?}", e));
                }
            }
        });
    }
   
}
