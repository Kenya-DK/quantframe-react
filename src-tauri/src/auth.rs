use crate::utils::modules::error::{self, AppError};
use crate::wfm_client::types::user_profile::UserProfile;
use crate::{helper, logger};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use sha256::digest;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub anonymous: bool,
    pub verification: bool,
    pub wfm_banned: bool,
    pub qf_banned: bool,
    pub id: String,
    pub wfm_access_token: Option<String>,
    pub qf_access_token: Option<String>,
    pub avatar: Option<String>,
    pub ingame_name: String,
    pub check_code: String,
    pub locale: String,
    pub platform: String,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<crate::qf_client::types::user_role::UserRole>,
    #[serde(default = "AuthState::order_limit")]
    pub order_limit: i64,
    #[serde(default = "AuthState::auctions_limit")]
    pub auctions_limit: i64,
    pub status: Option<String>,
}
// Allow us to run AuthState::default()
impl Default for AuthState {
    fn default() -> Self {
        Self {
            anonymous: true,
            verification: false,
            wfm_banned: false,
            qf_banned: false,
            id: "".to_string(),
            wfm_access_token: None,
            qf_access_token: None,
            avatar: Some("".to_string()),
            ingame_name: "".to_string(),
            locale: "".to_string(),
            platform: "".to_string(),
            region: "".to_string(),
            check_code: "".to_string(),
            role: None,
            order_limit: 100,
            auctions_limit: 50,
            status: Some("invisible".to_string()),
        }
    }
}
impl AuthState {
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
        digest(format!("hashStart-{}-hashEnd", self.id).as_bytes())
    }

    pub fn get_password(&self) -> String {
        digest(format!("hashStart-{}-hashEnd", self.check_code).as_bytes())
    }

    pub fn update_from_wfm_user_profile(
        &mut self,
        user_profile: &UserProfile,
        token: Option<String>,
    ) {
        self.id = user_profile.id.clone();
        self.anonymous = user_profile.anonymous;
        self.verification = user_profile.verification;
        self.wfm_banned = user_profile.banned;
        self.ingame_name = user_profile.ingame_name.clone().unwrap_or("".to_string());
        self.avatar = user_profile.avatar.clone();
        self.locale = user_profile.locale.clone();
        self.platform = user_profile.platform.clone();
        self.region = user_profile.region.clone();
        self.check_code = user_profile.check_code.clone().unwrap_or("".to_string());
        self.wfm_access_token = token;
        if user_profile.role != "user" {
            self.order_limit = 999;
            self.auctions_limit = 999;
        } else {
            self.order_limit = 100;
            self.auctions_limit = 50;
        }
    }

    pub fn reset(&mut self) {
        self.anonymous = true;
        self.verification = false;
        self.wfm_banned = false;
        self.qf_banned = false;
        self.id = "".to_string();
        self.wfm_access_token = None;
        self.qf_access_token = None;
        self.avatar = Some("".to_string());
        self.ingame_name = "".to_string();
        self.locale = "".to_string();
        self.platform = "".to_string();
        self.region = "".to_string();
        self.check_code = "".to_string();
        self.role = None;
        self.order_limit = 100;
        self.auctions_limit = 50;
        self.status = Some("invisible".to_string());
    }

    pub fn update_from_qf_user_profile(
        &mut self,
        user_profile: &crate::qf_client::types::user::User,
        token: Option<String>,
    ) {
        self.qf_access_token = token;
        self.qf_banned = user_profile.banned;
        self.role = user_profile.role.clone();
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
                error::create_log_file("auth_state.log".to_string(), &e);
                Self::save_to_file(&AuthState::default())?;
            }
        }
        Ok((AuthState::default(), false))
    }

    fn validate_json(json_str: &str) -> Result<(Self, bool), AppError> {
        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(AuthState::default())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        // Validate the JSON object against the required properties
        let (validated_json, missing_properties) =
            helper::validate_json(&json_value, &required_json, "");

        // Check for missing properties
        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                logger::warning_con("AuthState", &format!("Missing property: {}", property));
            }
        }

        // Deserialize the updated JSON object into a AuthState struct
        let deserialized: AuthState = serde_json::from_value(validated_json)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;

        Ok((deserialized, missing_properties.is_empty()))
    }
}
