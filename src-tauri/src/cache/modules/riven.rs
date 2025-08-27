use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::*;

use crate::{
    cache::{
        client::CacheState,
        types::{CacheRiven, CacheRivenWFMAttribute, CacheRivenWeapon},
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
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<CacheRiven>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.data.lock().unwrap();
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
                return Ok(items.into_iter().find(|item| item.name == find_by.value))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| item.wfm_url_name == find_by.value))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| item.unique_name == find_by.value))
            }
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
                return Ok(items.into_iter().find(|item| item.effect == find_by.value))
            }
            FindByType::Url => {
                return Ok(items
                    .into_iter()
                    .find(|item| item.url_name == find_by.value))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| item.game_ref == find_by.value))
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }
}
