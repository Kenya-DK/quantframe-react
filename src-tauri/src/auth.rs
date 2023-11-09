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
    // WarframeMarket specific fields
    pub wfm_access_token: Option<String>,
    pub qf_access_token: Option<String>,
    pub created_at: Option<i64>,
}
#[derive(Serialize, Deserialize)]
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
            wfm_access_token:None,
            locale_access_token:None,
            created_at: Some(chrono::Utc::now().timestamp()),
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
