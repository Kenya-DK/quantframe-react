use futures_util::stream::{AbortHandle, Abortable};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use crate::utils::modules::WsError;
use crate::wfm_client::enums::ApiVersion;
use crate::wfm_client::websocket::{MessageSender, Route, Router, WsClient, WsMessage};

// WebSocket client builder
pub struct WsClientBuilder {
    version: ApiVersion,
    router: Router,
    token: String,
    device_id: String,
}

impl WsClientBuilder {
    pub(crate) fn new(version: ApiVersion, token: String, device_id: String) -> Self {
        Self {
            version,
            router: Router::new(),
            token,
            device_id,
        }
    }

    /// Register a callback for a specific path with optional parameter
    ///
    /// Examples:
    /// - `register_callback("cmd/subscribe/newOrders", callback)` - matches any parameter
    /// - `register_callback("cmd/subscribe/newOrders:ok", callback)` - matches only :ok parameter
    pub fn register_callback<F>(mut self, path: &str, callback: F) -> Result<Self, WsError>
    where
        F: Fn(&WsMessage, &Route, &MessageSender) -> Result<(), WsError> + Send + Sync + 'static,
    {
        self.router.register(path, Arc::new(callback))?;
        Ok(self)
    }

    /// Get list of paths reserved by the client for internal usage
    pub fn get_reserved_paths() -> Vec<&'static str> {
        Router::get_reserved_paths()
    }

    /// Build and start the WebSocket client
    pub async fn build(self) -> Result<WsClient, WsError> {
        let router = Arc::new(self.router);
        let sender_holder = Arc::new(Mutex::new(None));
        let abort_handle_holder = Arc::new(Mutex::new(None));

        tokio::spawn({
            let sender_holder = Arc::clone(&sender_holder);
            let router = Arc::clone(&router);
            let abort_handle_holder = Arc::clone(&abort_handle_holder);

            // let version = self.version.clone();
            async move {
                let version = &self.version;
                let ws_url = version.websocket_url();
                loop {
                    let mut request = ws_url.into_client_request().unwrap();
                    let headers = request.headers_mut();
                    if version == &ApiVersion::V2 {
                        headers.append("Sec-WebSocket-Protocol", "wfm".parse().unwrap());
                    } else if version == &ApiVersion::V1 {
                        headers.append("cookie", format!("JWT={}", self.token).parse().unwrap());
                    }
                    headers.append("User-Agent", "wf-market-rs".parse().unwrap());

                    // println!("Attempting to connect to WebSocket...");

                    match connect_async(request).await {
                        Ok((ws_stream, _)) => {
                            // println!("Connected to WebSocket.");
                            let ws_error = Arc::new(Mutex::new(None));
                            let ws_error_write = Arc::clone(&ws_error);
                            let ws_error_read = Arc::clone(&ws_error);
                            let (mut write, read) = ws_stream.split();
                            let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();
                            let sender = MessageSender {
                                version: version.clone(),
                                tx: tx.clone(),
                            };

                            // Send connection message to the router
                            WsClient::send_connect_message(&router, &sender, version.clone())
                                .unwrap();

                            // Send authentication
                            if version == &ApiVersion::V2 {
                                let auth_payload = json!({
                                    "token": self.token,
                                    "deviceId": self.device_id,
                                });
                                match sender.send_request("@wfm|cmd/auth/signIn", auth_payload) {
                                    Ok(_) => {
                                        println!("Authentication request sent successfully.");
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to send authentication request: {:?}", e);
                                        continue; // Retry connection
                                    }
                                }
                            }

                            *sender_holder.lock().unwrap() = Some(sender.clone());

                            // Create an abort handle to control the write task
                            let (abort_handle, abort_registration) = AbortHandle::new_pair();
                            *abort_handle_holder.lock().unwrap() = Some(abort_handle.clone());

                            // Write task (wrapped in Abortable) Is responsible for sending messages
                            // It will be aborted if the read task fails or ends
                            let write_task = tokio::spawn(Abortable::new(
                                async move {
                                    let ws_error_write = Arc::clone(&ws_error_write);
                                    while let Some(msg) = rx.recv().await {
                                        if let Ok(json) = serde_json::to_string(&msg) {
                                            if let Err(e) = write
                                                .send(Message::Text(Utf8Bytes::from(json)))
                                                .await
                                            {
                                                eprintln!("Write failed: {}", e);
                                                *ws_error_write.lock().unwrap() = Some(e);
                                                break;
                                            }
                                        }
                                    }
                                },
                                abort_registration,
                            ));

                            // Read task (will trigger abort on write if it fails or ends)
                            let read_task = tokio::spawn({
                                let sender = sender.clone();
                                let version = version.clone();
                                let router = Arc::clone(&router);
                                let abort_handle = abort_handle.clone(); // Move handle in
                                let mut read = read;

                                async move {
                                    let ws_error_read = Arc::clone(&ws_error_read);
                                    while let Some(msg) = read.next().await {
                                        match msg {
                                            Ok(Message::Text(text)) => {
                                                if let Err(e) = WsClient::handle_text_message(
                                                    &router,
                                                    &text,
                                                    &sender,
                                                    version.clone(),
                                                ) {
                                                    eprintln!("Handle error: {:?}", e);
                                                }
                                            }
                                            Ok(Message::Close(_)) => {
                                                println!("Connection closed by server.");
                                                break;
                                            }
                                            Ok(_) => (),
                                            Err(e) => {
                                                eprintln!("Read error: {}", e);
                                                *ws_error_read.lock().unwrap() = Some(e);
                                                break;
                                            }
                                        }
                                    }

                                    // If we exit the read loop, abort the write task
                                    abort_handle.abort();
                                }
                            });

                            // Wait for both tasks
                            let _ = tokio::join!(read_task, write_task);
                            // Send a message to the sender to indicate disconnection
                            WsClient::send_disconnect_message(
                                &router,
                                &WsMessage::disconnect(
                                    format!(
                                        "Connection lost: {:?} will retry in 5 seconds",
                                        ws_error.lock().unwrap()
                                    ),
                                    self.version.clone(),
                                ),
                                &sender,
                            )
                            .unwrap();
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }

                        Err(err) => {
                            eprintln!("WebSocket connection failed: {}", err);
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        Ok(WsClient {
            sender: Arc::clone(&sender_holder),
            abort_handle: Arc::clone(&abort_handle_holder),
        })
    }
}
