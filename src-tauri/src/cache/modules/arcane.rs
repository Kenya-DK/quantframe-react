use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};
use utils::{find_by, get_location, info, read_json_file, Error, LoggerOptions};

use crate::{
    cache::{client::CacheState, types::CacheArcane},
    emit_error,
};

#[derive(Debug)]
pub struct ArcaneModule {
    path: PathBuf,
    items: Mutex<Vec<CacheArcane>>,
    client: Weak<CacheState>,
}
impl ArcaneModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Arcanes.json"),
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheArcane>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<Vec<CacheArcane>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:Arcane:load",
                    "Loaded Arcane items from cache",
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
}
