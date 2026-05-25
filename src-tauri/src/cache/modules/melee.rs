use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct MeleeModule {
    path: PathBuf,
    items: Mutex<Vec<CacheMelee>>,
}

impl MeleeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Melee.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, _language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMelee>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                info(
                    "Cache:Melee:load",
                    format!("Loaded {} Melee items", items.len()),
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
    pub fn get_all_items(&self) -> Result<Vec<CacheMelee>, Error> {
        let items_lock = self.items.lock().unwrap();
        Ok(items_lock.clone())
    }
}
