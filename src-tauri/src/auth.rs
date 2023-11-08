use crate::error::AppError;
use crate::helper;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "locale")]
    pub locale: String,

    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "active")]
    pub active: bool,

    #[serde(rename = "role")]
    pub role: Option<Role>,

    #[serde(rename = "role_id")]
    pub role_id: Option<String>,

    #[serde(rename = "token")]
    pub token: String,

    // WarframeMarket specific fields
    pub wfm_access_token: Option<String>,
    pub locale_access_token: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "can_managed")]
    pub can_managed: bool,

    #[serde(rename = "permissions")]
    pub permissions: String,
}
// Allow us to run AuthState::default()
impl Default for AuthState {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
            wfm_id: "".to_string(),
            avatar: "".to_string(),
            ingame_name: "".to_string(),
            locale: "".to_string(),
            platform: "".to_string(),
            region: "".to_string(),
            active: false,
            role: None,
            role_id: None,
            token: "".to_string(),
            wfm_access_token: None,
            locale_access_token: None,
        }
    }
}
impl AuthState {
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_roaming_path();
        let auth_path = app_path.join("auth.json");
        auth_path
    }
    pub fn setup() -> Result<Self, AppError> {
        let path_ref = Self::get_file_path();
        if path_ref.exists() {
            Self::read_from_file()
        } else {
            let default_auth = AuthState::default();
            default_auth.save_to_file()?;
            Ok(default_auth)
        }
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

    pub fn read_from_file() -> Result<Self, AppError> {
        let mut file = File::open(Self::get_file_path())
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        let auth = serde_json::from_str(&content)
            .map_err(|e| AppError::new("AuthState", eyre!(e.to_string())))?;
        Ok(auth)
    }
    pub fn send_to_window(&self) {
        helper::emit_update("user", "SET", Some(json!(self.clone())));
    }
}
