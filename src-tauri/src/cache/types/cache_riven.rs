use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRiven {
    #[serde(rename = "wfmWeapons")]
    pub wfm_weapons: Vec<CacheRivenWfmWeapon>,
    #[serde(rename = "wfmAttributes")]
    pub wfm_attributes: Vec<CacheRivenWfmAttribute>,
    #[serde(rename = "modifiersBasedOnTraitCount")]
    pub modifiers_based_on_trait_count: Vec<CacheRivenModifiersBasedOnTraitCount>,
    #[serde(rename = "dataByRivenInternalID")]
    pub riven_internal_id: HashMap<String, CacheRivenDataByRivenInternalID>,
    #[serde(rename = "weaponStats")]
    pub weapon_stat: HashMap<String, CacheWeaponStat>,
}

impl CacheRiven {
    pub fn new() -> Self {
        CacheRiven {
            wfm_weapons: Vec::new(),
            wfm_attributes: Vec::new(),
            modifiers_based_on_trait_count: Vec::new(),
            riven_internal_id: HashMap::new(),
            weapon_stat: HashMap::new(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheRivenModifiersBasedOnTraitCount {
    #[serde(rename = "goodModifiersCount")]
    pub good_modifiers_count: i64,

    #[serde(rename = "badModifiersCount")]
    pub bad_modifiers_count: i64,

    #[serde(rename = "goodModifierMultiplier")]
    pub good_modifier_multiplier: f64,

    #[serde(rename = "badModifierMultiplier")]
    pub bad_modifier_multiplier: f64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheRivenDataByRivenInternalID {
    #[serde(rename = "rivenInternalID")]
    pub riven_internal_id: String,

    #[serde(rename = "veiledName")]
    pub veiled_name: String,

    #[serde(rename = "baseDrain")]
    pub base_drain: i64,

    #[serde(rename = "fusionLimit")]
    pub fusion_limit: i64,

    #[serde(rename = "challenges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenges: Option<HashMap<String, RivenChallenges>>,

    #[serde(rename = "rivenStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riven_stats: Option<HashMap<String, RivenStat>>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RivenChallenges {
    #[serde(rename = "challengeUID")]
    pub challenge_uid: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "complications")]
    pub complications: HashMap<String, RivenComplication>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RivenComplication {
    #[serde(rename = "complicationID")]
    pub complication_id: String,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RivenStat {
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheWeaponStat {
    #[serde(rename = "omegaAtt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omega_att: Option<f64>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "rivenUID")]
    pub riven_uid: String,

    #[serde(rename = "good_rolls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_rolls: Option<GoodRolls>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoodRolls {
    #[serde(rename = "good_attrs")]
    good_attrs: Vec<GoodAttr>,
    #[serde(rename = "accepted_bad_attrs")]
    accepted_bad_attrs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GoodAttr {
    #[serde(rename = "mandatory")]
    mandatory: Vec<String>,

    #[serde(rename = "optional")]
    optional: Vec<String>,
}
