use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{
            cache_arch_gun::CacheArchGun, cache_item_base::CacheItemBase,
            cache_item_component::CacheItemComponent,
        },
    },
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct ArchGunModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheArchGun>,
    pub components: Vec<CacheItemComponent>,
}

impl ArchGunModule {
    pub fn new(client: CacheClient) -> Self {
        ArchGunModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "ArchGun".to_string(),
            path: PathBuf::from("items/Arch-Gun.json"),
            items: Vec::new(),
            components: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_arch_gun_module(self.clone());
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
        items.append(
            &mut self
                .components
                .iter()
                .map(|item| item.convert_to_base_item())
                .collect(),
        );
        items
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
        for item in items {
            if item.components.is_none() {
                continue;
            }
            self.components.append(&mut item.components.unwrap());
        }
        self.update_state();
        Ok(())
    }
}
