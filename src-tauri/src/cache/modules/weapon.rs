use std::sync::{Arc, Mutex};

use utils::{get_location, Error, MultiKeyMap};

use crate::cache::*;

#[derive(Debug)]
pub struct WeaponModule {
    lookup: Mutex<MultiKeyMap<CacheWeaponBase>>,
}

impl WeaponModule {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }

    pub fn load(&self, cache: &CacheState) -> Result<(), Error> {
        let mut lookup = self.lookup.lock().unwrap();

        // Helper closure to reduce duplication
        let mut insert_weapon = |weapon: &CacheWeaponBase| {
            let mut keys = vec![weapon.name.clone(), weapon.unique_name.clone()];

            if !weapon.wfm_riven_url.is_empty() {
                keys.push(weapon.wfm_riven_url.clone());
            }
            if !weapon.wfm_riven_id.is_empty() {
                keys.push(weapon.wfm_riven_id.clone());
            }

            lookup.insert_value(weapon.clone(), keys);
        };

        for weapon in cache.primary().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in cache.secondary().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in cache.melee().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in cache.archgun().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in cache.archmelee().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in cache.sentinel_weapon().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        Ok(())
    }

    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */

    /// Lookup by any indexed key.
    /// # Arguments
    /// - `id`: The identifier to search for (name, unique_name, wfm_url)
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheWeaponBase, Error> {
        let id = id.into();
        println!("Looking up weapon by id: {}", id);
        self.lookup
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    "WeaponModule:GetBy",
                    format!("Weapon not found for id '{}'", id),
                    get_location!(),
                )
            })
    }

    /// Get weapons by family.
    pub fn get_weapons_by_family(
        &self,
        family: impl Into<String>,
    ) -> Result<Vec<CacheWeaponBase>, Error> {
        let family = family.into();

        let items = self
            .lookup
            .lock()
            .unwrap()
            .get_all_values()
            .into_iter()
            .filter(|weapon| weapon.family == family)
            .collect::<Vec<_>>();

        if items.is_empty() {
            Err(Error::new(
                "WeaponModule:GetWeaponsByFamily",
                format!("No weapons found for family '{}'", family),
                get_location!(),
            ))
        } else {
            Ok(items)
        }
    }

    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */

    pub fn get_all_items(&self) -> Result<Vec<CacheWeaponBase>, Error> {
        Ok(self.lookup.lock().unwrap().get_all_values())
    }
}
