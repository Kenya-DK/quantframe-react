use std::collections::HashMap;

use serde_json::json;

use crate::{
    cache::{client::CacheClient, types::cache_item_component::CacheItemComponent},
    utils::modules::{error::AppError, logger},
};

#[derive(Clone, Debug)]
pub struct PartModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    pub warframe_parts: HashMap<String, CacheItemComponent>,
    pub weapon_parts: HashMap<String, CacheItemComponent>,
    pub skin_parts: HashMap<String, CacheItemComponent>,
}

impl PartModule {
    pub fn new(client: CacheClient) -> Self {
        PartModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Part".to_string(),
            warframe_parts: HashMap::new(),
            weapon_parts: HashMap::new(),
            skin_parts: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_part_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let cache = self.client.clone();

        // Initialize the warframe parts
        self.add_parts("Warframe", cache.warframe().get_parts());
        self.add_parts("Warframe", cache.archwing().get_parts());

        // Initialize the weapon parts
        self.add_parts("Weapon", cache.primary().get_parts());
        self.add_parts("Weapon", cache.secondary().get_parts());
        self.add_parts("Weapon", cache.melee().get_parts());
        self.add_parts("Weapon", cache.sentinel().get_parts());
        self.add_parts("Weapon", cache.arch_gun().get_parts());
        self.add_parts("Weapon", cache.arch_melee().get_parts());
        self.add_parts(
            "Weapon",
            cache
                .skin()
                .get_by("/Lotus/Upgrades/Skins/Kubrows/Collars/PrimeKubrowCollarA", "--item_by unique_name")?
                .unwrap()
                .get_item_components(),
        );
        
        self.update_state();
        Ok(())
    }
    fn add_part(&mut self, category: &str, item: CacheItemComponent) {
        match category {
            "Warframe" => {
                self.warframe_parts.insert(item.unique_name.clone(), item);
            }
            "Weapon" => {
                self.weapon_parts.insert(item.unique_name.clone(), item);
            }
            "Skin" => {
                self.skin_parts.insert(item.unique_name.clone(), item);
            }
            _ => {}
        }
    }
    fn add_parts(&mut self, category: &str, items: Vec<CacheItemComponent>) {
        match category {
            "Warframe" => {
                for item in items {
                    self.warframe_parts.insert(item.unique_name.clone(), item);
                }
            }
            "Weapon" => {
                for item in items {
                    self.weapon_parts.insert(item.unique_name.clone(), item);
                }
            }
            "Skin" => {
                for item in items {
                    self.skin_parts.insert(item.unique_name.clone(), item);
                }
            }
            _ => {}
        }
    }
    pub fn get_part_by_name(
        &self,
        category: &str,
        name: &str,
        use_external: bool,
    ) -> Option<CacheItemComponent> {
        let items = self.get_parts(category);
        let mut result: Option<CacheItemComponent> = None;
        for item in items {
            if use_external {
                if item.get_real_external_name() == name {
                    result = Some(item.clone());
                    break;
                }
            }
            if item.name == name {
                result = Some(item.clone());
                break;
            }
        }
        result
    }
    pub fn get_parts(&self, category: &str) -> Vec<CacheItemComponent> {
        let items = match category {
            "All" => {
                let mut all_items: Vec<CacheItemComponent> = Vec::new();
                all_items.extend(self.warframe_parts.values().cloned());
                all_items.extend(self.weapon_parts.values().cloned());
                all_items.extend(self.skin_parts.values().cloned());
                all_items
            },
            "Warframe" => self.warframe_parts.values().cloned().collect(),
            "Weapon" => self.weapon_parts.values().cloned().collect(),
            "Skin" => self.skin_parts.values().cloned().collect(),
            _ => {
                return Vec::new();
            }
        };
        items
    }
}
