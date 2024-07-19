use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRiven {
    #[serde(rename = "weapons")]
    pub weapons: Vec<CacheRivenWeapon>,
    #[serde(rename = "rivens_attributes")]
    pub rivens_attributes: Vec<CacheRivenAttribute>,
    #[serde(rename = "available_attributes")]
    pub available_attributes: Vec<CacheRivenWFMAttribute>,
}

impl CacheRiven {
    pub fn new() -> Self {
        CacheRiven {
            weapons: Vec::new(),
            rivens_attributes: Vec::new(),
            available_attributes: Vec::new(),
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWeapon {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_icon_format: Option<String>,
    #[serde(rename = "wfm_thumb")]
    pub wfm_thumb: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "disposition")]
    pub disposition: f64,
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: String,
    #[serde(rename = "godRoll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub god_roll: Option<CacheRivenGodRoll>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenGoodRoll {
    #[serde(rename = "required")]
    pub required: Vec<String>,
    #[serde(rename = "optional")]
    pub optional: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenGodRoll {
    #[serde(rename = "weapon_url_name")]
    pub weapon_url_name: String,
    #[serde(rename = "good_rolls")]
    pub good_rolls: Vec<CacheRivenGoodRoll>,
    #[serde(rename = "negative_attributes")]
    pub negative_attributes: Vec<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenAttribute {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "upgrades")]
    pub upgrades: Vec<CacheRivenUpgrade>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenUpgrade {
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    #[serde(rename = "modifierTag")]
    pub modifier_tag: String,
    #[serde(rename = "prefixTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suffixTag")]
    pub suffix_tag: Option<String>,
    #[serde(rename = "baseValue")]
    pub base_value: f64,
    #[serde(rename = "localizationString")]
    pub localization_string: String,
    #[serde(rename = "shortString")]
    pub short_string: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWFMAttribute {
    #[serde(rename = "units")]
    pub units: Option<String>,

    #[serde(rename = "negative_only")]
    pub negative_only: bool,

    #[serde(rename = "positive_only")]
    pub positive_only: bool,

    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
