use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct PrimaryModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CachePrimary>>,
}

impl PrimaryModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Primary.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CachePrimary>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                *lookup = MultiKeyMap::new();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let keys = vec![item.base.name.clone(), item.base.unique_name.clone()];
                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Primary:load",
                    format!("Loaded {} Primary items", lookup.len()),
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
    pub fn get_all_items(&self) -> Result<Vec<CachePrimary>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
