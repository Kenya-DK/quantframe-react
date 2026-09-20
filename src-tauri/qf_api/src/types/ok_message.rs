use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OkMessageResponse {
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "i18n_key", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,

    #[serde(rename = "i18n_values", skip_serializing_if = "Option::is_none")]
    pub i18n_values: Option<Value>,
}