use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct MiscModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheMisc>>,
}

impl MiscModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Misc.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, _language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMisc>>(&self.path) {
            Ok(mut items) => {
                let mut lookup_lock = self.lookup.lock().unwrap();
                for item in items.iter_mut() {
                    let mut keys = vec![item.base.name.clone(), item.base.unique_name.clone()];
                    keys.extend(item.base.previous_names.clone());
                    lookup_lock.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Misc:load",
                    format!("Loaded {} Misc items", items.len()),
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
    /// Get a misc item by various identifiers
    ///  # Arguments
    /// - `id`: The identifier to search for (name, url, unique name, or id)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheMisc, Error> {
        let id: String = id.into().trim_end().to_string();
        let lookup = self.lookup.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:Misc:GetBy",
                format!("Misc item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheMisc>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
