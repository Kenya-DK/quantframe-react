use crate::notification::client::NotifyClient;


#[derive(Clone, Debug)]
pub struct GUIModule {
    client: NotifyClient,
    pub debug_id: String,
    component: String,
}

impl GUIModule {
    pub fn new(client: NotifyClient) -> Self {
        GUIModule {
            client,
            debug_id: "GUIModule".to_string(),
            component: "GUINotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_gui_module(self.clone());
    }
}
