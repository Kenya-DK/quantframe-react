use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct GearModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheGear>>,
}

impl GearModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Gear.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheGear>>(&self.path) {
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
                    "Cache:Gear:load",
                    format!("Loaded {} Gear items", lookup.len()),
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
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheGear, Error> {
        let id = id.into();

        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "GearModule:GetBy",
                    format!("Gear not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheGear>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
