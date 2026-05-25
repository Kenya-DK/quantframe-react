use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct PrimaryModule {
    client: Weak<CacheState>,
    path: PathBuf,
    items: Mutex<Vec<CachePrimary>>,
}

impl PrimaryModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            path: client.base_path.join("items/Primary.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, _language: &LanguageModule) -> Result<(), Error> {
        let _client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file_optional::<Vec<CachePrimary>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                // for item in items.iter_mut() {
                //     client.add_weapon(item.base.clone());
                // }
                info(
                    "Cache:Primary:load",
                    format!("Loaded {} Primary items", items.len()),
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
    pub fn get_all_items(&self) -> Result<Vec<CachePrimary>, Error> {
        let items_lock = self.items.lock().unwrap();
        Ok(items_lock.clone())
    }
}
