use std::path::PathBuf;

use entity::sub_type::SubType;
use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::item_price_info::ItemPriceInfo},
    utils::modules::{
        error::AppError,
        logger::{self, LoggerOptions},
        states,
    },
};

#[derive(Clone, Debug)]
pub struct ItemPriceModule {
    pub client: CacheClient,
    component: String,
    path: PathBuf,
    pub items: Vec<ItemPriceInfo>,
    md5_file: String,
}

impl ItemPriceModule {
    pub fn new(client: CacheClient) -> Self {
        ItemPriceModule {
            client,
            component: "ItemPrice".to_string(),
            md5_file: "price_id.txt".to_string(),
            path: PathBuf::from("items/ItemPrices.json"),
            items: Vec::new(),
        }
    }

    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_item_price_module(self.clone());
    }

    pub fn get_all(&self) -> Result<Vec<ItemPriceInfo>, AppError> {
        Ok(self.items.clone())
    }

    pub fn get_by_filter<F>(&self, predicate: F) -> Vec<ItemPriceInfo>
    where
        F: Fn(&ItemPriceInfo) -> bool,
    {
        let items = self.get_all().expect("Failed to get items");
        items
            .into_iter()
            .filter(|item| predicate(item))
            .collect::<Vec<ItemPriceInfo>>()
    }

    pub fn update_cache_id(&mut self, cache_id: String) -> Result<(), AppError> {
        match self.client.write_text_to_file(
            &PathBuf::from(self.md5_file.clone()),
            cache_id.as_bytes().to_vec(),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn get_cache_id(&mut self) -> Result<String, AppError> {
        match self
            .client
            .read_text_from_file(&PathBuf::from(self.md5_file.clone()))
        {
            Ok(id) => Ok(id),
            Err(_) => Ok("N/A".to_string()),
        }
    }
    pub async fn download_cache_data(&mut self) -> Result<(), AppError> {
        let qf = states::qf_client()?;
        let price_data = qf.item().get_price_json_file().await?;
        match self.client.write_text_to_file(&self.path, price_data) {
            Ok(_) => {
                logger::info(
                    &self.component,
                    "Item prices have been updated.",
                    LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
    pub async fn load(&mut self) -> Result<(), AppError> {
        let qf = states::qf_client()?;
        let current_cache_id = self.get_cache_id()?;
        let remote_cache_id = match qf.item().get_price_cache_id().await {
            Ok(id) => id,
            Err(e) => {
                logger::error(
                    &self.component,
                    format!(
                        "There was an error fetching the price cache id: {}",
                        e.get_info().0
                    )
                    .as_str(),
                    LoggerOptions::default(),
                );
                logger::info(
                    &self.component,
                    "Using the current price cache id",
                    LoggerOptions::default(),
                );
                current_cache_id.clone()
            }
        };
        if current_cache_id != remote_cache_id {
            logger::info(
                &self.component,
                "Price cache id mismatch, downloading new price cache data",
                LoggerOptions::default(),
            );
            self.download_cache_data().await?;
            self.update_cache_id(remote_cache_id)?;
        }
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<ItemPriceInfo> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ItemPriceModule from file: {}", e)),
            )
        })?;
        self.items = items.clone();
        self.update_state();
        Ok(())
    }

    pub fn get_item_price(
        &self,
        url_name: &str,
        sub_type: Option<SubType>,
        order_type: &str,
    ) -> Result<ItemPriceInfo, AppError> {
        let items = self.get_all()?;
        let item = items
            .iter()
            .find(|item| {
                item.wfm_url == url_name
                    && item.order_type == order_type
                    && item.sub_type == sub_type
            })
            .ok_or_else(|| {
                AppError::new(
                    &self.component,
                    eyre!(format!("Item not found: {}", url_name)),
                )
            })?;
        Ok(item.clone())
    }
}
