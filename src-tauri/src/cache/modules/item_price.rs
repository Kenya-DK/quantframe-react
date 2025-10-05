use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use crate::{
    app::client,
    cache::{client::CacheState, types::item_price_info::ItemPriceInfo},
    emit_startup,
    enums::FindBy,
    utils::ErrorFromExt,
};
use entity::dto::SubType;
use qf_api::Client as QFClient;
use utils::{find_by, get_location, info, read_json_file, Error, LoggerOptions};
use wf_market::endpoints::order;

#[derive(Debug)]
pub struct ItemPriceModule {
    path: PathBuf,
    items: Mutex<Vec<ItemPriceInfo>>,
    client: Weak<CacheState>,
}

impl ItemPriceModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/ItemPrices.json"),
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }
    pub async fn check_update(&self, qf_client: &QFClient) -> Result<(bool, String), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let current_version = client.version.id_price.clone();
        let remote_version = match qf_client.item_price().get_cache_id().await {
            Ok(id) => id,
            Err(e) => {
                let err = Error::from_qf(
                    "Cache:ItemPrice:CheckUpdate",
                    "Failed to get item price cache ID",
                    e,
                    get_location!(),
                );
                err.log("cache_version.json");
                return Err(err);
            }
        };

        if !self.path.exists() {
            Ok((true, remote_version))
        } else {
            Ok((current_version != remote_version, remote_version))
        }
    }

    pub async fn load(
        &self,
        qf_client: &QFClient,
        price_require_update: bool,
    ) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        if price_require_update {
            match self.extract(qf_client).await {
                Ok(()) => {
                    info(
                        "Cache:ItemPrice:Load",
                        "Item price cache extracted successfully.",
                        &LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    e.log("cache_version.json");
                    return Err(e);
                }
            }
        }
        match read_json_file::<Vec<ItemPriceInfo>>(&client.base_path.join(self.path.clone())) {
            Ok(items) => {
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = items;
                info(
                    "Cache:ItemPrice:Load",
                    format!(
                        "Item price cache loaded successfully with {} items.",
                        items_lock.len()
                    ),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    async fn extract(&self, qf_client: &QFClient) -> Result<(), Error> {
        emit_startup!("cache.item_price_updating", json!({}));
        let content = qf_client.item_price().download_cache().await.map_err(|e| {
            Error::from_qf(
                "Cache:ItemPrice",
                "Failed to download cache",
                e,
                get_location!(),
            )
        })?;
        let mut file = File::create(self.path.clone()).map_err(|e| {
            Error::from_io(
                "Cache:ItemPrice",
                &self.path,
                "Failed to create file",
                e,
                get_location!(),
            )
        })?;

        file.write_all(&content).map_err(|e| {
            Error::from_io(
                "Cache:ItemPrice",
                &self.path,
                "Failed to write file",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }

    pub fn get_items(&self) -> Result<Vec<ItemPriceInfo>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }

    pub fn find_by(
        &self,
        url: impl Into<String>,
        sub_type: Option<SubType>,
    ) -> Result<Option<ItemPriceInfo>, Error> {
        let url = url.into();
        let items = self.get_items()?;
        let item = find_by(&items, |u| u.wfm_url == url && u.sub_type == sub_type);
        Ok(item.cloned())
    }
    pub fn find_by_id(
        &self,
        id: impl Into<String>,
        sub_type: Option<SubType>,
    ) -> Result<Option<ItemPriceInfo>, Error> {
        let id = id.into();
        let items = self.get_items()?;
        let item = find_by(&items, |u| u.wfm_id == id && u.sub_type == sub_type);
        Ok(item.cloned())
    }
    pub fn get_by_filter<F>(&self, predicate: F) -> Vec<ItemPriceInfo>
    where
        F: Fn(&ItemPriceInfo) -> bool,
    {
        let items = self.get_items().expect("Failed to get items");
        items
            .into_iter()
            .filter(|item| predicate(item))
            .collect::<Vec<ItemPriceInfo>>()
    }
    /**
     * Creates a new `ItemPriceModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &ItemPriceModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: old.path.clone(),
            client: Arc::downgrade(&client),
            items: Mutex::new(old.items.lock().unwrap().clone()),
        })
    }
}
