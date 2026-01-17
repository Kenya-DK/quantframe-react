use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::*;

#[derive(Debug)]
pub struct ChatIconModule {
    path: PathBuf,
    items: Mutex<Vec<CacheChatIcon>>,
}

impl ChatIconModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/ChatIcons.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheChatIcon>, Error> {
        let data = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(data)
    }
    pub fn load(&self) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheChatIcon>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:ChatIconModule::Load",
                    format!("Loaded {} ChatIcon items", items_lock.len()),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /**
     * Creates a new `ChatIconModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ChatIconModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
