use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct RecipeModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheRecipe>>,
}

impl RecipeModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Recipes.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheRecipe>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                *lookup = MultiKeyMap::new();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let mut keys = vec![item.base.name.clone(), item.base.unique_name.clone()];
                    if !item.override_unique_name.is_empty() {
                        keys.push(item.override_unique_name.clone());
                    }
                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Recipe:load",
                    format!("Loaded {} recipes from cache", lookup.len()),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Functions
    ------------------------------------------------------------- */
    pub fn can_craft(
        &self,
        items: &[CacheItemBase],
        tradeable_only: bool,
        from_recipe_only: bool,
    ) -> Result<Vec<CacheRecipe>, Error> {
        let recipes = self.get_all_items()?;
        let mut buildable_recipes: Vec<CacheRecipe> = Vec::new();
        for recipe in recipes {
            if recipe.can_craft(tradeable_only, from_recipe_only, items) {
                buildable_recipes.push(recipe);
            }
        }
        Ok(buildable_recipes)
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    // Get Item by various identifiers
    //  # Arguments
    // - `id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    //
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheRecipe, Error> {
        let id: String = id.into();
        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "Cache:Recipe:get_by",
                    format!("Recipe not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheRecipe>, Error> {
        Ok(self.lookup.lock().unwrap().get_all_values())
    }
}
