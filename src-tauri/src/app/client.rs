use qf_api::Client as QFClient;
use serde_json::json;
use sha256::digest;
use tauri::AppHandle;
use utils::{get_location, Error, LogLevel};
use wf_market::Client as WFClient;

use crate::app::modules::auth::update_user;
use crate::app::{AppState, Settings, User};
use crate::http_server::HttpServer;
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
                state.user = update_user(state.user, &wfu, &qfu);
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
        Ok(state)
    }

    pub fn update_settings(&mut self, settings: Settings) -> Result<(), Error> {
        self.settings = settings;
        self.settings.save()?;
        Ok(())
    }
}
