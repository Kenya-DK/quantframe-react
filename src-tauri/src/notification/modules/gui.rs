use serde_json::{json, Value};
use tauri::Emitter;

use crate::{
    notification::client::NotifyClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::logger::{self, LoggerOptions},
    },
    APP,
};

#[derive(Clone, Debug)]
pub struct GUIModule {
    client: NotifyClient,
    component: String,
}

impl GUIModule {
    pub fn new(client: NotifyClient) -> Self {
        GUIModule {
            client,
            component: "GUINotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }

    pub fn send_event(&self, event: UIEvent, data: Option<Value>) {
        let app = APP.get().expect("App not initialized");
        match app.emit("message", json!({ "event": event.as_str(), "data":  data })) {
            Ok(_) => {
                logger::info(
                    &self.get_component("SendEvent"),
                    format!("Event: {}", event.as_str()).as_str(),
                    LoggerOptions::default(),
                );
            }
            Err(e) => {
                logger::error(
                    &self.get_component("SendEvent"),
                    format!("Event: {}", e).as_str(),
                    LoggerOptions::default(),
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
        let app = APP.get().expect("App not initialized");
        match app.emit(
            "message_update",
            json!({ "event": event.as_str(), "operation":operation.as_str(), "data":  data }),
        ) {
            Ok(_) => {
                logger::info(
                    &self.get_component("SendEventUpdate"),
                    format!("Event: {}", event.as_str()).as_str(),
                    LoggerOptions::default(),
                );
            }
            Err(e) => {
                logger::error(
                    &self.get_component("SendEventUpdate"),
                    format!("Event: {}", e).as_str(),
                    LoggerOptions::default(),
                );
            }
        }
    }
}
