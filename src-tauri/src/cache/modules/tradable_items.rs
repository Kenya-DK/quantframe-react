use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{client::CacheState, modules::LanguageModule, types::CacheTradableItem};

#[derive(Debug)]
pub struct TradableItemModule {
    path: PathBuf,
    items: Mutex<Vec<CacheTradableItem>>,

    // Lookup maps
    item_lookup: Mutex<MultiKeyMap<CacheTradableItem>>,
}

impl TradableItemModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/TradableItems.json"),
            items: Mutex::new(Vec::new()),
            item_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheTradableItem>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheTradableItem>>(&self.path) {
            Ok(mut items) => {
                let mut item_lookup = self.item_lookup.lock().unwrap();
                for item in items.iter_mut() {
                    item.name = language
                        .translate(
                            &item.unique_name,
                            crate::cache::modules::LanguageKey::WfmName,
                        )
                        .unwrap_or(item.name.clone());
                    item_lookup.insert_value(
                        item.clone(),
                        vec![
                            item.wfm_id.clone(),
                            item.name.clone(),
                            item.wfm_url_name.clone(),
                            item.unique_name.clone(),
                        ],
                    );
                }

                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:TradableItem:load",
                    format!("Loaded {} tradable items from cache", items_lock.len()),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    /// Get a tradable item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, url, unique name, or id)
    ///
    pub fn get_by(&self, item_id: impl Into<String>) -> Result<CacheTradableItem, Error> {
        let item_id: String = item_id.into();
        let item_lookup = self.item_lookup.lock().unwrap();
        if let Some(item) = item_lookup.get(&item_id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:TradableItem:GetBy",
                format!("Tradable item not found for id '{}'", item_id),
                get_location!(),
            ))
        }
    }
<<<<<<< HEAD

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
        } else if mode == "id" {
            items
                .iter()
                .find(|x| helper::is_match(&x.wfm_id, input, case_insensitive, remove_string))
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
=======
    /**
     * Creates a new `TradableItemModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &TradableItemModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            item_lookup: Mutex::new(old.item_lookup.lock().unwrap().clone()),
        })
>>>>>>> better-backend
    }
}
