use std::f32::consts::E;

use eyre::eyre;
use serde_json::json;

use crate::{
    error::AppError,
    logger::LogLevel,
    qf_client::{
        client::{ApiResult, QFClient},
        structs::User,
    },
};
pub struct AuthModule<'a> {
    pub client: &'a QFClient,
}

impl<'a> AuthModule<'a> {
    pub async fn login(&self, username: String, password: String) -> Result<User, AppError> {
        let body = json!({
            "username": username,
            "password": password
        });

        let user: User = match self.client.post("auth/login", body).await {
            Ok(ApiResult::Success(payload, _headers)) => payload,
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "QuantframeApi",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };

        Ok(user)
    }

    pub async fn validate(&self) -> Result<Option<User>, AppError> {
        let mut auth = self.client.auth.lock()?.clone();
        if auth.qf_access_token.is_none() {
            return Ok(None);
        }

        let user: User = match self.client.get("auth/me").await {
            Ok(ApiResult::Success(payload, _headers)) => payload,
            Ok(ApiResult::Error(error, _headers)) => {
                auth.qf_access_token = None;
                auth.save_to_file()?;
                return Err(AppError::new_api(
                    "QuantframeApi",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                auth.qf_access_token = None;
                auth.save_to_file()?;
                return Err(err);
            }
        };
        // Ok(Some(user))
        Ok(Some(user))
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
        current_version: String,
    ) -> Result<User, AppError> {        
        let body = json!({
            "wfm_id": wfm_id,
            "avatar": avatar,
            "ingame_name": ingame_name,
            "locale": locale,
            "platform": platform,
            "region": region,
            "current_version": current_version,
            "password": password,
            "password_confirmation": password_confirmation
        });

        let user: User = match self.client.put("auth/registration", Some(body)).await {
            Ok(ApiResult::Success(payload, _headers)) => payload,
            Ok(ApiResult::Error(error, _headers)) => {
                let mut log_level = LogLevel::Error;
                if error.message.contains(&"user_already_exists".to_string()) {
                    log_level = LogLevel::Warning;
                }
                return Err(AppError::new_api(
                    "QuantframeApi",
                    error,
                    eyre!(""),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
        Ok(user)
    }
}
