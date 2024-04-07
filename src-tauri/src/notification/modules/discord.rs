use crate::notification::client::NotifyClient;

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
}
