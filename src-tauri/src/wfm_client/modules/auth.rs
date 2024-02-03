use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    auth::AuthState,
    error::{self, ApiResult, AppError},
    wfm_client::client::WFMClient,
};
pub struct AuthModule<'a> {
    pub client: &'a WFMClient,
    pub debug_id: String,
}

impl<'a> AuthModule<'a> {
    pub async fn login(&self, email: String, password: String) -> Result<AuthState, AppError> {
        let body = json!({
            "email": email,
            "password": password
        });

        let (mut user, headers): (AuthState, HeaderMap) = match self
            .client
            .post::<AuthState>("/auth/signin", Some("user"), body)
            .await
        {
            Ok(ApiResult::Success(user, headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "User:Login",
                    format!("User logged in: {}", user.ingame_name).as_str(),
                    None,
                );
                (user, headers)
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auth:Login",
                    e,
                    eyre!("There was an error logging in"),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        };

        // Get the "set-cookie" header
        let cookies = headers.get("set-cookie");
        // Check if the header is present
        if let Some(cookie_value) = cookies {
            // Convert HeaderValue to String
            let cookie_str = cookie_value.to_str().unwrap_or_default();

            // The slicing and splitting logic
            let access_token: Option<String> =
                Some(cookie_str[4..].split(';').next().unwrap_or("").to_string());
            user.access_token = access_token;
            user.avatar = user.avatar;
        } else {
            user.clone().access_token = None;
        }
        Ok(user)
    }

    pub async fn validate(&self) -> Result<bool, AppError> {
        let mut auth = self.client.auth.lock()?.clone();
        if auth.access_token.is_none() {
            return Ok(false);
        }

        match self
            .client
            .orders()
            .create("56783f24cbfa8f0432dd89a2", "buy", 1, 1, false, None)
            .await
        {
            Ok(order) => {
                self.client.orders().delete(&order.id.clone()).await?;
                Ok(true)
            }
            Err(e) => {
                if e.cause()
                    .contains("app.post_order.already_created_no_duplicates")
                {
                    return Ok(true);
                }
                auth.access_token = None;
                auth.id = "".to_string();
                auth.save_to_file()?;
                error::create_log_file("auth.log".to_string(), &e);
                Ok(false)
            }
        }
    }
}
