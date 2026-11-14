use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions, MultiKeyMap};

use crate::cache::{modules::LanguageModule, *};

#[derive(Debug)]
pub struct ArcaneModule {
    path: PathBuf,
    lookup: Mutex<MultiKeyMap<CacheArcane>>,
}
impl ArcaneModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Arcanes.json"),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<Vec<CacheArcane>>(&self.path) {
            Ok(mut items) => {
                for item in items.iter_mut() {
                    item.base.translate(&language);
                }
                let item_count = items.len();
                self.set_items(items);
                info(
                    "Cache:Arcane:load",
                    format!("Loaded {} Arcane items", item_count),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }

    fn set_items(&self, mut items: Vec<CacheArcane>) {
        // Shared names can belong to internal ability listeners as well as
        // tradable arcanes. MultiKeyMap keeps the first value for a key.
        items.sort_by_key(|item| !item.base.is_tradeable);
        let mut lookup = MultiKeyMap::new();
        for item in items {
            let mut keys = vec![item.base.unique_name.clone(), item.base.name.clone()];
            if let Some(wfm_url) = &item.base.wfm_url {
                keys.push(wfm_url.clone());
            }
            keys.extend(item.base.previous_names.iter().cloned());
            lookup.insert_value(item, keys);
        }
        *self.lookup.lock().unwrap() = lookup;
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */

    /// Lookup by any indexed key.
    /// # Arguments
    /// - `id`: The identifier to search for (name, unique_name, wfm_url)
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheArcane, Error> {
        let id = id.into();

        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "ArcaneModule:GetBy",
                    format!("Arcane item not found for id '{}'", id),
                    get_location!(),
                )
            })
    }
    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */
    pub fn get_all_items(&self) -> Result<Vec<CacheArcane>, Error> {
        let lookup = self.lookup.lock().unwrap();
        Ok(lookup.get_all_values())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const PREFIX: &str = "/Lotus/Upgrades/CosmeticEnhancers/Utility/";

    fn steadfast_items() -> Vec<CacheArcane> {
        let mut items = (1..=4)
            .map(|index| {
                serde_json::from_value(json!({
                    "uniqueName": format!("{PREFIX}NoCostCastChanceAbility{index}Listener"),
                    "name": "Arcane Steadfast",
                    "fusionLimit": 5
                }))
                .unwrap()
            })
            .collect::<Vec<_>>();
        items.push(
            serde_json::from_value(json!({
                "uniqueName": format!("{PREFIX}NoCostCastChanceOnCast"),
                "name": "Arcane Steadfast",
                "fusionLimit": 5,
                "isTradeable": true,
                "wfmUrl": "arcane_steadfast",
                "previousNames": ["Steadfast"]
            }))
            .unwrap(),
        );
        items
    }

    fn module() -> ArcaneModule {
        ArcaneModule {
            path: PathBuf::new(),
            lookup: Mutex::new(MultiKeyMap::new()),
        }
    }

    #[test]
    fn tradable_arcane_wins_shared_name_in_either_catalog_order() {
        let module = module();
        let mut items = steadfast_items();
        for _ in 0..2 {
            module.set_items(items.clone());
            for key in [
                "Arcane Steadfast",
                "ARCANE STEADFAST",
                "arcane_steadfast",
                "Steadfast",
            ] {
                let item = module.get_by(key).unwrap();
                assert_eq!(
                    item.base.unique_name,
                    format!("{PREFIX}NoCostCastChanceOnCast")
                );
                assert!(item.base.is_tradeable);
                assert_eq!(item.fusion_limit, 5);
            }
            assert_eq!(module.get_all_items().unwrap().len(), 5);
            for index in 1..=4 {
                let key = format!("{PREFIX}NoCostCastChanceAbility{index}Listener");
                assert_eq!(module.get_by(&key).unwrap().base.unique_name, key);
            }
            items.reverse();
        }
    }

    #[test]
    fn reloading_catalog_rebuilds_shared_name_lookup() {
        let module = module();
        module.set_items(vec![steadfast_items().remove(0)]);
        assert!(!module.get_by("Arcane Steadfast").unwrap().base.is_tradeable);

        module.set_items(steadfast_items());
        assert!(module.get_by("Arcane Steadfast").unwrap().base.is_tradeable);
        assert_eq!(module.get_all_items().unwrap().len(), 5);
    }
}
