use std::{collections::HashMap, sync::Arc};

use crate::{
    utils::modules::WsError,
    wfm_client::{
        enums::ApiVersion,
        websocket::{MessageSender, Route, WsMessage},
    },
};

// Updated callback type to include sender and route info
pub type MessageCallback =
    Arc<dyn Fn(&WsMessage, &Route, &MessageSender) -> Result<(), WsError> + Send + Sync>;

// Internal router
pub(crate) struct Router {
    routes: HashMap<String, MessageCallback>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    // Internal reserved paths that the client uses
    pub fn get_reserved_paths() -> Vec<&'static str> {
        vec!["cmd/auth/signIn"]
    }

    fn is_path_reserved(path: &str) -> bool {
        Self::get_reserved_paths().contains(&path)
    }

    pub fn register(&mut self, path: &str, callback: MessageCallback) -> Result<(), WsError> {
        // Check if path is reserved by the client
        if Self::is_path_reserved(path) {
            return Err(WsError::ReservedPath(path.to_string()));
        }

        // Check if already registered
        if self.routes.contains_key(path) {
            return Err(WsError::AlreadyRegistered(path.to_string()));
        }

        self.routes.insert(path.to_string(), callback);
        Ok(())
    }

    pub fn route_message(
        &self,
        message: &WsMessage,
        sender: &MessageSender,
    ) -> Result<(), WsError> {
        let route = Route::parse(&message.route)?;

        // Handle internal routes first
        if Self::is_path_reserved(route.base_path()) {
            self.handle_internal_route(&route, message, sender)?;
            return Ok(());
        }

        // Try to find callback with routing priority:
        // 1. Exact match with parameter (e.g., "cmd/subscribe/newOrders:ok")
        // 2. Base path match (e.g., "cmd/subscribe/newOrders")

        let callback = self
            .routes
            .get(&route.full_path())
            .or_else(|| self.routes.get(route.base_path()));

        if let Some(callback) = callback {
            callback(message, &route, sender)?;
        } else {
            // Optionally log unhandled routes
            // println!(
            //     "No handler for route: {} (full: {})",
            //     route.base_path(),
            //     route.full_path()
            // );
        }

        Ok(())
    }

    // Handle internal client routes
    fn handle_internal_route(
        &self,
        route: &Route,
        _message: &WsMessage,
        sender: &MessageSender,
    ) -> Result<(), WsError> {
        match route.base_path() {
            "cmd/auth/signIn" => {
                println!(
                    "Handling internal auth sign in with parameter: {:?}",
                    route.parameter
                );
                // Example: Handle different auth responses based on parameter
                match route.parameter.as_deref() {
                    Some("ok") => {
                        if let Some(connected_callback) = self.routes.get("internal/auth_connected")
                        {
                            let route = Route {
                                protocol: "@internal".to_string(),
                                path: "internal/auth_connected".to_string(),
                                parameter: None,
                            };
                            connected_callback(
                                &WsMessage::new(
                                    "@internal|internal/auth_connected",
                                    Some(serde_json::Value::from(true)),
                                    ApiVersion::default(),
                                )
                                .with_id("INTERNAL"),
                                &route,
                                &sender,
                            )?;
                        }
                    }
                    Some("error") => println!("Auth failed"),
                    _ => println!("Unknown auth response"),
                }
            }
            _ => {
                println!(
                    "Unhandled internal route: {} (parameter: {:?})",
                    route.base_path(),
                    route.parameter
                );
            }
        }
        Ok(())
    }
}
