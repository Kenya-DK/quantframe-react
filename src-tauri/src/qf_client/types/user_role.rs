use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRole {
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