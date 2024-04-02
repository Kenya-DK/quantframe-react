use std::path::PathBuf;

use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::cache_tradable_item::CacheTradableItem},
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct TradableItemModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CacheTradableItem>,
}

impl TradableItemModule {
    pub fn new(client: CacheClient) -> Self {
        TradableItemModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "TradeableItem".to_string(),
            path: PathBuf::from("item/tradeableItems.json"),
            items: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_tradable_items_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CacheTradableItem> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!(
                    "Failed to parse TradableItemModule from file: {}",
                    e
                )),
            )
        })?;
        self.items = items;
        self.update_state();
        Ok(())
    }
    // Method to get the list of tradable items
    pub fn get_items(&self) -> Result<Vec<CacheTradableItem>, AppError> {
        Ok(self.items.clone())
    }
}
