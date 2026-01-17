use serde::{Deserialize, Serialize};

use super::cache_item_base::CacheItemBase;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemComponent {
    #[serde(rename = "part_of_set")]
    #[serde(default)]
    pub part_of_set: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "itemCount")]
    pub item_count: i64,
    #[serde(rename = "tradable")]
    #[serde(default)]
    pub tradable: bool,
    #[serde(rename = "wfm_item_url")]
    pub wfm_item_url: Option<String>,
}
impl CacheItemComponent {
    pub fn convert_to_base_item(&self) -> CacheItemBase {
        CacheItemBase {
            unique_name: self.unique_name.clone(),
            name: self.name.clone(),
            wfm_item_url: self.wfm_item_url.clone(),
            category: "Component".to_string(),
            part_of_set: Some(self.part_of_set.clone()),
            components: None,
        }
    }
    pub fn display(&self) -> String {
        format!(
            "Name: {}, Unique Name: {}, Item Count: {}",
            self.name, self.unique_name, self.item_count
        )
    }
}
