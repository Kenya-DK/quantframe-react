use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_item_component::CacheItemComponent, cache_secondary::CacheSecondary},
    },
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct SecondaryModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheSecondary>,
    pub parts: HashMap<String, CacheItemComponent>,
}

impl SecondaryModule {
    pub fn new(client: CacheClient) -> Self {
        SecondaryModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Secondary".to_string(),
            path: PathBuf::from("item/Secondary.json"),
            items: Vec::new(),
            parts: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_secondary_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheSecondary> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse SecondaryModule from file: {}", e)),
            )
        })?;
        self.items = items.clone();
        // loop through items and add parts to parts
        for item in items {
            let components = item.get_item_components();
            for part in components {
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
    pub fn get_by_unique_name(&self, id: &str) -> Option<CacheSecondary> {
        self.items.iter().find(|x| x.unique_name == id).cloned()
    }
    pub fn get_by_name(&self, name: &str, ignore_case: bool) -> Option<CacheSecondary> {
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
