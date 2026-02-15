use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecryptKeys {
    #[serde(rename = "key")]
    pub key: Vec<u8>,

    #[serde(rename = "iv")]
    pub iv: Vec<u8>,
}
