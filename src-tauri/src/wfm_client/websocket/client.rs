use std::sync::{Arc, Mutex};

use futures_util::stream::AbortHandle;

use crate::{
    utils::modules::WsError,
    wfm_client::{
        enums::ApiVersion,
        websocket::{MessageSender, Route, Router, WsMessage},
    },
};

// The actual WebSocket client (runtime instance)
#[derive(Clone, Debug)]
pub struct WsClient {
    pub sender: Arc<Mutex<Option<MessageSender>>>,
    pub(crate) abort_handle: Arc<Mutex<Option<AbortHandle>>>,
}

impl WsClient {
    pub(crate) fn send_disconnect_message(
        router: &Router,
        message: &WsMessage,
        sender: &MessageSender,
    ) -> Result<(), WsError> {
        router.route_message(&message, sender)
    }
    pub(crate) fn send_connect_message(
        router: &Router,
        sender: &MessageSender,
        version: ApiVersion,
    ) -> Result<(), WsError> {
        let message = WsMessage::connect(version);
        router.route_message(&message, sender)
    }
    pub(crate) fn handle_text_message(
        router: &Router,
        text: &str,
        sender: &MessageSender,
        version: ApiVersion,
    ) -> Result<(), WsError> {
        let message: WsMessage = serde_json::from_str(text)
            .map_err(|_| WsError::InvalidMessageReceived(text.to_string()))?;

        router.route_message(&message.set_version(version), sender)
    }

    pub fn send_request(&self, route: &str, payload: serde_json::Value) -> Result<String, WsError> {
        let route_parsed =
            Route::parse(route).map_err(|_| WsError::InvalidPath(route.to_string()))?;
        if route_parsed.protocol == "internal" {
            return Err(WsError::ReservedPath(
                "Can't send on internal routes".to_string(),
            ));
        }
        let sender_guard = self.sender.lock().unwrap();
        if let Some(sender) = sender_guard.as_ref() {
            sender.send_request(route, payload)
        } else {
            Err(WsError::NotConnected)
        }
    }

    pub fn disconnect(&self) -> Result<(), WsError> {
        if let Some(abort_handle) = self.abort_handle.lock().unwrap().take() {
            abort_handle.abort();
            Ok(())
        } else {
            Err(WsError::NotConnected)
        }
    }
}
