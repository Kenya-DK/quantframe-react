use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheTradableItem {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "unique_name")]
    pub unique_name: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url_name")]
    pub wfm_url_name: String,

    #[serde(rename = "trade_tax")]
    pub trade_tax: i64,

    #[serde(rename = "mr_requirement")]
    pub mr_requirement: i64,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "wiki_url")]
    pub wiki_url: String,

    #[serde(rename = "image_url")]
    pub image_url: String,

    #[serde(rename = "bulk_tradable")]
    #[serde(default)]
    pub bulk_tradable: bool,

    #[serde(rename = "max_rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rank: Option<i64>,

    #[serde(rename = "sub_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_rank")]
    pub max_rank: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variants")]
    pub variants: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amber_stars")]
    pub amber_stars: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cyan_stars")]
    pub cyan_stars: Option<i64>,
}
