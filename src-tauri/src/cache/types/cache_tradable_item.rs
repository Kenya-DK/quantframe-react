use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheTradableItem {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "wfmId")]
    pub wfm_id: String,

    #[serde(rename = "wfmUrl")]
    pub wfm_url: String,

    #[serde(rename = "tradeTax")]
    pub trade_tax: i64,

    #[serde(rename = "masteryReq")]
    pub mr_requirement: i64,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "bulkTradable")]
    #[serde(default)]
    pub bulk_tradable: bool,

    #[serde(rename = "subTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "variantToUniqueName", default)]
    pub variant_to_unique_name: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxRank")]
    pub max_rank: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variants")]
    pub variants: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amberStars")]
    pub amber_stars: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cyanStars")]
    pub cyan_stars: Option<i64>,
}

impl Default for SubType {
    fn default() -> Self {
        SubType {
            max_rank: None,
            variants: None,
            amber_stars: None,
            cyan_stars: None,
        }
    }
}
