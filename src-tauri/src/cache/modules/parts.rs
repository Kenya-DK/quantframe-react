use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::cache_item_component::CacheItemComponent},
    utils::modules::error::AppError,
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
                .get_by_unique_name("/Lotus/Upgrades/Skins/Kubrows/Collars/PrimeKubrowCollarA")
                .unwrap()
                .get_item_components(),
        );

        // let content = self.client.read_text_from_file(&self.path)?;
        // let items: Vec<CacheSecondary> = serde_json::from_str(&content).map_err(|e| {
        //     AppError::new(
        //         self.get_component("Load").as_str(),
        //         eyre!(format!("Failed to parse PartModule from file: {}", e)),
        //     )
        // })?;
        // self.items = items.clone();
        // // loop through items and add parts to parts
        // for item in items {
        //     let components = item.get_item_components();
        //     for part in components {
        //         self.add_part(part);
        //     }
        // }
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
        cache: CacheClient,
        name: &str,
        use_external: bool,
    ) -> Option<CacheItemComponent> {
        let items = match category {
            "Warframe" => self.warframe_parts.values(),
            "Weapon" => self.weapon_parts.values(),
            "Skin" => self.skin_parts.values(),
            _ => {
                return None;
            }
        };
        let mut result: Option<CacheItemComponent> = None;
        for item in items {
            if use_external {
                if item.get_real_external_name(cache.clone()) == name {
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
}
