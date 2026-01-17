use serde::{Deserialize, Serialize};

use super::{cache_item_base::CacheItemBase, cache_item_component::CacheItemComponent};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArchGun {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheArchGun {
    pub fn get_item_components(&self) -> Vec<CacheItemComponent> {
        // Loop through the components and return the components
        match &self.components {
            Some(components) => {
                let mut component_list: Vec<CacheItemComponent> = Vec::new();
                for mut component in components.clone() {
                    component.part_of = Some(self.convert_to_base_item());
                    if !&self.name.contains("Prime") {
                        component.tradable = false;
                    }
                    component_list.push(component.clone());
                }
                component_list
            }
            None => Vec::new(),
        }
    }
    pub fn convert_to_base_item(&self) -> CacheItemBase {
        CacheItemBase {
            unique_name: self.unique_name.clone(),
            name: self.name.clone(),
            category: self.category.clone(),
            components: self.components.clone(),
        }
    }
}
