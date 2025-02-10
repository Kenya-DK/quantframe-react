use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
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
    pub permissions: String,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: "".to_string(),
            name: "".to_string(),
            permissions: "".to_string(),
            banned: false,
            banned_reason: None,
            banned_until: None,
            token: None,
        }
    }
}
