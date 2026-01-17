use serde::{Deserialize, Serialize};

use super::user_role::UserRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "id")]
    pub id: String,
    
    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "banned")]    
    pub banned: bool,

    #[serde(rename = "current_version")]
    pub current_version: String,

    #[serde(rename = "banned_reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_reason: Option<String>,

    #[serde(rename = "banned_until")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_until: Option<String>,

    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: "".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
            username: "".to_string(),
            banned: false,
            banned_reason: None,
            banned_until: None,
            current_version: "".to_string(),
            role: Some(UserRole::default()),
            token: None,
        }
    }
}
