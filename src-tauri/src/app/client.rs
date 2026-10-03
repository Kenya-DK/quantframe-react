use qf_api::Client as QFClient;
use sha256::digest;
use tauri::AppHandle;
use utils::{info, log_json, Error, LogLevel, LoggerOptions};
use wf_market::Client as WFClient;

use crate::app::modules::analytics::Analytics;
use crate::app::modules::auth::update_user;
use crate::app::{AppState, Settings, User};
use crate::http_server::HttpServer;
use crate::{emit_startup, SENSITIVE_FIELDS};
use crate::{emit_update_user, helper};

impl AppState {
    pub async fn new(
        tauri_app: AppHandle,
        use_temp_db: bool,
        is_pre_release: bool,
    ) -> Result<Self, Error> {
        let user = User::load().unwrap_or_else(|e| {
            e.log("app_init.log");
            User::default()
        });
        let info = tauri_app.package_info().clone();
        let is_development = if cfg!(dev) { true } else { false };

        let settings = Settings::load().unwrap_or_else(|e| {
            e.log("app_init.log");
            Settings::default()
        });
        let http_settings = settings.advanced_settings.http_server.clone();
        let device_id = digest(format!("hashStart-{}-hashEnd", helper::get_device_id()).as_bytes());
        let platform = tauri_plugin_os::platform().to_string();
        let user_agent = format!(
            "Quantframe/{} ({}; +https://quantframe.app)",
            info.version.to_string(),
            platform
        );
        let qf_client = QFClient::new(
            &user.qf_token,
            "rqf6ahg*RFY3wkn4neq",
            &platform,
            &device_id,
            is_development,
            &info.name,
            &info.version.to_string(),
            "N/A",
            "N/A",
            "N/A",
            is_pre_release,
        )
        .with_user_agent(&user_agent);
        let analytics = Analytics::new(qf_client.clone());
        let wfm_client = Self::new_base_wfm_client()
            .with_user_agent(&user_agent)
            .authenticate(&user.wfm_token, &device_id, false)
            .await
            .expect("Failed to create WFM client");
        let mut state = AppState {
            wfm_client,
            qf_client,
            analytics,
            user,
            is_development,
            use_temp_db,
            is_pre_release,
            settings,
            wfm_socket: None,
            wfm_chat_socket: None,
            http_server: HttpServer::new(&http_settings.host, http_settings.port),
        };
        let analytics_on_ban = state.analytics.clone();
        state.qf_client.on("user_banned", move |_, data| {
            analytics_on_ban.stop();
            emit_update_user!(json!({
                "qf_banned": true,
                "qf_banned_reason": data["banned_reason"].as_str().unwrap_or("").to_string(),
                "qf_banned_until": data["banned_until"].as_str().unwrap_or("").to_string()
            }));
        });
        match state.validate().await {
            Ok((wfu, qfu)) => {
                state.user = update_user(state.user, &wfu, &qfu);
            }
            Err(e) => {
                e.log("user_validation.log");
                if e.log_level != LogLevel::Warning {
                    state.user = User::default();
                }
            }
        }
        // Only collect analytics for authenticated, non-banned users.
        if !state.user.anonymous && !state.user.is_banned() {
            state.analytics.start();
            state.analytics.track_event(
                qf_api::enums::app_events::ApplicationEvent::AppStart,
                [("version", info.version.to_string())],
            );
        } else {
            state.analytics.stop();
        }
        state.user.save().expect("Failed to save user to auth.json");
        if http_settings.enable {
            state.http_server.start();
        }
        Ok(state)
    }
    pub(crate) fn new_base_wfm_client() -> WFClient {
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
    pub fn update_settings(&mut self, settings: Settings) -> Result<(), Error> {
        self.settings = settings;
        self.settings.save()?;
        Ok(())
    }
}
