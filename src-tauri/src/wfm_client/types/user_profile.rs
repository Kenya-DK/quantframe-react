use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserProfile {
    #[serde(rename = "anonymous")]
    pub anonymous: bool,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "avatar")]
    pub avatar: Option<String>,
    #[serde(rename = "linked_accounts")]
    pub linked_accounts: LinkedAccounts,
    #[serde(rename = "verification")]
    pub verification: bool,
    #[serde(rename = "written_reviews")]
    pub written_reviews: i32,
    #[serde(rename = "background")]
    pub background: Option<String>,
    #[serde(rename = "ingame_name")]
    pub ingame_name: Option<String>,
    #[serde(rename = "check_code")]
    pub check_code: Option<String>,
    #[serde(rename = "unread_messages")]
    pub unread_messages: i32,
    #[serde(rename = "has_mail")]
    pub has_mail: bool,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "banned")]
    pub banned: bool,
    #[serde(rename = "reputation")]
    pub reputation: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LinkedAccounts {
    #[serde(rename = "steam_profile")]
    pub steam_profile: bool,
    #[serde(rename = "patreon_profile")]
    pub patreon_profile: bool,
    #[serde(rename = "xbox_profile")]
    pub xbox_profile: bool,
    #[serde(rename = "discord_profile")]
    pub discord_profile: bool,
    #[serde(rename = "github_profile")]
    pub github_profile: bool,
}
