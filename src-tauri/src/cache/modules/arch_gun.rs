use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_arch_gun::CacheArchGun, cache_item_component::CacheItemComponent},
    },
    utils::modules::{error::AppError},
};

#[derive(Clone, Debug)]
pub struct ArchGunModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheArchGun>,
    pub parts: HashMap<String, CacheItemComponent>,
}

impl ArchGunModule {
    pub fn new(client: CacheClient) -> Self {
        ArchGunModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "ArchGun".to_string(),
            path: PathBuf::from("item/Arch-Gun.json"),
            items: Vec::new(),
            parts: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_arch_gun_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheArchGun> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ArchGunModule from file: {}", e)),
            )
        })?;
        self.items = items.clone();
        // loop through items and add parts to parts
        for item in items {
            let components = item.get_item_components();
            for mut part in components {
                part.part_of =Some(item.convert_to_base_item());
                self.add_part(part);
            }
        }
        self.update_state();
        Ok(())
    }
    fn add_part(&mut self, item: CacheItemComponent) {
        self.parts.insert(item.unique_name.clone(), item);
    }
    pub fn get_parts(&self) -> Vec<CacheItemComponent> {
        let mut result: Vec<CacheItemComponent> = Vec::new();
        for item in self.parts.values() {
            result.push(item.clone());
        }
        result
    }
    pub fn get_by_unique_name(&self, id: &str) -> Option<CacheArchGun> {
        self.items.iter().find(|x| x.unique_name == id).cloned()
    }
    pub fn get_by_name(&self, name: &str, ignore_case: bool) -> Option<CacheArchGun> {
        if ignore_case {
            self.items
                .iter()
                .find(|x| x.name.to_lowercase() == name.to_lowercase())
                .cloned()
        } else {
            self.items.iter().find(|x| x.name == name).cloned()
        }
    }
}
