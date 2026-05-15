use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenGoodRoll {
    #[serde(rename = "required")]
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "optional")]
    #[serde(default)]
    pub optional: Vec<String>,
}
