use serde::{Deserialize, Serialize};

use super::{cache_item_base::CacheItemBase, cache_item_component::CacheItemComponent};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CachePrimary {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "wfm_item_url")]
    pub wfm_item_url: Option<String>,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CachePrimary {
    pub fn convert_to_base_item(&self) -> CacheItemBase {
        CacheItemBase {
            unique_name: self.unique_name.clone(),
            name: self.name.clone(),
            wfm_item_url: self.wfm_item_url.clone(),
            part_of_set: None,
            category: self.category.clone(),
            components: self.components.clone(),
        }
    }
}
