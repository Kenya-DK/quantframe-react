use std::{
    fs::File,
    io::{Read, Write},
    ops::Sub,
    path::PathBuf,
};

use entity::sub_type::{self, SubType};
use eyre::eyre;

use crate::{
    cache::{client::CacheClient, types::item_price_info::ItemPriceInfo},
    utils::modules::{error::AppError, logger},
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

    pub fn get_items(&self) -> Result<Vec<ItemPriceInfo>, AppError> {
        Ok(self.items.clone())
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
        let qf = self.client.qf.lock()?.clone();
        let price_data = qf.price().get_json_file().await?;
        match self.client.write_text_to_file(&self.path, price_data) {
            Ok(_) => {
                logger::info_con(&self.component, "Item prices have been updated.");
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
    pub async fn load(&mut self) -> Result<(), AppError> {
        let qf = self.client.qf.lock()?.clone();
        let current_cache_id = self.get_cache_id()?;
        logger::info_con(
            &self.component,
            format!("Current price cache id: {}", current_cache_id).as_str(),
        );
        let remote_cache_id = match qf.price().get_cache_id().await {
            Ok(id) => id,
            Err(e) => {
                logger::error_con(
                    &self.component,
                    format!(
                        "There was an error fetching the price cache id: {}",
                        e.get_info().0
                    )
                    .as_str(),
                );
                logger::info_con(&self.component, "Using the current price cache id");
                current_cache_id.clone()
            }
        };
        logger::info_con(
            &self.component,
            format!("Remote price cache id: {}", remote_cache_id).as_str(),
        );
        if current_cache_id != remote_cache_id {
            logger::info_con(
                &self.component,
                "Price cache id mismatch, downloading new price cache data",
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
        let items = self.get_items()?;
        let item = items
            .iter()
            .find(|item| {
                item.url_name == url_name
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
