use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::cache_resource::CacheResource},
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct ResourceModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheResource>,
}

impl ResourceModule {
    pub fn new(client: CacheClient) -> Self {
        ResourceModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Resource".to_string(),
            path: PathBuf::from("item/Resources.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_resource_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheResource> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ResourceModule from file: {}", e)),
            )
        })?;
        self.items = items;
        self.update_state();
        Ok(())
    }
    pub fn get_by_name(&self, name: &str, ignore_case: bool) -> Option<CacheResource> {
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
