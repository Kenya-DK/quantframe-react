use std::sync::{Arc, Mutex, Weak};

use crate::app::client::AppState;
use crate::emit_event;
use crate::{
    macros,
    notification::{client::NotificationState, enums::*},
    APP,
};
use serde_json::{json, Value};
use tauri::{Emitter, Manager};
use utils::{error, info, LoggerOptions};

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
        emit_event!(
            "message",
            json!({ "event": event.as_str(), "data": data }),
            "SendEvent"
        );
    }

    pub fn send_event_update(
        &self,
        event: UIEvent,
        operation: UIOperationEvent,
        data: Option<Value>,
    ) {
        emit_event!(
            "message_update",
            json!({ "event": event.as_str(), "operation": operation.as_str(), "data": data }),
            "SendEventUpdate"
        );
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
