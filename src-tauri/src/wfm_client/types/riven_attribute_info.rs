use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RivenAttributeInfo {
    #[serde(rename = "negative_only")]
    negative_only: bool,

    #[serde(rename = "effect")]
    effect: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "exclusive_to")]
    exclusive_to: Option<Vec<String>>,

    #[serde(rename = "group")]
    group: String,

    #[serde(rename = "units")]
    units: Option<String>,

    #[serde(rename = "search_only")]
    search_only: bool,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "suffix")]
    suffix: Option<String>,

    #[serde(rename = "positive_is_negative")]
    positive_is_negative: bool,

    #[serde(rename = "prefix")]
    prefix: Option<String>,
}
