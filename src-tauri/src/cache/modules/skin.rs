use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{client::CacheClient, structs::CacheSkin},
    error::AppError,
};

#[derive(Clone, Debug)]
pub struct SkinModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheSkin>,
}

impl SkinModule {
    pub fn new(client: CacheClient) -> Self {
        SkinModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Skin".to_string(),
            path: PathBuf::from("item/Skins.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_skin_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheSkin> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse SkinModule from file: {}", e)),
            )
        })?;
        self.items = items;
        self.update_state();
        Ok(())
    }
    pub fn get_by_unique_name(&self, id: &str) -> Option<CacheSkin> {
        self.items.iter().find(|x| x.unique_name == id).cloned()
    }
    pub fn get_by_name(&self, name: &str, ignore_case: bool) -> Option<CacheSkin> {
        if ignore_case {
            self.items.iter().find(|x| x.name.to_lowercase() == name.to_lowercase()).cloned()
        } else {
            self.items.iter().find(|x| x.name == name).cloned()
        }
    }
}
