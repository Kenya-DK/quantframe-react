use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRiven {
    #[serde(rename = "wfmWeapons")]
    pub wfm_weapons: Vec<CacheRivenWfmWeapon>,
    #[serde(rename = "wfmAttributes")]
    pub wfm_attributes: Vec<CacheRivenWfmAttribute>,
}

impl CacheRiven {
    pub fn new() -> Self {
        CacheRiven {
            wfm_weapons: Vec::new(),
            wfm_attributes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheRivenWfmWeapon {
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url_name")]
    pub wfm_url_name: String,

    #[serde(rename = "wfm_group")]
    pub wfm_group: String,

    #[serde(rename = "riven_type")]
    pub riven_type: String,

    #[serde(rename = "wfm_icon")]
    pub wfm_icon: String,

    #[serde(rename = "wfm_icon_format")]
    pub wfm_icon_format: Option<String>,

    #[serde(rename = "wfm_thumb")]
    pub wfm_thumb: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "i18n")]
    pub i18_n: HashMap<String, I18N>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct I18N {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheRivenWfmAttribute {
    #[serde(rename = "units")]
    pub units: Option<String>,

    #[serde(rename = "negative_only")]
    pub negative_only: bool,

    #[serde(rename = "positive_only")]
    pub positive_only: bool,

    #[serde(rename = "suffix")]
    pub suffix: Option<String>,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "prefix")]
    pub prefix: Option<String>,

    #[serde(rename = "effect")]
    pub effect: String,

    #[serde(rename = "exclusive_to")]
    pub exclusive_to: Option<Vec<String>>,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "positive_is_negative")]
    pub positive_is_negative: bool,

    #[serde(rename = "search_only")]
    pub search_only: bool,

    #[serde(rename = "id")]
    pub id: String,
}
