use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct SyndicateModule {
    path: PathBuf,
    // Lookup maps
    syndicate_lookup: Mutex<MultiKeyMap<CacheSyndicate>>,
}

impl SyndicateModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("static/Syndicates.json"),
            syndicate_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, _: &LanguageModule) -> Result<(), Error> {
        info(
            "Cache:Syndicate:load",
            "Loading Syndicates... (skipped, language-agnostic)",
            &LoggerOptions::default(),
        );
        return Ok(()); // Syndicates are language-agnostic, so we can skip loading here
        match read_json_file_optional::<HashMap<String, CacheSyndicate>>(&self.path) {
            Ok(items) => {
                let mut syndicate_lookup_lock = self.syndicate_lookup.lock().unwrap();

                for (key, item) in items.into_iter() {
                    syndicate_lookup_lock.insert_value(
                        item.clone(),
                        vec![key.clone(), item.name.clone(), item.unique_name.clone()],
                    );
                }

                info(
                    "Cache:Syndicate:load",
                    format!("Loaded {} Syndicates", syndicate_lookup_lock.len()),
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
    /// Get a syndicate by various identifiers
    ///  # Arguments
    /// - `syndicate_id`: The identifier to search for (name, unique name, or id)
    ///
    pub fn get_by(&self, syndicate_id: impl Into<String>) -> Result<CacheSyndicate, Error> {
        let syndicate_id: String = syndicate_id.into();
        let syndicate_lookup = self.syndicate_lookup.lock().unwrap();
        if let Some(syndicate) = syndicate_lookup.get(&syndicate_id) {
            Ok(syndicate.clone())
        } else {
            Err(Error::new(
                "Cache:Syndicate:GetBy",
                format!("Syndicate not found for id '{}'", syndicate_id),
                get_location!(),
            ))
        }
    }
    /**
     * Creates a new `SyndicateModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &SyndicateModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            syndicate_lookup: Mutex::new(old.syndicate_lookup.lock().unwrap().clone()),
        })
    }
}
