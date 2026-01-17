use serde::{Deserialize, Serialize};

use super::{cache_item_base::CacheItemBase, cache_item_component::CacheItemComponent};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheWarframe {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components", default)]
    pub components: Vec<CacheItemComponent>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "wfm_item_url")]
    pub wfm_item_url: Option<String>,
    #[serde(rename = "tradable")]
    #[serde(default)]
    pub tradable: bool,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "wikiaThumbnail")]
    pub wikia_thumbnail: Option<String>,
    #[serde(rename = "wikiaUrl")]
    pub wikia_url: Option<String>,
}
impl CacheWarframe {
    pub fn convert_to_base_item(&self) -> CacheItemBase {
        CacheItemBase {
            unique_name: self.unique_name.clone(),
            name: self.name.clone(),
            wfm_item_url: self.wfm_item_url.clone(),
            category: self.category.clone(),
            part_of_set: None,
            components: self.components.clone(),
        }
    }
}
