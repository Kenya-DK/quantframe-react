decrypt_keys

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecryptKeys {
    #[serde(rename = "key")]
    pub key: Vec<i64>,

    #[serde(rename = "iv")]
    pub iv: Vec<i64>,
}
