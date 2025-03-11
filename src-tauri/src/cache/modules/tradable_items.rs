use std::{collections::HashMap, path::PathBuf};

use actix_web::cookie::time::ext;
use entity::{stock::item::create::CreateStockItem, wish_list::create::CreateWishListItem};
use eyre::eyre;
use serde_json::json;

use crate::{
    cache::{client::CacheClient, types::{cache_item_base::CacheItemBase, cache_tradable_item::CacheTradableItem}},
    helper,
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
            path: PathBuf::from("items/TradableItems.json"),
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

    pub fn get_item_dict(&self, by: &str) -> Result<HashMap<String, CacheTradableItem>, AppError> {
        let items = self.items.clone();
        let args = match helper::validate_args(by, vec!["--item_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--item_by").unwrap();

        let item_dict = if mode == "name" {
            items
                .iter()
                .map(|x| (x.name.clone(), x.clone()))
                .collect::<HashMap<String, CacheTradableItem>>()
        } else if mode == "url_name" {
            items
                .iter()
                .map(|x| (x.wfm_url_name.clone(), x.clone()))
                .collect::<HashMap<String, CacheTradableItem>>()
        } else if mode == "unique_name" {
            items
                .iter()
                .map(|x| (x.unique_name.clone(), x.clone()))
                .collect::<HashMap<String, CacheTradableItem>>()
        } else {
            return Err(AppError::new(
                &self.get_component("GetBy"),
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(item_dict)
    }

    pub fn get_by(&self, input: &str, by: &str) -> Result<Option<CacheTradableItem>, AppError> {
        let items = self.items.clone();
        let args = match helper::validate_args(by, vec!["--item_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--item_by").unwrap();
        let case_insensitive = args.get("--ignore_case").is_some();
        // let lang = args.get("--item_lang").unwrap_or(&"en".to_string());
        let remove_string = args.get("--remove_string");

        let item = if mode == "name" {
            items
                .iter()
                .find(|x| helper::is_match(&x.name, input, case_insensitive, remove_string))
                .cloned()
        } else if mode == "url_name" {
            items
                .iter()
                .find(|x| helper::is_match(&x.wfm_url_name, input, case_insensitive, remove_string))
                .cloned()
        } else if mode == "unique_name" {
            items
                .iter()
                .find(|x| helper::is_match(&x.unique_name, input, case_insensitive, remove_string))
                .cloned()
        } else {
            return Err(AppError::new(
                &self.get_component("GetBy"),
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(item)
    }

    pub fn validate_create_item(
        &self,
        input: &mut CreateStockItem,
        by: &str,
    ) -> Result<CreateStockItem, AppError> {
        let component = "ValidateCreateItem";

        if input.is_validated {
            return Ok(input.clone());
        }

        let item = self.get_by(&input.raw, by)?;
        if item.is_none() {
            let mut extra: serde_json::Value = json!(input);
            extra["notification"] = json!({
                "i18n_key_title":"created_stock.warning.not_found.title",
                "i18n_key_message":"created_stock.warning.not_found.message",
                "values": {
                    "item": input.raw,
                    "by": by
                }
            });
            return Err(AppError::new_with_level(
                component,
                eyre!(
                    "Item not found: {} | By: {}[J]{}[J]",
                    input.raw,
                    by,
                    extra.to_string()
                ),
                crate::utils::enums::log_level::LogLevel::Warning,
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
    pub fn validate_create_wish_item(
        &self,
        input: &mut CreateWishListItem,
        by: &str,
    ) -> Result<CreateWishListItem, AppError> {
        let component = "ValidateWishItem";

        if input.is_validated {
            return Ok(input.clone());
        }

        let item = self.get_by(&input.raw, by)?;
        if item.is_none() {
            let mut extra = json!(input);
            extra["notification"] = json!({
                "i18n_key_title":"create_wish_item.warning.not_found.title",
                "i18n_key_message":"create_wish_item.warning.not_found.message",
                "values": {
                    "item": input.raw,
                    "by": by
                }
            });
            return Err(AppError::new_with_level(
                component,
                eyre!(
                    "Item not found: {} | By: {}[J]{}[J]",
                    input.raw,
                    by,
                    extra.to_string()
                ),
                crate::utils::enums::log_level::LogLevel::Warning,
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
