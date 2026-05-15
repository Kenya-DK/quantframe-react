use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemBase {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "category", default)]
    pub category: String,

    #[serde(rename = "source", default)]
    pub source: String,

    #[serde(rename = "wfm_url")]
    pub wfm_url: Option<String>,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,
}
