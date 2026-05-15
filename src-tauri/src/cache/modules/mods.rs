use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ModModule {
    path: PathBuf,
    items: Mutex<Vec<CacheMod>>,

    upgrade_entries_lookup: Mutex<MultiKeyMap<CacheUpgradeEntry>>,
}

impl ModModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Mods.json"),
            items: Mutex::new(Vec::new()),
            upgrade_entries_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMod>>(&self.path) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                let mut upgrade_entries_lookup = self.upgrade_entries_lookup.lock().unwrap();
                for item in items.iter() {
                    if !item.upgrade_entries.is_empty() {
                        for entry in item.upgrade_entries.iter() {
                            upgrade_entries_lookup.insert_value(
                                entry.clone(),
                                vec![
                                    format!("{}|{}", item.base.unique_name, entry.wfm_url),
                                    format!("{}|{}", item.base.unique_name, entry.unique_name),
                                ],
                            );
                        }
                    }
                }
                *items_lock = items.clone();
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
    /// Get Riven Stat Tag by Riven Type and Tag
    ///  # Arguments
    /// - `riven_type`: The type of the Riven (e.g., "/Lotus/Upgrades/Mods/Randomized/LotusPistolRandomModRare")
    /// - `tag`: The tag of the upgrade possibly wfm_url or modifier_tag
    ///
    pub fn get_stat_tag_by(
        &self,
        riven_type: impl Into<String>,
        tag: impl Into<String>,
    ) -> Result<CacheUpgradeEntry, Error> {
        let riven_type: String = riven_type.into();
        let tag = tag.into();
        let lookup = self.upgrade_entries_lookup.lock().unwrap();
        if let Some(upgrade) = lookup.get(&format!("{}|{}", riven_type, tag)) {
            Ok(upgrade.clone())
        } else {
            Err(Error::new(
                "Cache:Mod:GetStatTagBy",
                format!(
                    "Riven upgrade not found for type '{}' and tag '{}'",
                    riven_type, tag
                ),
                get_location!(),
            ))
        }
    }
    /**
     * Creates a new `ModModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ModModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            upgrade_entries_lookup: Mutex::new(old.upgrade_entries_lookup.lock().unwrap().clone()),
        })
    }
}
