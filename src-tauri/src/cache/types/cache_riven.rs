use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRiven {
    #[serde(rename = "weapons")]
    pub weapons: Vec<CacheRivenWeapon>,
    #[serde(rename = "upgrade_types_dict")]
    pub upgrade_types: HashMap<String, Vec<CacheRivenUpgrade>>,
    #[serde(rename = "attributes")]
    pub attributes: Vec<CacheRivenWFMAttribute>,
}

impl CacheRiven {
    pub fn new() -> Self {
        CacheRiven {
            weapons: Vec::new(),
            upgrade_types: HashMap::new(),
            attributes: Vec::new(),
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
    #[serde(rename = "is_variant")]
    pub is_variant: bool,
    #[serde(rename = "wfm_icon_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_icon_format: Option<String>,
    #[serde(rename = "wfm_thumb")]
    pub wfm_thumb: String,
    #[serde(rename = "unique_name")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "disposition")]
    pub disposition: f64,
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: String,
    #[serde(rename = "rolls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub god_roll: Option<CacheRivenRolls>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenGoodRoll {
    #[serde(rename = "required")]
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "optional")]
    #[serde(default)]
    pub optional: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenRolls {
    #[serde(rename = "weapon_url_name")]
    pub weapon_url_name: String,
    #[serde(rename = "good_rolls")]
    pub good_rolls: Vec<CacheRivenGoodRoll>,
    #[serde(rename = "negative_attributes")]
    pub negative_attributes: Vec<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenUpgrade {
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    #[serde(rename = "modifier_tag")]
    pub modifier_tag: String,
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "localization_string")]
    pub localization_string: String,
    #[serde(rename = "short_string")]
    pub short_string: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWFMAttribute {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "gameRef")]
    pub game_ref: String,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "prefix")]
    pub prefix: String,

    #[serde(rename = "suffix")]
    pub suffix: String,

    #[serde(rename = "effect")]
    pub effect: String,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exclusiveTo")]
    pub exclusive_to: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positiveIsNegative")]
    pub positive_is_negative: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positiveOnly")]
    pub positive_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "negativeOnly")]
    pub negative_only: Option<bool>,
}
