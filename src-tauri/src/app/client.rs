use std::sync::Mutex;

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
use utils::{get_location, warning, Error, LogLevel, LoggerOptions};
use wf_market::client::Authenticated as WFAuthenticated;
use wf_market::enums::ApiVersion;
use wf_market::types::websocket::WsClient;
use wf_market::types::UserPrivate as WFUserPrivate;
use wf_market::Client as WFClient;

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
        let wfm_user = wfm_client
            .get_user()
            .map_err(|e| Error::from_wfm("Login", "Failed to get WFM user", e, get_location!()))?;

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
        let wfm_user = wfm_client.get_user().unwrap();

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
