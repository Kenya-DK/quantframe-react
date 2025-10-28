use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use utils::{get_location, info, read_json_file, Error, LoggerOptions};

use crate::{
    cache::{client::CacheState, types::CacheTradableItem},
    enums::{FindBy, FindByType},
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
            FindByType::Id => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.wfm_id)))
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }
    pub fn get_dict(
        &self,
        find_by: FindByType,
    ) -> Result<HashMap<String, CacheTradableItem>, Error> {
        let items = self.get_items()?;

        match find_by {
            FindByType::Name => {
                return Ok(items
                    .iter()
                    .map(|x| (x.name.clone(), x.clone()))
                    .collect::<HashMap<String, CacheTradableItem>>())
            }
            FindByType::Url => {
                return Ok(items
                    .iter()
                    .map(|x| (x.wfm_url_name.clone(), x.clone()))
                    .collect::<HashMap<String, CacheTradableItem>>())
            }
            FindByType::UniqueName => {
                return Ok(items
                    .iter()
                    .map(|x| (x.unique_name.clone(), x.clone()))
                    .collect::<HashMap<String, CacheTradableItem>>())
            }
            FindByType::Id => {
                return Ok(items
                    .iter()
                    .map(|x| (x.wfm_id.clone(), x.clone()))
                    .collect::<HashMap<String, CacheTradableItem>>())
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }
    /**
     * Creates a new `TradableItemModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &TradableItemModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            client: Arc::downgrade(&client),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
