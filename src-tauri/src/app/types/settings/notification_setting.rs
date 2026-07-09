use std::collections::HashMap;

use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
