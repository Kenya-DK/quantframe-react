use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, read_json_file_optional, Error};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct WarframeModule {
    path: PathBuf,
    items: Mutex<Vec<CacheWarframe>>,
}

impl WarframeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Warframes.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, _language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheWarframe>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();

                *items_lock = items.clone();
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheWarframe>, Error> {
        let items = self.items.lock().unwrap();
        Ok(items.clone())
    }
}
