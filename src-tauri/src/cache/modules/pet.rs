use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_item_base::CacheItemBase, cache_pet::CachePet},
    },
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct PetModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CachePet>,
}

impl PetModule {
    pub fn new(client: CacheClient) -> Self {
        PetModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Pet".to_string(),
            path: PathBuf::from("items/Pets.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_pet_module(self.clone());
    }
    pub fn get_all(&self) -> Vec<CacheItemBase> {
        let mut items: Vec<CacheItemBase> = Vec::new();
        items.append(
            &mut self
                .items
                .iter()
                .map(|item| item.convert_to_base_item())
                .collect(),
        );
        items
    }
    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CachePet> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse PetModule from file: {}", e)),
            )
        })?;
        self.items = items;

        self.update_state();
        Ok(())
    }
}
