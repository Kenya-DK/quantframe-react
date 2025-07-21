use std::sync::{Arc, Mutex, Weak};

use serde_json::{json, Value};
use tauri::Emitter;

use crate::{
    notification::{client::NotificationState, enums::*},
    utils::modules::logger::{self, LoggerOptions},
    APP,
};

#[derive(Debug)]
pub struct GuiModel {
    client: Weak<NotificationState>,
}

impl GuiModel {
    /**
     * Creates a new `GuiModel` with an empty Authentication list.
     * The `client` parameter is an `Arc<NotificationState<State>>` that allows the route
     */
    pub fn new(client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    pub fn send_event(&self, event: UIEvent, data: Option<Value>) {
        let app = APP.get().expect("App not initialized");
        match app.emit("message", json!({ "event": event.as_str(), "data":  data })) {
            Ok(_) => {
                logger::info(
                    "Notification:GuiModel:SendEvent",
                    format!("Event: {}", event.as_str()).as_str(),
                    LoggerOptions::default(),
                );
            }
            Err(e) => {
                logger::error(
                    "Notification:GuiModel:SendEvent",
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
                    "Notification:GuiModel:SendEventUpdate",
                    format!("Event: {}", event.as_str()).as_str(),
                    LoggerOptions::default(),
                );
            }
            Err(e) => {
                logger::error(
                    "Notification:GuiModel:SendEventUpdate",
                    format!("Event: {}", e).as_str(),
                    LoggerOptions::default(),
                );
            }
        }
    }
    /**
     * Creates a new `GuiModel` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &GuiModel, client: Arc<NotificationState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
