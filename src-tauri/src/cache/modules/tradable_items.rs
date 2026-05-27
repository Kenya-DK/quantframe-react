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
                    item.translate(&language);

                    let mut keys = vec![
                        item.wfm_id.clone(),
                        item.name.clone(),
                        item.wfm_url.clone(),
                        item.unique_name.clone(),
                    ];
                    if !item.variant_to_unique_name.is_empty() {
                        keys.extend(item.variant_to_unique_name.values().cloned());
                    }

                    item_lookup.insert_value(item.clone(), keys);
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
}
