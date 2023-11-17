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
pub struct UserModule<'a> {
    pub client: &'a QFClient,
}

impl<'a> UserModule<'a> {
    pub async fn update(
        &self,
        ingame_name: Option<String>,
        image: Option<String>,
        current_version: Option<String>,
        current_password: Option<String>,
        password: Option<String>,
        password_confirmation: Option<String>,
    ) -> Result<User, AppError> {
        let mut body = json!({});
        if ingame_name.is_some() {
            body["name"] = json!(ingame_name);
        }
        if image.is_some() {
            body["image"] = json!(image);
        }
        if current_version.is_some() {
            body["current_version"] = json!(current_version);
        }
        if current_password.is_some() {
            body["current_password"] = json!(current_password);
        }
        if password.is_some() {
            body["password"] = json!(password);
        }
        if password_confirmation.is_some() {
            body["password_confirmation"] = json!(password_confirmation);
        }

        let user: User = match self.client.post("users/profile", body).await {
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
}
