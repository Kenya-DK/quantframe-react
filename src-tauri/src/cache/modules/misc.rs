use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct MiscModule {
    path: PathBuf,
    items: Mutex<Vec<CacheMisc>>,
}

impl MiscModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Misc.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMisc>>(&self.path) {
            Ok(mut items) => {
                let mut items_lock = self.items.lock().unwrap();
                info(
                    "Cache:Misc:load",
                    format!("Loaded {} Misc items", items.len()),
                    &LoggerOptions::default(),
                );
                *items_lock = items.clone();
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }

    /**
     * Creates a new `MiscModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &MiscModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
