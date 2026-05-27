use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct SentinelModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheSentinel>>,
}

impl SentinelModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Sentinels.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheSentinel>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                *lookup = MultiKeyMap::new();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let keys = vec![item.base.name.clone(), item.base.unique_name.clone()];
                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Sentinel:load",
                    format!("Loaded {} Sentinel items", lookup.len()),
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
    /// Get a resource item by various identifiers
    ///  # Arguments
    /// - `id`: The identifier to search for (name, url, unique name, or id)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheSentinel, Error> {
        let id: String = id.into().trim_end().to_string();
        let lookup = self.lookup.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:Sentinel:GetBy",
                format!("Sentinel item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheSentinel>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
