use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use utils::*;

use crate::cache::{
    client::CacheState,
    modules::LanguageModule,
    types::{CacheRiven, CacheRivenWFMAttribute, CacheRivenWeapon},
    AttributeMatch, CacheRivenUpgrade, RivenRollEvaluation, RollCriteria,
};

#[derive(Debug)]
pub struct RivenModule {
    path: PathBuf,

    // Lookup maps
    upgrade_lookup: Mutex<MultiKeyMap<CacheRivenUpgrade>>,
    weapon_lookup: Mutex<MultiKeyMap<CacheRivenWeapon>>,
    attribute_lookup: Mutex<MultiKeyMap<CacheRivenWFMAttribute>>,
}

impl RivenModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Riven.json"),
            upgrade_lookup: Mutex::new(MultiKeyMap::new()),
            weapon_lookup: Mutex::new(MultiKeyMap::new()),
            attribute_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }

    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<CacheRiven>(&self.path) {
            Ok(mut data) => {
                for item in data.attributes.iter_mut() {
                    item.full = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Full)
                        .unwrap_or(item.full.clone());
                    item.short = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Short)
                        .unwrap_or(item.short.clone());
                    item.name = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Name)
                        .unwrap_or(item.name.clone());
                    item.text = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Text)
                        .unwrap_or(item.text.clone());
                }
                for item in data.weapons.iter_mut() {
                    item.name = language
                        .translate(&item.unique_name, crate::cache::modules::LanguageKey::Name)
                        .unwrap_or(item.name.clone());
                }

                // Create lookup maps
                let mut upgrade_lookup = self.upgrade_lookup.lock().unwrap();
                for (key, val) in data.upgrade_types.iter() {
                    for a in val.iter() {
                        upgrade_lookup.insert_value(
                            a.clone(),
                            vec![
                                format!("{}|{}", key, a.wfm_url),
                                format!("{}|{}", key, a.modifier_tag),
                            ],
                        );
                    }
                }
                let mut weapon_lookup = self.weapon_lookup.lock().unwrap();
                for item in data.weapons.iter() {
                    weapon_lookup.insert_value(
                        item.clone(),
                        vec![
                            item.name.clone(),
                            item.wfm_url_name.clone(),
                            item.wfm_id.clone(),
                            item.unique_name.clone(),
                        ],
                    );
                }
                let mut attribute_lookup = self.attribute_lookup.lock().unwrap();
                for item in data.attributes.iter() {
                    attribute_lookup.insert_value(
                        item.clone(),
                        vec![
                            item.name.clone(),
                            item.url_name.clone(),
                            item.unique_name.clone(),
                        ],
                    );
                }

                info(
                    "Cache:Riven:load",
                    "Loaded Riven items from cache",
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

    /// Get Riven Upgrade by Riven Type and Tag  
    ///  # Arguments
    /// - `riven_type`: The type of the Riven (e.g., "/Lotus/Upgrades/Mods/Randomized/LotusPistolRandomModRare")
    /// - `tag`: The tag of the upgrade possibly wfm_url or modifier_tag
    ///
    pub fn get_upgrade_by(
        &self,
        riven_type: impl Into<String>,
        tag: impl Into<String>,
    ) -> Result<CacheRivenUpgrade, Error> {
        let riven_type: String = riven_type.into();
        let tag = tag.into();
        let upgrade_lookup = self.upgrade_lookup.lock().unwrap();
        if let Some(upgrade) = upgrade_lookup.get(&format!("{}|{}", riven_type, tag)) {
            Ok(upgrade.clone())
        } else {
            Err(Error::new(
                "Cache:Riven:GetUpgradeBy",
                format!(
                    "Riven upgrade not found for type '{}' and tag '{}'",
                    riven_type, tag
                ),
                get_location!(),
            ))
        }
    }

    /// Get Weapon
    ///  # Arguments
    /// - `weapon_id`: The weapon id to lookup by (name, wfm_url, wfm_id, unique_name)
    ///
    pub fn get_weapon_by(&self, weapon_id: impl Into<String>) -> Result<CacheRivenWeapon, Error> {
        let weapon_id: String = weapon_id.into();
        let weapon_lookup = self.weapon_lookup.lock().unwrap();
        if let Some(weapon) = weapon_lookup.get(&weapon_id) {
            Ok(weapon.clone())
        } else {
            Err(Error::new(
                "Cache:Riven:GetWeaponBy",
                format!("Riven weapon not found for id '{}'", weapon_id),
                get_location!(),
            ))
        }
    }

    /// Get Attribute
    ///  # Arguments
    /// - `attribute_id`: The attribute id to lookup by (name, url_name, unique_name)
    ///
    pub fn get_attribute_by(
        &self,
        attribute_id: impl Into<String>,
    ) -> Result<CacheRivenWFMAttribute, Error> {
        let attribute_id: String = attribute_id.into();
        let attribute_lookup = self.attribute_lookup.lock().unwrap();
        if let Some(attribute) = attribute_lookup.get(&attribute_id) {
            Ok(attribute.clone())
        } else {
            Err(Error::new(
                "Cache:Riven:GetAttributeBy",
                format!("Riven attribute not found for id '{}'", attribute_id),
                get_location!(),
            ))
        }
    }

    /* -------------------------------------------------------------
        Vector Functions
    ------------------------------------------------------------- */

    /// Get all weapons
    /// Returns a vector of all Riven weapons in the cache.
    pub fn get_all_weapons(&self) -> Result<Vec<CacheRivenWeapon>, Error> {
        let weapon_lookup = self
            .weapon_lookup
            .lock()
            .expect("Failed to lock weapon lookup mutex");
        Ok(weapon_lookup.get_all_values())
    }

    /// Get all attributes
    /// Returns a vector of all Riven attributes in the cache.
    pub fn get_all_attributes(&self) -> Result<Vec<CacheRivenWFMAttribute>, Error> {
        let attribute_lookup = self
            .attribute_lookup
            .lock()
            .expect("Failed to lock attribute lookup mutex");
        Ok(attribute_lookup.get_all_values())
    }

    /* -------------------------------------------------------------
        Helper Functions
    ------------------------------------------------------------- */
    pub fn fill_roll_evaluation(
        &self,
        raw: impl Into<String>,
        stats: Vec<(String, bool)>,
    ) -> Result<RivenRollEvaluation, Error> {
        let weapon = self.get_weapon_by(raw.into())?;

        let god_roll = match weapon.god_roll {
            Some(gr) => gr,
            None => return Ok(RivenRollEvaluation::default()),
        };

        let mut summary = RivenRollEvaluation::default();

        let stat_lookup: HashSet<(&str, bool)> =
            stats.iter().map(|(s, p)| (s.as_str(), *p)).collect();

        let has_stat =
            |name: &str, positive: bool| -> bool { stat_lookup.contains(&(name, positive)) };

        let resolve_attr = |attr: &str, positive: bool| -> AttributeMatch {
            let unique_name = format!("WF_Special/RivenAttributes/{}", attr);
            match self.get_attribute_by(&unique_name) {
                Ok(att) => AttributeMatch::new(&att.short, has_stat(&att.url_name, positive)),
                Err(e) => AttributeMatch::new(e.to_string(), false),
            }
        };

        // Negative attributes
        for bad in &god_roll.negative_attributes {
            let att = resolve_attr(bad, false);
            summary.add_negative_attribute(att.label, att.matches);
        }

        // Good rolls
        for roll in god_roll.good_rolls {
            let mut roll_summary = RollCriteria::default();

            roll_summary.required = roll
                .required
                .iter()
                .map(|r| resolve_attr(r, true))
                .collect();

            roll_summary.optional = roll
                .optional
                .iter()
                .map(|o| resolve_attr(o, true))
                .collect();

            summary.add_valid_roll(roll_summary);
        }

        Ok(summary)
    }

    /**
     * Creates a new `RivenModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            upgrade_lookup: Mutex::new(old.upgrade_lookup.lock().unwrap().clone()),
            weapon_lookup: Mutex::new(old.weapon_lookup.lock().unwrap().clone()),
            attribute_lookup: Mutex::new(old.attribute_lookup.lock().unwrap().clone()),
        })
    }
}
