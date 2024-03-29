use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        structs::{CacheItemComponent, CacheSentinel},
    },
    error::AppError,
};

#[derive(Clone, Debug)]
pub struct SentinelModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheSentinel>,
    pub parts: HashMap<String, CacheItemComponent>,
}

impl SentinelModule {
    pub fn new(client: CacheClient) -> Self {
        SentinelModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Sentinel".to_string(),
            path: PathBuf::from("item/Sentinels.json"),
            items: Vec::new(),
            parts: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_sentinel_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheSentinel> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse SentinelModule from file: {}", e)),
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
}
