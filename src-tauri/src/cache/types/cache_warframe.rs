use serde::{Deserialize, Serialize};

use super::{cache_item_base::CacheItemBase, cache_item_component::CacheItemComponent};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheWarframe {
    #[serde(rename = "armor")]
    pub armor: i32,
    #[serde(rename = "aura")]
    pub aura: Option<String>,
    #[serde(rename = "bpCost")]
    pub bp_cost: Option<i32>,
    #[serde(rename = "buildPrice")]
    pub build_price: Option<i32>,
    #[serde(rename = "buildQuantity")]
    pub build_quantity: Option<i32>,
    #[serde(rename = "buildTime")]
    pub build_time: Option<i32>,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "color")]
    pub color: Option<i32>,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
    #[serde(rename = "conclave")]
    pub conclave: Option<bool>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "health")]
    pub health: i32,
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[serde(rename = "isPrime")]
    pub is_prime: bool,
    #[serde(rename = "marketCost")]
    pub market_cost: Option<i32>,
    #[serde(rename = "masterable")]
    pub masterable: bool,
    #[serde(rename = "masteryReq")]
    pub mastery_req: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passiveDescription")]
    pub passive_description: Option<String>,
    #[serde(rename = "polarities")]
    pub polarities: Option<Vec<String>>,
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "productCategory")]
    pub product_category: String,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "sex")]
    pub sex: Option<String>,
    #[serde(rename = "shield")]
    pub shield: i32,
    #[serde(rename = "sprint")]
    pub sprint: Option<f64>,
    #[serde(rename = "sprintSpeed")]
    pub sprint_speed: f64,
    #[serde(rename = "stamina")]
    pub stamina: i32,
    #[serde(rename = "tradable")]
    pub tradable: bool,
    #[serde(rename = "type")]
    pub warframe_type: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "wikiaThumbnail")]
    pub wikia_thumbnail: Option<String>,
    #[serde(rename = "wikiaUrl")]
    pub wikia_url: Option<String>,
    #[serde(rename = "estimatedVaultDate")]
    pub estimated_vault_date: Option<String>,
    #[serde(rename = "vaultDate")]
    pub vault_date: Option<String>,
    #[serde(rename = "vaulted")]
    pub vaulted: Option<bool>,
    #[serde(rename = "exalted")]
    pub exalted: Option<Vec<String>>,
}
impl CacheWarframe {
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
