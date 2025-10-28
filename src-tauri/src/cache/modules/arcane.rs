use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};
use utils::{get_location, info, read_json_file, Error, LoggerOptions};

use crate::cache::*;

#[derive(Debug)]
pub struct ArcaneModule {
    path: PathBuf,
    items: Mutex<Vec<CacheArcane>>,
    components: Mutex<Vec<CacheItemComponent>>,
    client: Weak<CacheState>,
}
impl ArcaneModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Arcanes.json"),
            items: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<Vec<CacheArcane>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                let mut components_lock = self.components.lock().unwrap();
                info(
                    "Cache:Arcane:load",
                    format!("Loaded {} Arcane items", items.len()),
                    &LoggerOptions::default(),
                );
                *items_lock = items.clone();
                for mut item in items {
                    components_lock.append(&mut item.components);
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
     * Creates a new `ArcaneModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ArcaneModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            client: Arc::downgrade(&client),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            components: Mutex::new(old.components.lock().unwrap().clone()),
        })
    }
}
