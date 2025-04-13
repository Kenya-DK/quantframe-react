use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_item_base::CacheItemBase, cache_mod::CacheMod},
    },
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct ModModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheMod>,
}

impl ModModule {
    pub fn new(client: CacheClient) -> Self {
        ModModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Mod".to_string(),
            path: PathBuf::from("items/Mods.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_mods_module(self.clone());
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
        let items: Vec<CacheMod> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ModModule from file: {}", e)),
            )
        })?;
        self.items = items;
        self.update_state();
        Ok(())
    }
}
