use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct RivenGoodRollModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheRivenRoll>>,
}

impl RivenGoodRollModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/RivenGodRolls.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheRivenRoll>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                info(
                    "Cache:RivenGoodRoll:load",
                    format!("Loaded {} Riven good rolls from cache", items.len()),
                    &LoggerOptions::default(),
                );
                for item in items.drain(..) {
                    let keys = vec![item.unique_name.clone()];
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
    /// - `item_id`: The identifier to search for (unique_name)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheRivenRoll, Error> {
        let id: String = id.into();
        let lookup = self.lookup.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:GetItemById",
                format!("Item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /**
     * Creates a new `RivenGoodRollModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenGoodRollModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            lookup: Mutex::new(old.lookup.lock().unwrap().clone()),
        })
    }
}
