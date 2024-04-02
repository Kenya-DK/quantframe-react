use std::{path::PathBuf, sync::Arc};

use eyre::eyre;
use serde_json::json;

use crate::{
    cache::{
        client::CacheClient,
        types::cache_riven::{CacheRiven, CacheRivenWfmAttribute, CacheRivenWfmWeapon},
    },
    helper, logger,
    utils::modules::error::AppError,
    wfm_client::types::{riven_attribute_info::RivenAttributeInfo, riven_type_info::RivenTypeInfo},
};
#[derive(Clone, Debug)]
pub struct RivenModule {
    pub client: CacheClient,
    debug_id: String,
    component: String,
    path: PathBuf,
    data: CacheRiven,
}

impl RivenModule {
    pub fn new(client: CacheClient) -> Self {
        RivenModule {
            client,
            debug_id: "ch_client_auction".to_string(),
            path: PathBuf::from("riven/rivens.json"),
            data: CacheRiven::new(),
            component: "RivenModule".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_riven_module(self.clone());
    }
    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let data: CacheRiven = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse RivenModule from file: {}", e)),
            )
        })?;
        self.data = data.clone();
        self.update_state();
        Ok(())
    }

    pub fn get_wfm_riven_types(&self) -> Result<Vec<CacheRivenWfmWeapon>, AppError> {
        let items = self.data.wfm_weapons.clone();
        Ok(items)
    }
    pub fn get_wfm_riven_attributes(&self) -> Result<Vec<CacheRivenWfmAttribute>, AppError> {
        let attributes = self.data.wfm_attributes.clone();
        Ok(attributes)
    }
    // Old code

    pub async fn refresh(&self) -> Result<(), AppError> {
        self.refresh_types().await?;
        self.refresh_attributes().await?;
        Ok(())
    }
    pub async fn refresh_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        helper::send_message_to_window(
            "set_initializstatus",
            Some(json!({"status": "Downloading Riven Data from Warframe.Market..."})),
        );
        let riven_types = wfm.auction().get_all_riven_types().await?;

        let arced_mutex = Arc::clone(&self.client.cache_data);
        let mut my_lock = arced_mutex.lock()?;
        my_lock.riven.items = riven_types.clone();
        Ok(riven_types)
    }
    pub async fn refresh_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        helper::send_message_to_window(
            "set_initializstatus",
            Some(json!({"status": "Downloading Riven Attribute Data from Warframe.Market..."})),
        );
        let all_riven_attributes = wfm.auction().get_all_riven_attribute_types().await?;
        let arced_mutex = Arc::clone(&self.client.cache_data);
        let mut my_lock = arced_mutex.lock()?;
        my_lock.riven.attributes = all_riven_attributes.clone();
        Ok(all_riven_attributes)
    }
    pub fn get_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let items = self.client.cache_data.lock()?.clone().riven.items;
        Ok(items.clone())
    }

    pub fn get_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let attributes = self.client.cache_data.lock()?.clone().riven.attributes;
        Ok(attributes.clone())
    }

    pub fn find_type(&self, url_name: &str) -> Result<Option<RivenTypeInfo>, AppError> {
        let types = self.client.cache_data.lock()?.clone().riven.items;
        let riven_type = types.iter().find(|&x| x.url_name == url_name).cloned();
        if !riven_type.is_some() {
            logger::warning_con(
                "CacheRivens",
                format!("Riven Type: {} not found", url_name).as_str(),
            );
        }
        Ok(riven_type)
    }
}
