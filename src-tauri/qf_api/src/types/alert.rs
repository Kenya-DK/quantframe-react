use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "type")]
    pub alert_type: String,

    #[serde(rename = "context")]
    pub context: String,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "properties")]
    pub properties: Option<Value>,
}

impl Alert {
    pub fn get_uuid(&self) -> String {
        format!("{}:{}", self.id, self.updated_at)
    }
}
