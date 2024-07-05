use std::{
    fs::File,
    io::{Read, Write},
};

use eyre::eyre;


use crate::{
    cache::{client::CacheClient, types::item_price_info::ItemPriceInfo},
    utils::modules::{error::AppError, logger},
};

#[derive(Clone, Debug)]
pub struct ItemPriceModule {
    pub client: CacheClient,
    component: String,
    json_file: String,
    md5_file: String,
    folder: String,
}

impl ItemPriceModule {
    pub fn new(client: CacheClient) -> Self {
        ItemPriceModule {
            client,
            component: "ItemPrice".to_string(),
            json_file: "item_prices.json".to_string(),
            md5_file: "price_id.txt".to_string(),
            folder: "price".to_string(),
        }
    }

    pub fn get_items(&self) -> Result<Vec<ItemPriceInfo>, AppError> {
        let path = self
            .client
            .get_path(self.folder.as_str())
            .join(self.json_file.clone());
        let content = std::fs::read_to_string(path).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!("Failed to read file: {}", e.to_string())),
            )
        })?;
        let items: Vec<ItemPriceInfo> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!("Failed to parse json: {}", e.to_string())),
            )
        })?;
        Ok(items)
    }

    pub fn update_cache_id(&mut self, cache_id: String) -> Result<(), AppError> {
        let cache_path = self
            .client
            .get_path(self.folder.as_str())
            .join(self.md5_file.clone());
        let mut file = File::create(cache_path)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        file.write_all(cache_id.as_bytes())
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        Ok(())
    }

    fn get_cache_id(&mut self) -> Result<String, AppError> {
        let cache_path = self
            .client
            .get_path(self.folder.as_str())
            .join(self.md5_file.clone());
        if !cache_path.exists() {
            return Ok("N/A".to_string());
        }
        let mut file = File::open(cache_path)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
        Ok(content)
    }
    pub async fn download_cache_data(&mut self) -> Result<(), AppError> {
        let qf = self.client.qf.lock()?.clone();
        let price_data = qf.price().get_json_file().await?;

        let extract_to = self
            .client
            .get_path(self.folder.as_str())
            .join(self.json_file.clone());
        let mut out = File::create(extract_to).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!("Failed to create file: {}", e)),
            )
        })?;
        out.write_all(&price_data).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!("Failed to write to file: {}", e)),
            )
        })?;

        logger::info_con(&self.component, "Item prices have been updated.");
        Ok(())
    }
    pub async fn load(&mut self) -> Result<(), AppError> {
        let qf = self.client.qf.lock()?.clone();
        let settings = self.client.settings.lock()?.clone();
        if settings.dev_mode {
            logger::warning_con(&self.component, "DevMode is enabled, using old item prices");
            return Ok(());
        }

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
        Ok(())
    }
}
