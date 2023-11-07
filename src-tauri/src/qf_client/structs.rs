use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize)]
pub struct User {
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

#[derive(Serialize, Deserialize)]
pub struct Role {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "created_at")]
  created_at: String,

  #[serde(rename = "updated_at")]
  updated_at: String,

  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "can_managed")]
  can_managed: bool,

  #[serde(rename = "permissions")]
  permissions: String,
}
