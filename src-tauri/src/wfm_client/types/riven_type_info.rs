use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RivenTypeInfo {
    #[serde(rename = "thumb")]
    pub thumb: String,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "mastery_level")]
    pub mastery_level: i64,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "item_name")]
    pub item_name: String,

    #[serde(rename = "icon_format")]
    pub icon_format: Option<String>,

    #[serde(rename = "riven_type")]
    pub riven_type: Option<String>,
}
