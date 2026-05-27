use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ModModule {
    path: PathBuf,

    upgrade_entries_lookup: Mutex<MultiKeyMap<CacheUpgradeEntry>>,
    lookup: Mutex<MultiKeyMap<CacheMod>>,
}

impl ModModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Mods.json"),
            upgrade_entries_lookup: Mutex::new(MultiKeyMap::new()),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheMod>>(&self.path) {
            Ok(mut items) => {
                let mut upgrade_entries_lookup = self.upgrade_entries_lookup.lock().unwrap();
                let mut lookup = self.lookup.lock().unwrap();
                for item in items.iter_mut() {
                    item.base.translate(&language);
                    let mut keys = vec![item.base.unique_name.clone(), item.base.name.clone()];

                    if let Some(wfm_url) = &item.base.wfm_url {
                        keys.push(wfm_url.clone());
                    }

                    keys.extend(item.base.previous_names.iter().cloned());

                    lookup.insert_value(item.clone(), keys);

                    // Create upgrade entry lookups for each upgrade entry in the mod
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
                info(
                    "Cache:Mod:load",
                    format!("Loaded {} Mod items", lookup.len()),
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
    /// - `id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheMod, Error> {
        let id: String = id.into();
        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "ModModule:GetBy",
                    format!("Mod not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
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
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheMod>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}
