use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::cache_arcane::CacheArcane}, utils::modules::error::AppError,
    
};

#[derive(Clone, Debug)]
pub struct ArcaneModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheArcane>,
}

impl ArcaneModule {
    pub fn new(client: CacheClient) -> Self {
        ArcaneModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Arcane".to_string(),
            path: PathBuf::from("item/Arcanes.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_arcane_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheArcane> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ArcaneModule from file: {}", e)),
            )
        })?;
        self.items = items;
        self.update_state();
        Ok(())
    }
    pub fn get_by_unique_name(&self, id: &str) -> Option<CacheArcane> {
        self.items.iter().find(|x| x.unique_name == id).cloned()
    }
    pub fn get_by_name(&self, name: &str, ignore_case: bool) -> Option<CacheArcane> {
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
