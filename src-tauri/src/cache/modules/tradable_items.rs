use std::path::PathBuf;

use entity::stock::item::create::CreateStockItem;
use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::cache_tradable_item::CacheTradableItem}, helper, utils::modules::error::AppError
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

    pub fn find_item(
        &self,
        input: &str,
        by: &str,
    ) -> Result<Option<CacheTradableItem>, AppError> {
        let items = self.items.clone();
        let args = helper::parse_args_from_string(by);
        let mode = args.get("--item_by");
        if mode.is_none() {
            return Err(AppError::new(
                "FindItem",
                eyre!("Missing item_by argument"),
            ));
        }
        let mode = mode.unwrap();

        let riven_type = if mode =="name" {
            let lang = args.get("--item_lang");
            if lang.is_none() {
                return Err(AppError::new(
                    "item_lang",
                    eyre!("Missing item_lang argument"),
                ));
            }
            items.iter().find(|x| x.name == input).cloned()
        } else if mode == "url_name" {
            items.iter().find(|x| x.wfm_url_name == input).cloned()
        } else if mode == "unique_name" {
             items.iter().find(|x| x.unique_name == input).cloned()
        } else {
            return Err(AppError::new(
                "FindItem",
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(riven_type)
    }

    pub fn validate_create_item(
        &self,
        input: &mut CreateStockItem,
        by: &str,
    ) -> Result<CreateStockItem, AppError> {
        let component = "ValidateCreateItem";

        let item = self.find_item(&input.wfm_url, by)?;
        if item.is_none() {
            return Err(AppError::new(
                component,
                eyre!("Invalid item value Name: {} | Url: {} | Unique: {} | By: {}", input.item_name, input.wfm_url, input.item_unique_name, by),
            ));
        }

        let item = item.unwrap();
        input.wfm_id = item.wfm_id.clone();
        input.wfm_url = item.wfm_url_name.clone();
        input.item_name = item.name.clone();
        input.item_unique_name = item.unique_name.clone();
        input.tags = item.tags.clone();
        Ok(input.clone())
    }


}
