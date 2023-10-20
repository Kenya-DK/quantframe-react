use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{auth::AuthState, error::AppError, logger, wfm_client::client::WFMClient};
pub struct AuthModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> AuthModule<'a> {
    pub async fn login(&self, email: String, password: String) -> Result<AuthState, AppError> {
        let body = json!({
            "email": email,
            "password": password
        });
        let (mut user, headers): (AuthState, HeaderMap) =
            self.client.post("/auth/signin", Some("user"), body).await?;

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
            .create(
                "Lex Prime Set",
                "56783f24cbfa8f0432dd89a2",
                "buy",
                1,
                1,
                false,
                None,
            )
            .await
        {
            Ok(order) => {
                self.client
                    .orders()
                    .delete(
                        &order.id.clone(),
                        "Lex Prime Set",
                        "56783f24cbfa8f0432dd89a2",
                        "buy",
                    )
                    .await?;
                Ok(true)
            }
            Err(_e) => {
                auth.access_token = None;
                auth.id = "".to_string();
                auth.save_to_file()?;
                logger::info(
                    "WarframeMarket",
                    "Invalid API Key",
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(false)
            }
        }
    }
}
