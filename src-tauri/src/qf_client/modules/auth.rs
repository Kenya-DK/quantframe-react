use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    auth::AuthState,
    error::AppError,
    qf_client::client::QFClient,
};
pub struct AuthModule<'a> {
    pub client: &'a QFClient,
}

impl<'a> AuthModule<'a> {
    pub async fn login(&self, email: String, password: String) -> Result<AuthState, AppError> {
        let body = json!({
            "email": email,
            "password": password
        });
        let (mut user, headers): (AuthState, HeaderMap) =
            self.client.post("/auth/signin", body).await?;

        // Get the "set-cookie" header
        let cookies = headers.get("set-cookie");
        // Check if the header is present
        if let Some(cookie_value) = cookies {
            // Convert HeaderValue to String
            let cookie_str = cookie_value.to_str().unwrap_or_default();

            // The slicing and splitting logic
            let access_token: Option<String> =
                Some(cookie_str[4..].split(';').next().unwrap_or("").to_string());
            user.wfm_access_token = access_token;
        } else {
            user.clone().wfm_access_token = None;
        }
        Ok(user)
    }

    pub async fn validate(&self) -> Result<Option<AuthState>, AppError> {
        let auth = self.client.auth.lock()?.clone();
        if auth.wfm_access_token.is_none() {
            return Ok(None);
        }
        match self.client.get("/auth/me").await {
            Ok((user, _)) => {
                Ok(Some(user))
            }
            Err(_e) => {
                Ok(None)
            }            
        }
    }

    pub async fn registration(
        &self,
        wfm_id: String,
        avatar: Option<String>,
        ingame_name: String,
        locale: String,
        platform: String,
        region: String,
        password: String,
        password_confirmation: String,
    ) -> Result<AuthState, AppError> {
        let body = json!({
            "wfm_id": wfm_id,
            "avatar": avatar,
            "ingame_name": ingame_name,
            "locale": locale,
            "platform": platform,
            "region": region,
            "password": password,
            "password_confirmation": password_confirmation
        });
        let (mut user, headers): (AuthState, HeaderMap) =
            self.client.post("/auth/registration", body).await?;

        // Get the "set-cookie" header
        let cookies = headers.get("set-cookie");
        // Check if the header is present
        if let Some(cookie_value) = cookies {
            // Convert HeaderValue to String
            let cookie_str = cookie_value.to_str().unwrap_or_default();

            // The slicing and splitting logic
            let access_token: Option<String> =
                Some(cookie_str[4..].split(';').next().unwrap_or("").to_string());
            user.wfm_access_token = access_token;
        } else {
            user.clone().wfm_access_token = None;
        }
        Ok(user)
    }
}
