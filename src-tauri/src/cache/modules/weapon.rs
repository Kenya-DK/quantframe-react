use std::sync::{Arc, Mutex, Weak};

use utils::{get_location, Error, MultiKeyMap};

use crate::cache::*;

#[derive(Debug)]
pub struct WeaponModule {
    client: Weak<CacheState>,
    lookup: Mutex<MultiKeyMap<CacheWeaponBase>>,
}

impl WeaponModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }

    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        let mut lookup = self.lookup.lock().unwrap();

        // Helper closure to reduce duplication
        let mut insert_weapon = |weapon: &CacheWeaponBase| {
            let mut keys = vec![
                format!("Name:{}", weapon.name),
                format!("Unique:{}", weapon.unique_name),
            ];

            if !weapon.wfm_riven_url.is_empty() {
                keys.push(format!("Wfm:{}", weapon.wfm_riven_url));
            }

            lookup.insert_value(weapon.clone(), keys);
        };

        for weapon in client.primary().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in client.secondary().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in client.melee().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in client.archgun().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in client.archmelee().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        for weapon in client.sentinel_weapon().get_all_items()? {
            insert_weapon(&weapon.base);
        }

        Ok(())
    }

    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */

    /// Lookup by any indexed key.
    /// # Arguments
    /// - `id`: The identifier to search for (name, unique_name, wfm_riven_url)
    /// Note: Use
    ///  - `Name:WeaponName` for name lookup
    ///  - `Unique:UniqueName` for unique name lookup
    ///  - `Wfm:WfmRivenUrl` for WFM Riven URL lookup
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheWeaponBase, Error> {
        let id = id.into();

        const VALID_PREFIXES: [&str; 3] = ["Name:", "Unique:", "Wfm:"];

        if !VALID_PREFIXES.iter().any(|prefix| id.starts_with(prefix)) {
            return Err(Error::new(
                "WeaponModule:GetBy",
                format!(
                    "Invalid id '{}'. Expected prefixes: {}",
                    id,
                    VALID_PREFIXES.join(", ")
                ),
                get_location!(),
            ));
        }

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

    /// Clone from existing module while sharing client state.
    pub fn from_existing(old: &WeaponModule) -> Arc<Self> {
        Arc::new(Self {
            client: old.client.clone(),
            lookup: Mutex::new(old.lookup.lock().unwrap().clone()),
        })
    }
}
