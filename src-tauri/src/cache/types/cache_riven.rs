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
    wfm_id: String,

    #[serde(rename = "wfm_url_name")]
    wfm_url_name: String,

    #[serde(rename = "wfm_group")]
    wfm_group: String,

    #[serde(rename = "riven_type")]
    riven_type: String,

    #[serde(rename = "wfm_icon")]
    wfm_icon: String,

    #[serde(rename = "wfm_icon_format")]
    wfm_icon_format: Option<String>,

    #[serde(rename = "wfm_thumb")]
    wfm_thumb: String,

    #[serde(rename = "i18n")]
    i18_n: HashMap<String, I18N>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct I18N {
    #[serde(rename = "name")]
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheRivenWfmAttribute {
    #[serde(rename = "units")]
    units: Option<String>,

    #[serde(rename = "negative_only")]
    negative_only: bool,

    #[serde(rename = "positive_only")]
    positive_only: bool,

    #[serde(rename = "suffix")]
    suffix: Option<String>,

    #[serde(rename = "url_name")]
    url_name: String,

    #[serde(rename = "prefix")]
    prefix: Option<String>,

    #[serde(rename = "effect")]
    effect: String,

    #[serde(rename = "exclusive_to")]
    exclusive_to:Option<Vec<String>>,

    #[serde(rename = "group")]
    group: String,

    #[serde(rename = "positive_is_negative")]
    positive_is_negative: bool,

    #[serde(rename = "search_only")]
    search_only: bool,

    #[serde(rename = "id")]
    id: String,
}
