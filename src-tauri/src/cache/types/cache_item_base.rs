use serde::{Deserialize, Serialize};

use super::cache_item_component::CacheItemComponent;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemBase {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "wfm_item_url")]
    pub wfm_item_url: Option<String>,
    #[serde(rename = "part_of_set")]
    pub part_of_set: Option<String>,
    #[serde(rename = "components", default)]
    pub components: Vec<CacheItemComponent>,
}
impl CacheItemBase {
    pub fn get_tradable_components(&self) -> Vec<CacheItemComponent> {
        self.components
            .iter()
            .filter(|component| component.tradable)
            .cloned()
            .collect()
    }
    pub fn display(&self) -> String {
        format!(
            "Name: {}, Unique Name: {}, Category: {}, Components: {}",
            self.name,
            self.unique_name,
            self.category,
            self.components.len(),
        )
    }
}
