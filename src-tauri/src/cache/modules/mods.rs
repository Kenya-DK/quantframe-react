use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ModModule {
    path: PathBuf,
    items: Mutex<MultiKeyMap<CacheMod>>,
}

impl ModModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Mods.json"),
            items: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMod>>(&self.path) {
            Ok(mut items) => {
                let mut items_lock = self.items.lock().unwrap();
                for item in items.iter_mut() {
                    item.name = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Name)
                        .unwrap_or(item.name.clone());

                    items_lock.insert_value(
                        item.clone(),
                        vec![format!("{}", item.unique_name), format!("{}", item.name)],
                    );
                }
                info(
                    "Cache:Mod:load",
                    format!("Loaded {} Mod items", items_lock.len()),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */

    /// Get Mod by unique name or name
    ///  # Arguments
    /// - `id`: The unique name or name of the mod to lookup
    ///
    /// Returns the `CacheMod` matching the provided `id`.
    pub fn get(&self, id: impl Into<String>) -> Result<CacheMod, Error> {
        let unique_name: String = id.into();
        let items_lock = self.items.lock().unwrap();
        if let Some(item) = items_lock.get(&unique_name) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:Mod:Get",
                format!("Mod with unique name '{}' not found", unique_name),
                get_location!(),
            ))
        }
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    /// Get all mods as a vector
    pub fn collect_all_items(&self) -> Vec<CacheItemBase> {
        let items_lock = self.items.lock().unwrap();
        items_lock
            .get_all_values()
            .iter()
            .map(|item| item.convert_to_base_item())
            .collect()
    }
    /**
     * Creates a new `ModModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ModModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
