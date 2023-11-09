use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "anonymous")]
  pub anonymous: bool,

  #[serde(rename = "verification")]
  pub verification: bool,

  #[serde(rename = "ingame_name")]
  pub ingame_name: String,

  #[serde(rename = "check_code")]
  pub check_code: String,

  #[serde(rename = "role")]
  pub role: String,

  #[serde(rename = "patreon_profile")]
  pub patreon_profile: PatreonProfile,

  #[serde(rename = "platform")]
  pub platform: String,

  #[serde(rename = "region")]
  pub region: String,

  #[serde(rename = "banned")]
  pub banned: bool,

  #[serde(rename = "ban_reason")]
  pub ban_reason: Option<String>,

  #[serde(rename = "avatar")]
  pub avatar: Option<String>,

  #[serde(rename = "background")]
  pub background: Option<String>,

  #[serde(rename = "linked_accounts")]
  pub linked_accounts: LinkedAccounts,

  #[serde(rename = "has_email")]
  pub has_email: bool,

  #[serde(rename = "written_reviews")]
  pub written_reviews: i64,

  #[serde(rename = "unread_messages")]
  pub unread_messages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedAccounts {
  #[serde(rename = "steam_profile")]
  pub steam_profile: bool,

  #[serde(rename = "patreon_profile")]
  pub patreon_profile: bool,

  #[serde(rename = "xbox_profile")]
  pub xbox_profile: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PatreonProfile {
  #[serde(rename = "patreon_founder")]
  pub patreon_founder: bool,

  #[serde(rename = "subscription")]
  pub subscription: bool,

  #[serde(rename = "patreon_badge")]
  pub patreon_badge: String,
}