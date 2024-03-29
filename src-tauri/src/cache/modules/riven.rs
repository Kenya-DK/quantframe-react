use std::sync::Arc;

use serde_json::json;

use crate::{
    cache::client::CacheClient,
    error::AppError,
    helper, logger,
    structs::{RivenAttributeInfo, RivenTypeInfo},
};
#[derive(Clone, Debug)]
pub struct RivenModule {
    pub client: CacheClient,
    debug_id: String,
    component: String,
}

impl RivenModule{
    pub fn new(client: CacheClient) -> Self {
        RivenModule {
            client,
            debug_id: "ch_client_auction".to_string(),
            component: "Riven".to_string(),
        }
    }
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
