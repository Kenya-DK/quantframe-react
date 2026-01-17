use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheChatIcon {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "url")]
    pub url: String,
}
