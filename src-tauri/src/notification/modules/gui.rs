use serde_json::{json, Value};
use tauri::Manager;

use crate::{
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::logger,
    },
};

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

    pub fn send_event(&self, event: UIEvent, data: Option<Value>) {
        let window = self.client.app_handler.get_window("main").unwrap().clone();
        match window.emit("message", json!({ "event": event.as_str(), "data":  data })) {
            Ok(_) => {
                logger::info_con(
                    &self.get_component("SendEvent"),
                    format!("Event: {}", event.as_str()).as_str(),
                );
            }
            Err(e) => {
                logger::error_con(
                    &self.get_component("SendEvent"),
                    format!("Event: {}", e).as_str(),
                );
            }
        }
    }
    pub fn send_event_update(
        &self,
        event: UIEvent,
        operation: UIOperationEvent,
        data: Option<Value>,
    ) {
        let window = self.client.app_handler.get_window("main").unwrap().clone();
        match window.emit(
            "message_update",
            json!({ "event": event.as_str(), "operation":operation.as_str(), "data":  data }),
        ) {
            Ok(_) => {
                logger::info_con(
                    &self.get_component("SendEventUpdate"),
                    format!("Event: {}", event.as_str()).as_str(),
                );
            }
            Err(e) => {
                logger::error_con(
                    &self.get_component("SendEventUpdate"),
                    format!("Event: {}", e).as_str(),
                );
            }
        }
    }
}
