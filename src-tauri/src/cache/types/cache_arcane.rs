use serde::{Deserialize, Serialize};

use super::cache_item_component::CacheItemComponent;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArcane {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
