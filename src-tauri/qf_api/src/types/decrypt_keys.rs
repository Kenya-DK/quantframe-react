use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecryptKeys {
    #[serde(rename = "key")]
    pub key: Vec<i64>,

    #[serde(rename = "iv")]
    pub iv: Vec<i64>,
}
