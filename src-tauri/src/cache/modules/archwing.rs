use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::{find_by, get_location, info, read_json_file, Error, LoggerOptions};

use crate::cache::*;

#[derive(Debug)]
pub struct ArchwingModule {
    path: PathBuf,
    items: Mutex<Vec<CacheArchwing>>,
    components: Mutex<Vec<CacheItemComponent>>,
    client: Weak<CacheState>,
}

impl ArchwingModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Archwing.json"),
            items: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheArchwing>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<Vec<CacheArchwing>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                let mut components_lock = self.components.lock().unwrap();
                info(
                    "Cache:Archwing:load",
                    format!("Loaded {} Archwing items", items.len()),
                    &LoggerOptions::default(),
                );
                *items_lock = items.clone();
                for mut item in items {
                    if item.components.is_none() {
                        continue;
                    }
                    components_lock.append(&mut item.components.take().unwrap());
                }
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    pub fn collect_all_items(&self) -> Vec<CacheItemBase> {
        let items_lock = self.items.lock().unwrap();
        let components_lock = self.components.lock().unwrap();
        let mut items: Vec<CacheItemBase> = Vec::new();
        items.append(
            &mut items_lock
                .iter()
                .map(|item| item.convert_to_base_item())
                .collect(),
        );
        items.append(
            &mut components_lock
                .iter()
                .map(|item| item.convert_to_base_item())
                .collect(),
        );
        items
    }
    /**
     * Creates a new `ArchwingModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ArchwingModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            client: Arc::downgrade(&client),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            components: Mutex::new(old.components.lock().unwrap().clone()),
        })
    }
}
