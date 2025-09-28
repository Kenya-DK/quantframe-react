use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManualUpdate {
    #[serde(rename = "has_update")]
    pub has_update: bool,

    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "download")]
    pub download: String,

    #[serde(rename = "version")]
    pub version: String,
}
