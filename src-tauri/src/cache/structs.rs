use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};

use super::client::CacheClient;
// Base Structure for the cache
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemComponent {
    #[serde(rename = "part_of")]
    pub part_of: Option<CacheItemBase>,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "itemCount")]
    pub item_count: i32,
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[serde(rename = "tradable")]
    pub tradable: bool,
    #[serde(rename = "masterable")]
    pub masterable: bool,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "primeSellingPrice")]
    pub prime_selling_price: Option<i32>,
    #[serde(rename = "ducats")]
    pub ducats: Option<i32>,
    #[serde(rename = "componentType")]
    pub component_type: String,
}
impl CacheItemComponent {
    pub fn get_real_external_name(&self, cache: CacheClient) -> String {
        let mut real_external_name = self.name.clone();
        let part_of = self.part_of.clone();
        if real_external_name == "Forma" {
            real_external_name = "Forma Blueprint".to_string();
        } else {
            if real_external_name.contains("Kavasa Prime")
                || real_external_name.contains("Orokin Cell")
                || self.unique_name.contains("/Resources/")
                || self.unique_name.contains("/Types/Items/")
            {
                return self.name.clone();
            }
            if part_of.clone().is_some() {
                real_external_name =
                    format!("{} {}", part_of.clone().unwrap().name, &real_external_name);
                return real_external_name;
            }
        }
        if real_external_name.contains("Voidrig") {
            real_external_name = real_external_name.replace("Voidrig Voidrig", "Voidrig");
        }
        if real_external_name.contains("Bonewidow") {
            real_external_name = real_external_name.replace("Bonewidow Bonewidow", "Bonewidow");
        }
        if real_external_name.contains("War war") {
            real_external_name = real_external_name.replace("War War", "War");
        }
        if real_external_name.contains("War War") {
            real_external_name = real_external_name.replace("War War", "War");
        }
        if real_external_name.contains("Decurion decurion") {
            real_external_name = real_external_name.replace("Decurion Decurion", "Decurion");
        }
        if real_external_name.contains("Decurion Decurion") {
            real_external_name = real_external_name.replace("Decurion Decurion", "Decurion");
        }
        if real_external_name == "Broken War Blade" {
            real_external_name = "War Blade".to_string();
        }
        if real_external_name == "Broken War Hilt" {
            real_external_name = "War Hilt".to_string();
        }
        if real_external_name.contains("Dual Decurion") && !real_external_name.contains("Blueprint")
        {
            real_external_name = real_external_name.replace("Dual Decurion", "Decurion");
        }
        if part_of.clone().is_some() && !part_of.clone().unwrap().name.contains("Ambassador") {
            let is_warframe = part_of.clone().unwrap().category == "Warframes";
            if !is_warframe {}

            if is_warframe && !real_external_name.ends_with("Blueprint") {
                real_external_name = format!("{} Blueprint", real_external_name);
            }
        }
        real_external_name
    }
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemBase {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}

// End of the base structure for the cache
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheTradableItem {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSecondary {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheSecondary {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArchMelee {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheArchMelee {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheArchwing {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheArchwing {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheMelee {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheMelee {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheMod {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CachePrimary {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CachePrimary {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSentinel {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheSentinel {
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheResource {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheFish {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CachePet {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheMisc {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheSkin {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "components")]
    pub components: Option<Vec<CacheItemComponent>>,
}
impl CacheSkin {
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
