use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct SkinModule {
    path: PathBuf,
    items: Mutex<Vec<CacheSkin>>,
    components: Mutex<Vec<CacheItemComponent>>,
}

impl SkinModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Skins.json"),
            items: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheSkin>>(&self.path) {
            Ok(mut items) => {
                for item in items.iter_mut() {
                    item.name = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Name)
                        .unwrap_or(item.name.clone());
                }
                let mut items_lock = self.items.lock().unwrap();
                let mut components_lock = self.components.lock().unwrap();
                info(
                    "Cache:Skin:load",
                    format!("Loaded {} Skin items", items.len()),
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
     * Creates a new `SkinModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &SkinModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            components: Mutex::new(old.components.lock().unwrap().clone()),
        })
    }
}
