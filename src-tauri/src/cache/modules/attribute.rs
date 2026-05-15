use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct AttributeModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheAttribute>>,
}
impl AttributeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Attributes.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheAttribute>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                info(
                    "Cache:Attribute:load",
                    format!("Loaded {} Attribute items", items.len()),
                    &LoggerOptions::default(),
                );
                for item in items.drain(..) {
                    let keys = vec![
                        item.label.clone(),
                        item.wfm_url.clone(),
                        item.unique_name.clone(),
                    ];
                    lookup.insert_value(item, keys);
                }
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    /// Get Item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheAttribute, Error> {
        let id: String = id.into();
        let lookup = self.lookup.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "CacheAttribute:GetItemById",
                format!("Item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_items(&self) -> Result<Vec<CacheAttribute>, Error> {
        let items_lock = self.lookup.lock().unwrap();
        Ok(items_lock.get_all_values())
    }
    /**
     * Creates a new `AttributeModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &AttributeModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            lookup: Mutex::new(old.lookup.lock().unwrap().clone()),
        })
    }
}
