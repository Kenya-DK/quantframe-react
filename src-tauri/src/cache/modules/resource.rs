use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ResourceModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheResource>>,
}

impl ResourceModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Resources.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheResource>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                *lookup = MultiKeyMap::new();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let mut keys = vec![item.base.name.clone(), item.base.unique_name.clone()];
                    keys.extend(item.base.previous_names.clone());
                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Resource:load",
                    format!("Loaded {} Resource items", lookup.len()),
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
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheResource, Error> {
        let id: String = id.into().trim_end().to_string();
        let lookup = self.lookup.lock().unwrap();
        if let Some(item) = lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:Resources:GetBy",
                format!("Resource item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheResource>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
