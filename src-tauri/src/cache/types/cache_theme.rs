use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CacheTheme {
    pub name: String,
    pub author: String,
    #[serde(rename = "iconBase64")]
    pub icon_base64: String,

    properties: serde_json::Value,
}
