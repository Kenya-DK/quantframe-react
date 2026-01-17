use crate::utils::modules::error::{self, AppError};
use crate::utils::modules::logger::LoggerOptions;
use crate::wfm_client::modules::auth::SigninResponse;
use crate::wfm_client::types::user_profile::UserPrivate;
use crate::{helper, logger};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

use serde_json::Value;
use sha256::digest;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub verification: bool,
    pub wfm_banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_banned_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_banned_reason: Option<String>,
    pub qf_banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qf_banned_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qf_banned_reason: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qf_access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub ingame_name: String,
    pub check_code: String,
    pub locale: String,
    pub platform: String,
    pub region: String,
    #[serde(default = "AuthState::order_limit")]
    pub order_limit: i64,
    #[serde(default = "AuthState::auctions_limit")]
    pub auctions_limit: i64,
    pub unread_messages: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patreon_tier: Option<String>,
}
// Allow us to run AuthState::default()
impl Default for AuthState {
    fn default() -> Self {
        Self {
            verification: false,
            wfm_banned: false,
            wfm_banned_until: None,
            wfm_banned_reason: None,
            qf_banned: false,
            qf_banned_until: None,
            qf_banned_reason: None,
            id: "".to_string(),
            wfm_access_token: None,
            qf_access_token: None,
            avatar: Some("".to_string()),
            ingame_name: "".to_string(),
            locale: "".to_string(),
            platform: "".to_string(),
            region: "".to_string(),
            check_code: "".to_string(),
            order_limit: 100,
            unread_messages: 0,
            auctions_limit: 50,
            patreon_tier: None,
            permissions: None,
        }
    }
}
impl AuthState {
    pub fn is_logged_in(&self) -> bool {
        self.verification && !self.wfm_banned && !self.qf_banned
    }
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_storage_path();
        let auth_path = app_path.join("auth.json");
        auth_path
    }
    pub fn order_limit() -> i64 {
        let auth = AuthState::default();
        auth.order_limit
    }
    pub fn auctions_limit() -> i64 {
        let auth = AuthState::default();
        auth.auctions_limit
    }
    pub fn setup() -> Result<Self, AppError> {
        let path_ref = Self::get_file_path();
        if path_ref.exists() {
            let (se, valid) = Self::read_from_file()?;
            if valid {
                Ok(se)
            } else {
                se.save_to_file()?;
                Ok(se)
            }
        } else {
            let default_auth = AuthState::default();
            default_auth.save_to_file()?;
            Ok(default_auth)
        }
    }

    pub fn get_username(&self) -> String {
        self.id.clone()
        // digest(format!("hashStart-{}-hashEnd", self.id).as_bytes())
    }

    pub fn get_password(&self) -> String {
        // TODO: Enable This
        self.check_code.clone()
        // digest(format!("hashStart-{}-hashEnd", self.check_code).as_bytes())
    }

    pub fn get_user_hash(&self) -> String {
        digest(format!("hashStart-{}-{}-hashEnd", self.id, self.check_code).as_bytes())
    }

    pub fn get_device_id(&self) -> String {
        let device_id = helper::get_device_id();
        digest(format!("hashStart-{}-hashEnd", device_id).as_bytes())
    }

    pub fn update_from_wfm_user_profile(
        &mut self,
        user_profile: &SigninResponse,
        token: Option<String>,
    ) {
        self.id = user_profile.id.clone();
        self.verification = user_profile.verification;
        self.wfm_banned = user_profile.banned;
        self.unread_messages = user_profile.unread_messages as i64;
        self.ingame_name = user_profile.ingame_name.clone();
        self.avatar = user_profile.avatar.clone();
        self.locale = user_profile.locale.clone();
        self.platform = user_profile.platform.clone();
        self.region = user_profile.locale.clone();
        self.check_code = user_profile.check_code.clone();
        self.wfm_access_token = token;
        self.order_limit = 100;
        self.auctions_limit = 50;
    }
    pub fn update_from_wfm_user_profile2(
        &mut self,
        user_profile: &UserPrivate,
        token: Option<String>,
    ) {
        self.id = user_profile.id.clone();
        self.verification = user_profile.verification;
        self.wfm_banned = user_profile.banned.unwrap_or(false);
        self.unread_messages = user_profile.unread_messages as i64;
        self.wfm_banned_reason = user_profile.ban_message.clone();
        self.wfm_banned_until = user_profile.ban_until.clone();
        self.ingame_name = user_profile.ingame_name.clone();
        self.avatar = user_profile.avatar.clone();
        self.locale = user_profile.locale.clone();
        self.platform = user_profile.platform.clone();
        self.region = user_profile.locale.clone();
        self.check_code = user_profile.check_code.clone();
        self.wfm_access_token = token;
        self.order_limit = 100;
        self.auctions_limit = 50;
        if user_profile.tier.is_some() {
            self.order_limit = -1;
            self.auctions_limit = -1;
        }
    }
    pub fn reset(&mut self) {
        self.verification = false;
        self.wfm_banned = false;
        self.wfm_banned_until = None;
        self.wfm_banned_reason = None;
        self.qf_banned = false;
        self.qf_banned_until = None;
        self.qf_banned_reason = None;
        self.id = "".to_string();
        self.wfm_access_token = None;
        self.qf_access_token = None;
        self.avatar = Some("".to_string());
        self.ingame_name = "".to_string();
        self.locale = "".to_string();
        self.platform = "".to_string();
        self.region = "".to_string();
        self.check_code = "".to_string();
        self.order_limit = 100;
        self.auctions_limit = 50;
        self.permissions = None;
        self.patreon_tier = None;
    }

    pub fn update_from_qf_user_profile(
        &mut self,
        user_profile: &crate::qf_client::types::user::User,
        token: Option<String>,
    ) {
        self.qf_access_token = token;
        self.qf_banned = user_profile.banned;
        self.qf_banned_reason = user_profile.banned_reason.clone();
        self.qf_banned_until = user_profile.banned_until.clone();
        self.permissions = user_profile.permissions.clone();
        self.patreon_tier = user_profile.patreon_tier.clone();
    }

    pub fn save_to_file(&self) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        let mut file = File::create(Self::get_file_path())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        file.write_all(json.as_bytes())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        Ok(())
    }

    pub fn ban_user_wfm(&mut self, reason: &str) {
        self.wfm_banned = true;
        self.wfm_banned_reason = Some(reason.to_string());
        self.save_to_file().unwrap();
    }

    pub fn ban_user_qf(&mut self, reason: &str, until: &str) {
        self.qf_banned = true;
        self.qf_banned_reason = Some(reason.to_string());
        self.qf_banned_until = Some(until.to_string());
        self.save_to_file().unwrap();
    }
    pub fn read_from_file() -> Result<(Self, bool), AppError> {
        let mut file = File::open(Self::get_file_path())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        match Self::validate_json(&content) {
            Ok((auth, valid)) => {
                return Ok((auth, valid));
            }
            Err(e) => {
                error::create_log_file("auth_state.log", &e);
                Self::save_to_file(&AuthState::default())?;
            }
        }
        Ok((AuthState::default(), false))
    }

    fn validate_json(json_str: &str) -> Result<(Self, bool), AppError> {
        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        // If role is string, convert it to Null
        let json_value = match json_value.get("role") {
            Some(role) => {
                if role.is_string() {
                    let mut json_value = json_value.clone();
                    json_value
                        .as_object_mut()
                        .unwrap()
                        .insert("role".to_string(), Value::Null);
                    json_value
                } else {
                    json_value.clone()
                }
            }
            None => json_value.clone(),
        };

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(AuthState::default())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        // Validate the JSON object against the required properties
        let (validated_json, missing_properties) =
            helper::validate_json(&json_value, &required_json, "");

        // Check for missing properties
        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                logger::warning(
                    "AuthState",
                    &format!("Missing property: {}", property),
                    LoggerOptions::default(),
                );
            }
        }

        // Deserialize the updated JSON object into a AuthState struct
        let deserialized: AuthState = serde_json::from_value(validated_json)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        Ok((deserialized, missing_properties.is_empty()))
    }
}
