use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use regex::Regex;
use utils::*;

use crate::{
    cache::{
        client::CacheState,
        types::{CacheRiven, CacheRivenWFMAttribute, CacheRivenWeapon},
        CacheRivenUpgrade,
    },
    enums::{FindBy, FindByType},
};

#[derive(Debug)]
pub struct RivenModule {
    path: PathBuf,
    data: Mutex<CacheRiven>,
    client: Weak<CacheState>,
}

impl RivenModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/Riven.json"),
            data: Mutex::new(CacheRiven::new()),
            client: Arc::downgrade(&client),
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

    pub fn load(&self) -> Result<(), Error> {
        let _client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file_optional::<CacheRiven>(&self.path) {
            Ok(mut items) => {
                let mut items_lock = self.data.lock().unwrap();

                // Get All value types for riven upgrades
                let mut all_upgrade_types: Vec<CacheRivenUpgrade> = Vec::new();
                for upgrades in items.upgrade_types.values() {
                    for upgrade in upgrades {
                        if !all_upgrade_types
                            .iter()
                            .any(|u| u.modifier_tag == upgrade.modifier_tag)
                        {
                            all_upgrade_types.push(upgrade.clone());
                        }
                    }
                }

                items.available_upgrade_types = all_upgrade_types;
                *items_lock = items;
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
        find_by: FindBy,
    ) -> Result<Option<CacheRivenUpgrade>, Error> {
        let re = Regex::new(r"<.*?>").unwrap();
        let items = self.get_upgrade_types()?;
        match find_by.find_by {
            FindByType::Name => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.modifier_tag)))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.wfm_url)))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.modifier_tag)))
            }
            FindByType::Custom(ref s) => match s.as_str() {
                "short_string" => {
                    return Ok(items.into_iter().find(|item| {
                        find_by.is_match(&re.replace_all(&item.short_string, "").to_string())
                    }));
                }
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
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.effect)))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.url_name)))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.game_ref)))
            }
            FindByType::Custom(s) => match s.as_str() {
                "upgrades|short_string" => {
                    let upgrade = self.get_riven_upgrade_by(FindBy::new(
                        FindByType::Custom(String::from("short_string")),
                        &find_by.value,
                    ))?;
                    if upgrade.is_none() {
                        return Ok(None);
                    }
                    self.get_riven_attribute_by(FindBy::new(
                        FindByType::Url,
                        &upgrade.unwrap().wfm_url,
                    ))
                }
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
    /**
     * Creates a new `RivenModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            client: Arc::downgrade(&client),
            data: Mutex::new(old.data.lock().unwrap().clone()),
        })
    }
}
