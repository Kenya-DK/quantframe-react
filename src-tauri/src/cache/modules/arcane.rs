use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ArcaneModule {
    path: PathBuf,
    items: Mutex<Vec<CacheArcane>>,
    components: Mutex<Vec<CacheItemComponent>>,
}
impl ArcaneModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Arcanes.json"),
            items: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheArcane>>(&self.path) {
            Ok(mut items) => {
                for item in items.iter_mut() {
                    item.name = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Name)
                        .unwrap_or(item.name.clone());
                }

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
    pub fn from_existing(old: &ArcaneModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            components: Mutex::new(old.components.lock().unwrap().clone()),
        })
    }
}
