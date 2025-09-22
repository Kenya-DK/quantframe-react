use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use crate::app::{Settings, User};
use crate::types::*;
use crate::utils::modules::states;
use crate::utils::{AuctionListExt, ErrorFromExt, OrderListExt};
use crate::{emit_startup, emit_update_user, helper, send_event, APP, HAS_STARTED};
use qf_api::errors::ApiError as QFApiError;
use qf_api::types::UserPrivate as QFUserPrivate;
use qf_api::Client as QFClient;
use serde_json::{json, Value};
use sha256::digest;
use tauri::{AppHandle, Manager};
use utils::{get_location, info, warning, Error, LogLevel, LoggerOptions};
use wf_market::client::Authenticated as WFAuthenticated;
use wf_market::enums::ApiVersion;
use wf_market::types::websocket::{WsClient, WsMessage};
use wf_market::types::{Chat, ChatMessage, UserPrivate as WFUserPrivate};
use wf_market::Client as WFClient;
pub static ACTIVE_CHAT_ID: OnceLock<Mutex<Option<String>>> = OnceLock::new();
pub fn set_active_chat_id(chat_id: Option<String>) {
    let active_chat_id = ACTIVE_CHAT_ID.get_or_init(|| Mutex::new(None));
    let mut guard = active_chat_id.lock().unwrap();
    *guard = chat_id;
}
pub fn get_active_chat_id() -> Option<String> {
    let active_chat_id = ACTIVE_CHAT_ID.get_or_init(|| Mutex::new(None));
    let guard = active_chat_id.lock().unwrap();
    guard.clone()
}
#[derive(Clone)]
pub struct AppState {
    pub user: User,
    pub settings: Settings,
    pub wfm_client: WFClient<WFAuthenticated>,
    pub qf_client: QFClient,
    pub is_development: bool,
    pub wfm_socket: Option<WsClient>,
}

fn update_user(mut cu_user: User, user: &WFUserPrivate, qf_user: &QFUserPrivate) -> User {
    cu_user.anonymous = false;
    cu_user.verification = user.verification;
    cu_user.wfm_banned = user.banned.unwrap_or(false);
    cu_user.wfm_banned_reason = user.ban_message.clone();
    cu_user.wfm_banned_until = user.ban_until.clone();
    cu_user.qf_banned = qf_user.banned;
    cu_user.qf_banned_reason = qf_user.banned_reason.clone();
    cu_user.qf_banned_until = qf_user.banned_until.clone();
    cu_user.wfm_id = user.id.to_string();
    cu_user.wfm_username = user.ingame_name.clone();
    cu_user.check_code = user.check_code.clone();
    cu_user.locale = user.locale.clone();
    cu_user.platform = user.platform.clone();
    cu_user.unread_messages = user.unread_messages as i64;
    cu_user.wfm_username = user.ingame_name.clone();
    cu_user.wfm_id = user.id.to_string();
    cu_user.wfm_avatar = user.avatar.clone();
    cu_user.unread_messages = user.unread_messages as i64;
    cu_user
}

fn send_ws_state(event: UIEvent, cause: &str, data: Value) -> Error {
    let err = Error::new("WebSocket", "Connection state", get_location!())
        .with_context(data)
        .with_cause(cause)
        .set_log_level(LogLevel::Warning);
    err.log("websocket_info.log");
    send_event!(event, Some(json!(err)));
    err
}

fn update_user_status(states: impl Into<String>) {
    let states = states.into();
    let app = APP.get().expect("APP not initialized");
    let state = app.state::<Mutex<AppState>>();
    let mut guard = state.lock().expect("Failed to lock notification state");
    guard.user.wfm_status = states;
    guard.user.save().expect("Failed to save user status");
    emit_update_user!(json!({"wfm_status": guard.user.wfm_status}));
}

fn handle_new_message(wfm_client: &WFClient<WFAuthenticated>, msg: &WsMessage) {
    let binding = wfm_client.chat();
    let active_chat_id = get_active_chat_id().unwrap_or_default();
    let chat_message = match msg
        .get_payload_as::<ChatMessage>(msg.route.ends_with("MESSAGE_SENT").then_some("message"))
    {
        Ok(chat_msg) => chat_msg,
        Err(e) => {
            let err = Error::from_json(
                "ChatMessage:Received",
                &PathBuf::from("websocket"),
                msg.payload.as_ref().unwrap_or(&json!({})).to_string(),
                "Failed to parse chat message from websocket",
                e,
                get_location!(),
            );
            err.log("websocket_info.log");
            return;
        }
    };
    if chat_message.is_none() {
        return;
    }
    let chat_message = chat_message.unwrap();

    fn handle_notify(
        chat: &Chat,
        chat_message: &ChatMessage,
        active_chat_id: impl Into<String>,
        requirer_refresh: bool,
        un_read: u32,
    ) {
        let active_chat_id = active_chat_id.into();
        let state = states::app_state().expect("Failed to get settings");

        let from_user = match chat.find_user(&chat_message.message_from) {
            Some(c) => c.name.clone(),
            None => "".to_string(),
        };
        let mut chat_payload = json!(chat_message);
        chat_payload["requirer_refresh"] = json!(requirer_refresh);
        send_event!(UIEvent::OnWfmChatMessage, chat_payload);

        if active_chat_id == chat.id || state.user.wfm_id == chat_message.message_from {
            return;
        }
        emit_update_user!(json!({ "unread_messages": un_read }));
        state
            .settings
            .notifications
            .on_wfm_chat_message
            .send(&HashMap::from([
                (
                    "<WFM_MESSAGE>".to_string(),
                    chat_message.raw_message.clone(),
                ),
                ("<CHAT_NAME>".to_string(), chat.chat_name.clone()),
                ("<FROM_USER>".to_string(), from_user.clone()),
            ]));
        info(
            "ChatMessage:Notify",
            &format!("New message in chat {} from {}", chat.chat_name, from_user),
            &LoggerOptions::default(),
        );
    }

    let chat = binding
        .cache_chats_mut()
        .handle_chat_message(&chat_message, &active_chat_id);
    if chat.is_none() {
        tauri::async_runtime::spawn(async move {
            match binding.get_chats().await {
                Ok(mut messages) => {
                    let chat = messages.get_by_id(&chat_message.chat_id, true);
                    if chat.is_some() {
                        handle_notify(
                            &chat.unwrap(),
                            &chat_message,
                            &active_chat_id,
                            true,
                            binding.cache_chats().total_unread_count(),
                        );
                    }
                }
                Err(e) => {
                    let err = Error::from_wfm(
                        "ChatMessage:FetchChat",
                        "Failed to fetch chats after receiving new message",
                        e,
                        get_location!(),
                    );
                    err.log("websocket_info.log");
                }
            }
        });
    } else {
        handle_notify(
            &chat.unwrap(),
            &chat_message,
            &active_chat_id,
            false,
            binding.cache_chats().total_unread_count(),
        );
    }
}

async fn setup_socket(wfm_client: WFClient<WFAuthenticated>) -> Result<WsClient, Error> {
    if wfm_client.get_user().is_err() {
        return Err(Error::new(
            "AppState:SetupSocket",
            "WFM client user is not authenticated, please login first.",
            get_location!(),
        ));
    }

    let ws_client = wfm_client
        .create_websocket(ApiVersion::V1)
        .set_log_unhandled(true)
        .register_callback("internal/connected", move |msg, _, _| {
            send_ws_state(UIEvent::OnError, "connected", json!(msg.payload));
            Ok(())
        })
        .unwrap()
        .register_callback("internal/disconnected", move |msg, _, _| {
            send_ws_state(UIEvent::OnError, "disconnected", json!(msg.payload));
            Ok(())
        })
        .unwrap()
        .register_callback("internal/reconnecting", move |msg, _, _| {
            send_ws_state(UIEvent::OnError, "reconnecting", json!(msg.payload));
            Ok(())
        })
        .unwrap()
        .register_callback("event/account/banned", move |msg, _, _| {
            let payload = msg.clone().payload.unwrap();
            emit_update_user!(json!({
                "wfm_banned": true,
                "wfm_banned_reason": payload["banMessage"].as_str().unwrap_or("").to_string(),
                "wfm_banned_until": payload["banUntil"].as_str().unwrap_or("").to_string()
            }));
            Ok(())
        })
        .unwrap()
        .register_callback("event/account/banLifted", move |_, _, _| {
            emit_update_user!(json!({"wfm_banned": false}));
            Ok(())
        })
        .unwrap()
        .register_callback("MESSAGE/ONLINE_COUNT", move |_, _, _| Ok(()))
        .unwrap()
        .register_callback("event/reports/online", move |_, _, _| Ok(()))
        .unwrap()
        .register_callback("USER/SET_STATUS", move |msg, _, _| {
            match msg.payload.as_ref() {
                Some(payload) => update_user_status(payload.as_str().unwrap_or("").to_string()),
                None => {}
            }
            emit_update_user!(json!({"wfm_status": msg.payload}));
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
            if !HAS_STARTED.get().cloned().unwrap_or(false) {
                return Ok(());
            }
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
        .register_callback("chats/NEW_MESSAGE,chats/MESSAGE_SENT", move |msg, _, _| {
            if !HAS_STARTED.get().cloned().unwrap_or(false) {
                return Ok(());
            }
            handle_new_message(&wfm_client, msg);
            Ok(())
        })
        .unwrap()
        .register_callback("ERROR", move |msg, _, _| {
            warning(
                "WebSocket:Error",
                &format!("WebSocket error: {:?}", msg),
                &LoggerOptions::default().set_file("websocket_info.log"),
            );
            Ok(())
        })
        .unwrap()
        .build()
        .await
        .unwrap();
    Ok(ws_client)
}

impl AppState {
    pub async fn new(tauri_app: AppHandle) -> Self {
        let user = User::load().expect("Failed to load user from auth.json");
        let info = tauri_app.package_info().clone();
        let is_development = if cfg!(dev) { true } else { false };
        let mut state = AppState {
            wfm_client: WFClient::new_default(&user.wfm_token, "N/A")
                .await
                .expect("Failed to create WFM client"),
            qf_client: QFClient::new(
                &user.qf_token,
                "rqf6ahg*RFY3wkn4neq",
                &tauri_plugin_os::platform().to_string(),
                &digest(format!("hashStart-{}-hashEnd", helper::get_device_id()).as_bytes()),
                is_development,
                &info.name,
                &info.version.to_string(),
                "N/A",
                "N/A",
                "N/A",
            ),
            user,
            is_development,
            settings: Settings::load().expect("Failed to load settings from settings.json"),
            wfm_socket: None,
        };

        state
            .qf_client
            .analytics()
            .add_metric("app_start", info.version.to_string());
        state.qf_client.on("user_banned", move |_, data| {
            emit_update_user!(json!({
                "qf_banned": true,
                "qf_banned_reason": data["banned_reason"].as_str().unwrap_or("").to_string(),
                "qf_banned_until": data["banned_until"].as_str().unwrap_or("").to_string()
            }));
        });
        match state.validate().await {
            Ok((wfu, qfu)) => {
                emit_startup!(
                    "validation.success",
                    json!({
                        "name": wfu.ingame_name,
                    })
                );
                state.user = update_user(state.user, &wfu, &qfu);
            }
            Err(e) => {
                e.log("user_validation.log");
                emit_startup!("validation.error", json!({}));
                state.user = User::default();
            }
        }

        state.user.save().expect("Failed to save user to auth.json");
        state
    }
}

impl AppState {
    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(QFClient, WFClient<WFAuthenticated>, User, WsClient), Error> {
        let cache = states::cache_client()?;
        // Login to WFM client
        let mut wfm_client = match WFClient::new()
            .login(email, password, &self.wfm_client.get_device_id())
            .await
        {
            Ok(client) => client,
            Err(e) => {
                return Err(Error::from_wfm(
                    "AppState:Login",
                    "Failed to login to WFM client",
                    e,
                    get_location!(),
                ))
            }
        };
        let mut wfm_user = wfm_client
            .get_user()
            .map_err(|e| Error::from_wfm("Login", "Failed to get WFM user", e, get_location!()))?;

        wfm_user.unread_messages = wfm_client.chat().cache_chats().total_unread_count() as i16;
        let mut user = self.user.clone();
        wfm_client.set_device_id(&self.qf_client.device);
        user.wfm_token = wfm_client.get_token();

        let mut qf_client = self.qf_client.clone();
        qf_client.set_wfm_id(&wfm_user.id);
        qf_client.set_wfm_username(&wfm_user.ingame_name);
        qf_client.set_wfm_platform(&wfm_user.platform);

        // Try to sign in to QF client, auto-register if user doesn't exist
        let qf_user = self.authenticate_qf_user(&qf_client, &wfm_user).await?;
        user.qf_token = qf_user.token.clone().unwrap();
        qf_client.set_token(&user.qf_token);
        let updated_user = update_user(user, &wfm_user, &qf_user);
        let ws = setup_socket(wfm_client.clone()).await?;
        updated_user.save()?;
        Ok((qf_client, wfm_client, updated_user, ws))
    }

    pub async fn validate(&mut self) -> Result<(WFUserPrivate, QFUserPrivate), Error> {
        if self.user.wfm_token == "" || self.user.qf_token == "" {
            return Err(Error::new(
                "AppState:Validate",
                "User tokens are empty, please login first.",
                get_location!(),
            ));
        }
        let wfm_client = match WFClient::new()
            .login_with_token(&self.user.wfm_token, &self.wfm_client.get_device_id())
            .await
        {
            Ok(client) => client,
            Err(e) => {
                return Err(Error::from_wfm(
                    "AppState:Validate",
                    "Failed to login with WFM token",
                    e,
                    get_location!(),
                ));
            }
        };
        let mut wfm_user = wfm_client.get_user().unwrap();
        wfm_user.unread_messages = wfm_client.chat().cache_chats().total_unread_count() as i16;
        self.qf_client.set_wfm_id(&wfm_user.id);
        self.qf_client.set_wfm_username(&wfm_user.ingame_name);
        self.qf_client.set_wfm_platform(&wfm_user.platform);
        let qf_user = match self.qf_client.authentication().me().await {
            Ok(u) => u,
            Err(QFApiError::Unauthorized(err)) if err.error.message == "Unauthorized" => {
                self.authenticate_qf_user(&self.qf_client, &wfm_user)
                    .await?
            }
            Err(e) => {
                return Err(Error::from_qf(
                    "AppState:Validate",
                    "Failed to get QF user",
                    e,
                    get_location!(),
                ))
            }
        };
        if !qf_user.token.is_none() {
            self.qf_client.set_token(qf_user.token.as_ref().unwrap());
        }
        let ws = setup_socket(wfm_client.clone()).await?;
        self.wfm_socket = Some(ws);
        if !qf_user.banned {
            match self.qf_client.analytics().start() {
                Ok(_) => {}
                Err(e) => {
                    return Err(Error::from_qf(
                        "AppState:Validate",
                        "Failed to start QF analytics",
                        e,
                        get_location!(),
                    ));
                }
            }
        }
        self.wfm_client = wfm_client;
        Ok((wfm_user, qf_user))
    }

    async fn authenticate_qf_user(
        &self,
        qf_client: &QFClient,
        wfm_user: &WFUserPrivate,
    ) -> Result<QFUserPrivate, Error> {
        match qf_client
            .authentication()
            .signin(&wfm_user.id, &wfm_user.check_code)
            .await
        {
            Ok(user) => Ok(user),
            Err(QFApiError::InvalidCredentials(err)) if err.error.message == "invalid_username" => {
                qf_client
                    .authentication()
                    .register(&wfm_user.id, &wfm_user.check_code)
                    .await
                    .map_err(|e| {
                        Error::from_qf(
                            "AppState:AuthenticateQFUser",
                            "Failed to register QF user",
                            e,
                            get_location!(),
                        )
                    })
            }
            Err(e) => Err(Error::from_qf(
                "AppState:AuthenticateQFUser",
                "Failed to authenticate QF user",
                e,
                get_location!(),
            )),
        }
    }

    pub fn update_settings(&mut self, settings: Settings) -> Result<(), Error> {
        self.settings = settings;
        self.settings.save()?;
        Ok(())
    }
}
