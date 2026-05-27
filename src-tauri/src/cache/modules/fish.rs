use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct FishModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheFish>>,
}

impl FishModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Fish.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheFish>>(&self.path) {
            Ok(mut items) => {
                let mut lookup = self.lookup.lock().unwrap();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let mut keys = vec![item.base.unique_name.clone()];

                    let variant = item
                        .base
                        .sub_type
                        .as_ref()
                        .and_then(|sub_type| sub_type.variant.as_deref())
                        .unwrap_or("");
                    if let Some(wfm_url) = &item.base.wfm_url {
                        if !variant.is_empty() {
                            keys.push(format!("{wfm_url}|{variant}"));
                        } else {
                            keys.push(wfm_url.clone());
                        }
                    }

                    let mut push_name_key = |name: &str| {
                        let key = if !variant.is_empty() {
                            format!("{}|{}", name, variant)
                        } else {
                            name.to_string()
                        };
                        if lookup.get(&key).is_none()
                            && !keys.iter().any(|existing| existing == &key)
                        {
                            keys.push(key);
                        }
                    };

                    push_name_key(&item.base.name);
                    for prev_name in &item.base.previous_names {
                        push_name_key(prev_name);
                    }

                    lookup.insert_value(item.clone(), keys);
                }
                info(
                    "Cache:Fish:load",
                    format!("Loaded {} Fish items", lookup.len()),
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
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheFish, Error> {
        let id = id.into();

        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "FishModule:GetBy",
                    format!("Fish not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheFish>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
