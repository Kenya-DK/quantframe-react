use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct RecipeModule {
    path: PathBuf,
    items: Mutex<Vec<CacheRecipe>>,
}

impl RecipeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Recipes.json"),
            items: Mutex::new(Vec::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheRecipe>>(&self.path) {
            Ok(mut items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:Recipe:load",
                    format!("Loaded {} recipes from cache", items_lock.len()),
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
    /// Get Item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    // pub fn get_by(&self, id: impl Into<String>) -> Result<CacheRecipe, Error> {
    //     let id: String = id.into();
    //     let lookup = self.item_lookup.lock().unwrap();
    //     if let Some(item) = lookup.get(&id) {
    //         Ok(item.clone())
    //     } else {
    //         Err(Error::new(
    //             "Cache:GetItemById",
    //             format!("Item not found for id '{}'", id),
    //             get_location!(),
    //         ))
    //     }
    //     Ok(CacheRecipe::default()) // Placeholder until Riven weapons are re-added to the cache
    // }
    /**
     * Creates a new `RecipeModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RecipeModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
