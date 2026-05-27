use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct BundleModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheBundle>>,
}

impl BundleModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Bundle.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheBundle>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let mut keys = vec![item.base.unique_name.clone(), item.base.name.clone()];

                    if let Some(wfm_url) = &item.base.wfm_url {
                        keys.push(wfm_url.clone());
                    }

                    keys.extend(item.base.previous_names.iter().cloned());

                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Bundle:load",
                    format!("Loaded {} Bundle items", lookup.len()),
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

    /// Lookup by any indexed key.
    /// # Arguments
    /// - `id`: The identifier to search for (name, unique_name, wfm_url)
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheBundle, Error> {
        let id = id.into();

        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "BundleModule:GetBy",
                    format!("Bundle not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
}
