use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct UserPrivate {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "banned")]
    pub banned: bool,

    #[serde(rename = "banned_reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_reason: Option<String>,

    #[serde(rename = "banned_until")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_until: Option<String>,

    #[serde(rename = "permissions")]
    pub permissions: Option<String>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "patreon_tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patreon_tier: Option<String>,
}
