use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct SentinelModule {
    path: PathBuf,
    items: Mutex<Vec<CacheSentinel>>,
}

impl SentinelModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Sentinels.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, _language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheSentinel>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                info(
                    "Cache:Sentinel:load",
                    format!("Loaded {} Sentinel items", items.len()),
                    &LoggerOptions::default(),
                );
                *items_lock = items.clone();
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheSentinel>, Error> {
        let items = self.items.lock().unwrap();
        Ok(items.clone())
    }
}
