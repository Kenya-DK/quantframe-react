use std::sync::Arc;

use qf_api::errors::ApiError as QFApiError;
use qf_api::types::UserPrivate as QFUserPrivate;
use qf_api::Client as QFClient;
use sha256::digest;
use tauri::AppHandle;
use utils::{get_location, info, log_json, Error, LogLevel, LoggerOptions};
use wf_market::client::Authenticated as WFAuthenticated;
use wf_market::types::websocket::WsClient;
use wf_market::types::UserPrivate as WFUserPrivate;
use wf_market::Client as WFClient;

use crate::app::{Settings, User};
use crate::http_server::HttpServer;
use crate::utils::ErrorFromExt;
use crate::{emit_startup, emit_update_user, helper, SENSITIVE_FIELDS};

use super::ws::{self, setup_socket};

#[derive(Clone)]
pub struct AppState {
    pub user: User,
    pub settings: Settings,
    pub wfm_client: WFClient<WFAuthenticated>,
    pub qf_client: QFClient,
    pub is_development: bool,
    pub is_pre_release: bool,
    pub use_temp_db: bool,
    pub wfm_socket: Option<WsClient>,
    pub wfm_chat_socket: Option<WsClient>,
    pub http_server: Arc<HttpServer>,
}

impl AppState {
    pub async fn new(tauri_app: AppHandle, use_temp_db: bool, is_pre_release: bool) -> Self {
        let user = User::load().expect("Failed to load user from auth.json");
        let info = tauri_app.package_info().clone();
        let is_development = if cfg!(dev) { true } else { false };

        let settings = Settings::load().expect("Failed to load settings from settings.json");
        let http_settings = settings.advanced_settings.http_server.clone();
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
                is_pre_release,
            ),
            user,
            is_development,
            use_temp_db,
            is_pre_release,
            settings,
            wfm_socket: None,
            wfm_chat_socket: None,
            http_server: HttpServer::new(&http_settings.host, http_settings.port),
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
                state.user = ws::update_user(state.user, &wfu, &qfu);
            }
            Err(e) => {
                e.log("user_validation.log");
                if e.log_level != LogLevel::Warning {
                    state.user = User::default();
                }
            }
        }
        state.user.save().expect("Failed to save user to auth.json");
        if http_settings.enable {
            state.http_server.start();
        }
        state
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<
        (
            QFClient,
            WFClient<WFAuthenticated>,
            User,
            WsClient,
            WsClient,
        ),
        Error,
    > {
        let mut wfm_client = match self
            .new_base_wfm_client()
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

        let qf_user = self.authenticate_qf_user(&qf_client, &wfm_user).await?;
        user.qf_token = qf_user.token.clone().unwrap();
        qf_client.set_token(&user.qf_token);
        let updated_user = ws::update_user(user, &wfm_user, &qf_user);
        let (ws, ws_chat) = setup_socket(wfm_client.clone()).await?;
        updated_user.save()?;
        Ok((qf_client, wfm_client, updated_user, ws, ws_chat))
    }

    fn new_base_wfm_client(&self) -> WFClient {
        let wfm_client = WFClient::new()
            .with_callback("api:after", |_, data| {
                info(
                    "WarframeMarket:API",
                    &format!(
                        "Method: {} | Route: {} | Took {}ms",
                        data.get_property_value("method", String::new()),
                        data.get_property_value("url", String::new()),
                        data.get_property_value("duration_ms", 0)
                    ),
                    &LoggerOptions::default(),
                );
            })
            .with_callback("api:refresh", |_, data| {
                let state = data.get_property_value("state", String::from("unknown"));
                emit_startup!(format!("wfm.{}", state), json!({}));
            })
            .with_callback("api:error", |_, data| {
                let mut data = data.clone();
                data.mask_sensitive_data(SENSITIVE_FIELDS);
                let timestamp = chrono::Local::now()
                    .with_timezone(&chrono::Utc)
                    .format("%Y_%m_%d_%H_%M_%S")
                    .to_string();

                if let Some(data) = data.properties.clone() {
                    log_json(data, &format!("wfm_api_error_{}.json", timestamp)).ok();
                }
            });
        wfm_client
    }

    pub async fn validate(&mut self) -> Result<(WFUserPrivate, QFUserPrivate), Error> {
        if self.user.wfm_token == "" || self.user.qf_token == "" {
            return Err(Error::new(
                "AppState:Validate",
                "User tokens are empty, please login first.",
                get_location!(),
            ));
        }
        let wfm_client = match self
            .new_base_wfm_client()
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
            Err(QFApiError::Unauthorized(err)) if err.error.message.contains("Unauthorized") => {
                self.authenticate_qf_user(&self.qf_client, &wfm_user)
                    .await?
            }
            Err(e) => {
                let level = match e {
                    QFApiError::RequestError(_) => LogLevel::Warning,
                    _ => LogLevel::Critical,
                };
                return Err(Error::from_qf(
                    "AppState:Validate",
                    "Failed to get QF user",
                    e,
                    get_location!(),
                )
                .set_log_level(level));
            }
        };
        if !qf_user.token.is_none() {
            self.qf_client.set_token(qf_user.token.as_ref().unwrap());
        }
        let (ws, ws_chat) = setup_socket(wfm_client.clone()).await?;
        self.wfm_socket = Some(ws);
        self.wfm_chat_socket = Some(ws_chat);
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
            Err(QFApiError::InvalidCredentials(err))
                if err.error.message.contains("invalid_username") =>
            {
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
