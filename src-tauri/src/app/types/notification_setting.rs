use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSetting {
    pub discord_notify: bool,
    pub system_notify: bool,
    pub content: String,
    pub title: String,
    // Use For Discord
    pub webhook: Option<String>,
    pub user_ids: Option<Vec<String>>,
}
impl NotificationSetting {
    pub fn new(
        discord_notify: bool,
        system_notify: bool,
        content: &str,
        title: &str,
        webhook: Option<String>,
        user_ids: Option<Vec<String>>,
    ) -> Self {
        Self {
            discord_notify,
            system_notify,
            content: content.to_string(),
            title: title.to_string(),
            webhook,
            user_ids,
        }
    }
}

impl NotificationSetting {
    pub fn is_enabled(&self) -> bool {
        self.discord_notify || self.system_notify
    }
}
