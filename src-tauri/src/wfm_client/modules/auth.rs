use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    auth::AuthState,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger,
        },
    },
    wfm_client::{client::WFMClient, types::user_profile::UserProfile},
};
#[derive(Clone, Debug)]
pub struct AuthModule {
    pub client: WFMClient,
    pub debug_id: String,
    component: String,
}

impl AuthModule {
    pub fn new(client: WFMClient) -> Self {
        AuthModule {
            client,
            debug_id: "wfm_client_auth".to_string(),
            component: "Auth".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }

    pub fn is_logged_in(&self) -> Result<(), AppError> {
        let auth = self.client.auth.lock()?;
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
                logger::info_con(
                    &self.get_component("Login"),
                    &format!(
                        "User logged in: {}",
                        user.ingame_name.clone().unwrap_or("".to_string())
                    ),
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
                error::create_log_file("command.log".to_string(), &e);
                return Err(e);
            }
        };
        if user.anonymous || !user.verification {
            logger::warning_con(
                &self.get_component("Validate"),
                "Validation failed for user, user is anonymous or not verified",
            );
        } else {
            logger::info_con(
                &self.get_component("Validate"),
                "User validated successfully",
            );
        }
        return Ok(user);
    }
}
