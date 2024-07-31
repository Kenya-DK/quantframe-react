use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    qf_client::{client::QFClient, types::user::User},
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger,
        },
    },
};

#[derive(Clone, Debug)]
pub struct AuthModule {
    pub client: QFClient,
    component: String,
}

impl AuthModule {
    pub fn new(client: QFClient) -> Self {
        AuthModule {
            client,
            component: "Auth".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    pub async fn me(&self) -> Result<User, AppError> {
        let settings = self.client.settings.lock()?.clone();
        if settings.dev_mode {
            logger::warning_con(
                &self.get_component("Me"),
                "DevMode is enabled, returning default user",
            );
            return Ok(User::default());
        }

        match self.client.get::<User>("auth/profile").await {
            Ok(ApiResult::Success(user, _)) => {
                return Ok(user);
            }
            Ok(ApiResult::Error(e, _headers)) => {
                let log_level = if e.status_code < 200 || e.status_code > 299 {
                    LogLevel::Warning
                } else {
                    LogLevel::Critical
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Login"),
                    e,
                    eyre!("There was an error fetching user profile"),
                    log_level,
                ));
            }
            Err(e) => return Err(e),
        };
    }
    pub async fn login(
        &self,
        username: &str,
        password: &str,
        in_game_name: &str,
    ) -> Result<User, AppError> {
        let settings = self.client.settings.lock()?.clone();
        if settings.dev_mode {
            logger::warning_con(
                &self.get_component("Login"),
                "DevMode is enabled, returning default user",
            );
            return Ok(User::default());
        }
        let app = self.client.app.lock()?.clone();
        let body = json!({
            "username": username,
            "password": password,
            "ingame_name": in_game_name,
            "current_version": app.get_app_info().version.to_string(),
        });
        match self.client.post::<User>("auth/login", body).await {
            Ok(ApiResult::Success(user, _)) => {
                return Ok(user);
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
        }
    }
    pub async fn register(&self, username: &str, password: &str, in_game_name: &str,) -> Result<User, AppError> {
        let app = self.client.app.lock()?.clone();
        let body = json!({
            "username": username,
            "password": password,
            "password_confirmation": password,
            "ingame_name": in_game_name,
            "current_version": app.get_app_info().version.to_string(),
        });

        let (user, _): (User, HeaderMap) = match self
            .client
            .put::<User>("auth/registration", Some(body))
            .await
        {
            Ok(ApiResult::Success(user, headers)) => (user, headers),
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Register"),
                    e,
                    eyre!("There was an error registering"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        };
        return Ok(user);
    }
    pub async fn validate(&self) -> Result<Option<User>, AppError> {
        let settings = self.client.settings.lock()?.clone();
        if settings.dev_mode {
            logger::warning_con(
                &self.get_component("Validate"),
                "DevMode is enabled, returning default user",
            );
            return Ok(Some(User::default()));
        }
        let mut auth = self.client.auth.lock()?.clone();
        // Validate Auth
        let user = match self.me().await {
            Ok(user) => Some(user),
            Err(e) => {
                if e.log_level() == LogLevel::Critical {
                    error::create_log_file("qf_validate.log".to_string(), &e);
                    return Err(e);
                }
                None
            }
        };
        if user.is_some() {
            logger::info_con(&self.get_component("Validate"), "User is logged in");
            auth.update_from_qf_user_profile(&user.clone().unwrap(), auth.qf_access_token.clone());
        } else {
            logger::warning_con(&self.get_component("Validate"), "User is not logged in");
            auth.reset();
        }
        auth.save_to_file()?;
        return Ok(user);
    }
}
