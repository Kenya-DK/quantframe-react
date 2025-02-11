use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

use crate::{
    qf_client::{client::QFClient, types::user::User},
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger, states,
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
        let app = states::app_state()?;

        match self
            .client
            .get::<User>(&format!("auth/me?v={}", app.get_app_info().version), false)
            .await
        {
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
    pub async fn login(&self, username: &str, password: &str) -> Result<User, AppError> {
        let app = states::app_state()?;
        let body = json!({
            "username": username,
            "password": password,
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
    pub async fn logout(&self) -> Result<(), AppError> {
        match self.client.post::<Value>("auth/logout", json!({})).await {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Logout"),
                    e,
                    eyre!("There was an error logging out"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
    pub async fn login_or_register(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, AppError> {
        // Try to login first
        match self.login(username, password).await {
            Ok(user) => {
                self.client.analytics().set_send_metrics(true);
                return Ok(user);
            }
            Err(e) => {
                if e.log_level() == LogLevel::Critical {
                    error::create_log_file("auth_login.log".to_string(), &e);
                    return Err(e);
                }
            }
        };
        // Try to register if login fails
        match self.register(username, password).await {
            Ok(user) => {
                return Ok(user);
            }
            Err(e) => {
                if e.log_level() == LogLevel::Critical {
                    error::create_log_file("auth_register.log".to_string(), &e);
                }
                return Err(e);
            }
        };
    }
    pub async fn register(&self, username: &str, password: &str) -> Result<User, AppError> {
        let body = json!({
            "username": username,
            "password": password,
        });

        let (user, _): (User, HeaderMap) = match self.client.post::<User>("users", body).await {
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
        let mut auth = states::auth()?;
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
        } else {
            logger::warning_con(&self.get_component("Validate"), "User is not logged in");
            auth.reset();
        }
        return Ok(user);
    }
}
