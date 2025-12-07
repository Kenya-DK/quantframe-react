use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use regex::Regex;
use utils::*;

use crate::{
    cache::{
        client::CacheState,
        modules::LanguageModule,
        types::{CacheRiven, CacheRivenWFMAttribute, CacheRivenWeapon},
        CacheRivenUpgrade,
    },
    enums::{FindBy, FindByType},
};

#[derive(Debug)]
pub struct RivenModule {
    path: PathBuf,
    data: Mutex<CacheRiven>,
}

impl RivenModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Riven.json"),
            data: Mutex::new(CacheRiven::new()),
        })
    }
    pub fn get_items(&self) -> Result<CacheRiven, Error> {
        let data = self
            .data
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(data)
    }
    pub fn get_upgrade_types(&self) -> Result<Vec<CacheRivenUpgrade>, Error> {
        let data = self
            .data
            .lock()
            .expect("Failed to lock items mutex")
            .available_upgrade_types
            .clone();
        Ok(data)
    }

    pub fn load(&self, language: &LanguageModule) -> Result<(), Error> {
        match read_json_file_optional::<CacheRiven>(&self.path) {
            Ok(mut data) => {
                let mut items_lock = self.data.lock().unwrap();

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

                // Get All value types for riven upgrades
                let mut all_upgrade_types: Vec<CacheRivenUpgrade> = Vec::new();
                let mut dict_url: HashMap<String, CacheRivenUpgrade> = HashMap::new();
                for upgrades in data.upgrade_types.values() {
                    for upgrade in upgrades {
                        if !all_upgrade_types
                            .iter()
                            .any(|u| u.modifier_tag == upgrade.modifier_tag)
                        {
                            all_upgrade_types.push(upgrade.clone());
                            if !dict_url.contains_key(&upgrade.wfm_url) {
                                dict_url.insert(upgrade.wfm_url.clone(), upgrade.clone());
                            }
                        }
                    }
                }

                data.available_upgrade_types = all_upgrade_types;
                *items_lock = data;
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

    pub fn get_riven_by(&self, find_by: FindBy) -> Result<Option<CacheRivenWeapon>, Error> {
        let items = self.get_items()?.weapons;

        match find_by.find_by {
            FindByType::Name => {
                return Ok(items.into_iter().find(|item| find_by.is_match(&item.name)))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.wfm_url_name)))
            }
            FindByType::Id => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.wfm_id)))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.unique_name)))
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }

    pub fn get_riven_upgrade_by(
        &self,
        mut find_by: FindBy,
    ) -> Result<Option<CacheRivenUpgrade>, Error> {
        let re = Regex::new(r"<.*?>").unwrap();
        let upgrade_types = if let FindByType::Custom(ref s) = find_by.find_by {
            let operation = s.split('|').next().unwrap_or("").to_string();
            let by = s.split('|').nth(1).unwrap_or("");
            println!("Operation: {}, By: {}", operation, by);
            if operation == "upgrade_type" {
                // Spilt by '|' to get the upgrade type and tag
                let upgrade_type = find_by.value.split('|').next().unwrap_or("").to_string();
                let tag = find_by.value.split('|').nth(1).unwrap_or("");
                println!("Upgrade Type: {}, Tag: {}", upgrade_type, tag);
                find_by = match by {
                    "name" => FindBy::new(FindByType::Name, tag),
                    "url" => FindBy::new(FindByType::Url, tag),
                    "unique_name" => FindBy::new(FindByType::UniqueName, tag),
                    _ => FindBy::new(FindByType::UniqueName, tag),
                };
                if let Some(upgrade_types) = self.get_items()?.upgrade_types.get(&upgrade_type) {
                    upgrade_types.clone()
                } else {
                    vec![]
                }
            } else {
                self.get_upgrade_types()?
            }
        } else {
            self.get_upgrade_types()?
        };

        match find_by.find_by {
            FindByType::Name => {
                return Ok(upgrade_types.into_iter().find(|item| {
                    find_by.is_match(&re.replace_all(&item.short_string, "").to_string())
                }))
            }
            FindByType::Url => {
                return Ok(upgrade_types
                    .into_iter()
                    .find(|item| find_by.is_match(&item.wfm_url)))
            }
            FindByType::UniqueName => {
                return Ok(upgrade_types
                    .into_iter()
                    .find(|item| find_by.is_match(&item.modifier_tag)))
            }
            FindByType::Custom(ref s) => match s.as_str() {
                _ => Err(Error::new(
                    "Cache:TradableItem:GetBy",
                    "Unsupported FindBy custom type",
                    get_location!(),
                )),
            },
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }

    pub fn get_riven_attribute_by(
        &self,
        find_by: FindBy,
    ) -> Result<Option<CacheRivenWFMAttribute>, Error> {
        let items = self.get_items()?.attributes;
        match find_by.find_by {
            FindByType::Name => {
                return Ok(items.into_iter().find(|item| find_by.is_match(&item.name)))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.url_name)))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.unique_name)))
            }
            FindByType::Custom(ref s) => {
                if s.starts_with("upgrade") {
                    let upgrade = self.get_riven_upgrade_by(find_by.clone())?;
                    if let Some(upgrade) = upgrade {
                        let attribute = items
                            .into_iter()
                            .find(|item| item.url_name == upgrade.wfm_url);
                        return Ok(attribute);
                    }
                    return Ok(None);
                }
                return Err(Error::new(
                    "Cache:TradableItem:GetBy",
                    format!("Unsupported FindBy custom type for Riven Attribute: {}", s),
                    get_location!(),
                ));
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }

    /**
     * Creates a new `RivenModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenModule) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            data: Mutex::new(old.data.lock().unwrap().clone()),
        })
    }
}
