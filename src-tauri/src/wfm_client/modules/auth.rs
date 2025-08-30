use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    utils::{
        enums::{log_level::LogLevel, ui_events::UIEvent},
        modules::{
            error::{self, ApiResult, AppError},
            logger::{self, LoggerOptions},
            states,
        },
    },
    wfm_client::{
        client::WFMClient,
        enums::ApiVersion,
        types::user_profile::UserProfile,
        websocket::{WsClient, WsClientBuilder},
    },
};
#[derive(Clone, Debug)]
pub struct AuthModule {
    pub client: WFMClient,
    is_init: bool,
    pub ws_client: Option<WsClient>,
    component: String,
}
pub fn update_user_status(status: String) {
    let notify = states::notify_client().unwrap();
    notify
        .gui()
        .send_event(UIEvent::UpdateUserStatus, Some(json!(status)));
}
impl AuthModule {
    pub fn new(client: WFMClient) -> Self {
        AuthModule {
            is_init: false,
            ws_client: None,
            client,
            component: "Auth".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }

    pub fn is_logged_in(&self) -> Result<(), AppError> {
        let auth = states::auth()?;
        if !auth.is_logged_in() {
            return Err(AppError::new_with_level(
                &self.get_component("IsLoggedIn"),
                eyre!("User is not logged in"),
                LogLevel::Error,
            ));
        }
        Ok(())
    }

    pub async fn me(&self) -> Result<UserProfile, AppError> {
        match self
            .client
            .get::<UserProfile>("/profile", Some("profile"))
            .await
        {
            Ok(ApiResult::Success(user, _)) => {
                return Ok(user);
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Login"),
                    e,
                    eyre!("There was an error fetching user profile"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        };
    }
    pub fn stop_websocket(&mut self) {
        if let Some(ws_client) = &self.ws_client {
            ws_client.disconnect().unwrap_or_else(|e| {
                logger::error(
                    &self.get_component("StopWebsocket"),
                    &format!("Failed to disconnect WebSocket: {:?}", e),
                    LoggerOptions::default(),
                );
            });
            self.ws_client = None;
            self.is_init = false;
        } else {
            logger::warning(
                &self.get_component("StopWebsocket"),
                "WebSocket client is not initialized, cannot stop WebSocket",
                LoggerOptions::default(),
            );
        }
    }
    pub async fn setup_websocket(&mut self, token: &str) -> Result<(), AppError> {
        if self.is_init {
            return Ok(());
        }
        self.is_init = true;
        let build = WsClientBuilder::new(ApiVersion::V1, token.to_string(), "QF".to_string());
        let client = build
            .register_callback("USER/SET_STATUS", move |msg, _, _| {
                update_user_status(
                    msg.payload
                        .as_ref()
                        .unwrap()
                        .as_str()
                        .unwrap_or("invisible")
                        .to_string(),
                );
                Ok(())
            })
            .unwrap()
            .register_callback("cmd/status/set:ok", move |msg, _, _| {
                match msg.payload.as_ref() {
                    Some(payload) => update_user_status(
                        payload["status"]
                            .as_str()
                            .unwrap_or("invisible")
                            .to_string(),
                    ),
                    None => {}
                }
                Ok(())
            })
            .unwrap()
            .register_callback("event/status/set", move |msg, _, _| {
                match msg.payload.as_ref() {
                    Some(payload) => update_user_status(
                        payload["status"]
                            .as_str()
                            .unwrap_or("invisible")
                            .to_string(),
                    ),

                    None => {}
                }
                Ok(())
            })
            .unwrap()
            .register_callback("chats/NEW_MESSAGE", move |msg, _, _| {
                let notify = states::notify_client().unwrap();
                notify
                    .gui()
                    .send_event(UIEvent::ReceiveMessage, msg.payload.clone());
                Ok(())
            })
            .unwrap()
            .register_callback("chats/MESSAGE_SENT", move |msg, _, _| {
                let notify = states::notify_client().unwrap();
                notify
                    .gui()
                    .send_event(UIEvent::ChatMessageSent, msg.payload.clone());
                Ok(())
            })
            .unwrap()
            .register_callback("MESSAGE/ONLINE_COUNT", move |_, _, _| Ok(()))
            .unwrap()
            .register_callback("internal/disconnected", move |_, _, _| Ok(()))
            .unwrap()
            .build()
            .await
            .unwrap();
        self.ws_client = Some(client);
        self.client.update_auth_module(self.clone());
        Ok(())
    }
    pub fn set_user_status(&self, status: String) -> Result<(), AppError> {
        if let Some(ws_client) = &self.ws_client {
            match ws_client.send_request("@WS/USER/SET_STATUS", json!(status)) {
                Ok(_) => {}
                Err(e) => panic!("{:?}", e),
            }

            // match ws_client.send_request(
            //     "@wfm|cmd/status/set",
            //     json!({
            //         "status": status
            //     }),
            // ) {
            //     Ok(_) => {}
            //     Err(e) => panic!("{:?}", e),
            // }
        } else {
            println!("WS client is not initialized, cannot set user status");
        }
        Ok(())
    }
    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(UserProfile, Option<String>), AppError> {
        let body = json!({
            "email": email,
            "password": password
        });

        let (user, headers): (UserProfile, HeaderMap) = match self
            .client
            .post::<UserProfile>("/auth/signin", Some("user"), body)
            .await
        {
            Ok(ApiResult::Success(user, headers)) => {
                logger::info(
                    &self.get_component("Login"),
                    &format!(
                        "User logged in: {}",
                        user.ingame_name.clone().unwrap_or("".to_string())
                    ),
                    LoggerOptions::default(),
                );
                (user, headers)
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Login"),
                    e,
                    eyre!("There was an error logging in"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        };

        // Get the "set-cookie" header
        let cookies = headers.get("set-cookie");
        // Check if the header is present
        let token = if let Some(cookie_value) = cookies {
            // Convert HeaderValue to String
            let cookie_str = cookie_value.to_str().unwrap_or_default();

            // The slicing and splitting logic
            let access_token: Option<String> =
                Some(cookie_str[4..].split(';').next().unwrap_or("").to_string());
            access_token
        } else {
            None
        };
        Ok((user, token))
    }

    pub async fn validate(&self) -> Result<UserProfile, AppError> {
        // Validate Auth
        let user = match self.me().await {
            Ok(user) => user,
            Err(e) => {
                error::create_log_file("command.log", &e);
                return Err(e);
            }
        };
        if user.anonymous || !user.verification {
            logger::warning(
                &self.get_component("Validate"),
                "Validation failed for user, user is anonymous or not verified",
                LoggerOptions::default(),
            );
        } else {
            logger::info(
                &self.get_component("Validate"),
                "User validated successfully",
                LoggerOptions::default(),
            );
        }
        return Ok(user);
    }
}
