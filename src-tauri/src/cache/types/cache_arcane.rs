use serde::{Deserialize, Serialize};

use super::cache_item_component::CacheItemComponent;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArcane {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[serde(rename = "masterable")]
    pub masterable: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rarity")]
    pub rarity: Option<String>,
    #[serde(rename = "tradable")]
    pub tradable: bool,
    #[serde(rename = "type")]
    pub arcane_type: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "excludeFromCodex")]
    pub exclude_from_codex: Option<bool>,
    #[serde(rename = "buildPrice")]
    pub build_price: Option<i32>,
    #[serde(rename = "buildQuantity")]
    pub build_quantity: Option<i32>,
    #[serde(rename = "buildTime")]
    pub build_time: Option<i32>,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
    #[serde(rename = "consumeOnBuild")]
    pub consume_on_build: Option<bool>,
    #[serde(rename = "skipBuildTimePrice")]
    pub skip_build_time_price: Option<i32>,
}
