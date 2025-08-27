use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::{find_by, get_location, info, read_json_file, Error, LoggerOptions};

use crate::cache::{client::CacheState, types::CacheResource};

#[derive(Debug)]
pub struct ResourceModule {
    path: PathBuf,
    items: Mutex<Vec<CacheResource>>,
    client: Weak<CacheState>,
}

impl ResourceModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Resources.json"),
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheResource>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<Vec<CacheResource>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:Resource:load",
                    "Loaded Resource items from cache",
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
}
