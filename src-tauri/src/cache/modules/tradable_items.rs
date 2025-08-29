use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use chrono::format;
use utils::{find_by, get_location, info, read_json_file, Error, LoggerOptions};

use crate::{
    cache::{client::CacheState, types::CacheTradableItem},
    enums::{FindBy, FindByType},
    DATABASE,
};

#[derive(Debug)]
pub struct TradableItemModule {
    path: PathBuf,
    items: Mutex<Vec<CacheTradableItem>>,
    client: Weak<CacheState>,
}

impl TradableItemModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/TradableItems.json"),
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub fn get_items(&self) -> Result<Vec<CacheTradableItem>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match read_json_file::<Vec<CacheTradableItem>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:TradableItem:load",
                    format!("Loaded {} tradable items from cache", items_lock.len()),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    pub fn get_by(&self, find_by: FindBy) -> Result<Option<CacheTradableItem>, Error> {
        let items = self.get_items()?;

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
            FindByType::Id => {
                return Ok(items.into_iter().find(|item| item.wfm_id == find_by.value))
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }
}
