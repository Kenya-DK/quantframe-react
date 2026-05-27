use entity::dto::SubType;
use serde::{Deserialize, Serialize};

use crate::cache::{modules::LanguageModule, CacheItemBase};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheWeaponBase {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icon", default)]
    pub icon: String,
    #[serde(rename = "wfmUrl", default)]
    pub wfm_url: String,
    #[serde(rename = "wfmRivenUrl", default)]
    pub wfm_riven_url: String,
    #[serde(rename = "subType")]
    pub sub_type: Option<SubType>,
    #[serde(rename = "wfmRivenId", default)]
    pub wfm_riven_id: String,
    #[serde(rename = "wfmId", default)]
    pub wfm_id: String,
    #[serde(rename = "rivenType", default)]
    pub riven_type: String,
    #[serde(default, rename = "category")]
    pub category: String,
    #[serde(default, rename = "source")]
    pub source: String,
    #[serde(default, rename = "family")]
    pub family: String,
    #[serde(default, rename = "upgradeType")]
    pub upgrade_type: String,
    #[serde(rename = "disposition", default)]
    pub disposition: f64,
    #[serde(rename = "dispositionRank", default)]
    pub disposition_rank: i64,
    #[serde(rename = "isVariant", default)]
    pub is_variant: bool,
}
impl CacheWeaponBase {
    pub fn translate(&mut self, language: &LanguageModule) {
        if let Ok(translation) = language.get_by(&self.unique_name) {
            if !translation.name.is_empty() {
                self.name = translation.name.clone();
            }
        }
    }
}
impl Default for CacheWeaponBase {
    fn default() -> Self {
        Self {
            unique_name: String::new(),
            name: String::new(),
            icon: String::new(),
            wfm_url: String::new(),
            wfm_id: String::new(),
            riven_type: String::new(),
            wfm_riven_url: String::new(),
            wfm_riven_id: String::new(),
            category: String::new(),
            source: String::new(),
            family: String::new(),
            sub_type: None,
            disposition: 0.0,
            disposition_rank: 0,
            is_variant: false,
            upgrade_type: String::new(),
        }
    }
}
impl From<CacheWeaponBase> for CacheItemBase {
    fn from(item: CacheWeaponBase) -> Self {
        Self {
            unique_name: item.unique_name,
            name: item.name,
            category: item.category,
            source: item.source,
            previous_names: vec![],
            is_tradeable: !item.wfm_url.is_empty(), // Assume non-tradeable if no WFM URL
            wfm_url: Some(item.wfm_url),
            sub_type: item.sub_type,
            tags: vec![],
            quantity: 1,
        }
    }
}
