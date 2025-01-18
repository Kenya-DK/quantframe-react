use serde::{Deserialize, Serialize};

use super::cache_item_base::CacheItemBase;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheItemComponent {
    #[serde(rename = "part_of")]
    pub part_of: Option<CacheItemBase>,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "itemCount")]
    pub item_count: i64,
    #[serde(rename = "tradable")]
    #[serde(default)]
    pub tradable: bool,
    #[serde(rename = "componentType")]
    pub component_type: String,
}
impl CacheItemComponent {
    pub fn get_real_external_name(&self) -> String {
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
