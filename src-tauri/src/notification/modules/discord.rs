use serde_json::json;

use crate::{notification::client::NotifyClient, qf_client::types::user, utils::modules::logger};

#[derive(Clone, Debug)]
pub struct DiscordModule {
    client: NotifyClient,
    user_name: String,
    profile_picture: String,
    pub debug_id: String,
    component: String,
}

impl DiscordModule {
    pub fn new(client: NotifyClient) -> Self {
        DiscordModule {
            client,
            user_name: "Quantframe".to_string(),
            profile_picture: "https://i.imgur.com/bgR6vAd.png".to_string(),
            debug_id: "DiscordModule".to_string(),
            component: "GUINotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    pub fn create_tag_string(&self, tags: Option<Vec<String>>) -> String {
        if tags.is_none() {
            return "".to_string();
        }
        let tags = tags.unwrap();
        let mut tag_string = String::new();
        for tag in tags {
            if tag == "" {
                continue;
            }
            tag_string.push_str(&format!("<@{}> ", tag));
        }
        tag_string
    }

    fn get_embed_footer(&self) -> serde_json::Value {
        let app = self.client.app.lock().unwrap().clone();
        let packageinfo = app.get_app_info();

        // Create a new Discord notification JSON object
        json!({
            "text": format!("© Quantframe v{} (Kenya-DK)", packageinfo.version.to_string()),
            "icon_url": "https://imgur.com/eNhqdBk.png"
        })
    }

    pub fn send_notification(
        &self,
        webhook: &str,
        title: &str,
        content: &str,
        user_ids: Option<Vec<String>>,
    ) {
        let component = self.get_component("SendNotification");
        let tags = self.create_tag_string(user_ids);

        let title = title.to_string();
        let content = content.to_string();
        let webhook = webhook.to_string();
        let footer = self.get_embed_footer();
        let user_name = self.user_name.clone();
        let profile_picture = self.profile_picture.clone();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();
            // Get the current timestamp formatted 2024-09-04T22:00:00.000Z
            let timestamp = chrono::Local::now()
                .to_utc()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string();

            // Create a new Discord notification JSON object
            let mut body = json!({
                "username": user_name,
                "avatar_url": profile_picture,
                "content": "",
                "embeds": [
                    {
                        "title": title,
                        "description": content,
                        "color": 5814783,
                        "thumbnail": {
                            "url": "https://cataas.com/cat"
                        },
                        "footer": footer,
                        "timestamp": timestamp
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
    pub fn send_embed_notification(
        &self,
        webhook: &str,
        mut embeds: Vec<serde_json::Value>,
    ) {
        let component = self.get_component("SendNotification");
        let webhook = webhook.to_string();
        let user_name = self.user_name.clone();
        let profile_picture = self.profile_picture.clone();
        let footer = self.get_embed_footer();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();

            for embed in embeds.iter_mut() {
                embed["footer"] = footer.clone();
                embed["timestamp"] = json!(chrono::Local::now()
                    .to_utc()
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string());
            }

            // Create a new Discord notification JSON object
            let body = json!({
                "username": user_name,
                "avatar_url": profile_picture,
                "content": "",
                "embeds": embeds
            });
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
