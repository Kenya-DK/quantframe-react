use std::sync::{Arc, Mutex};

use serde_json::json;

use crate::{
    cache::client::CacheClient,
    error::AppError,
    logger,
    structs::{RivenAttributeInfo, RivenTypeInfo}, helper,
};
pub struct RivenModule<'a> {
    pub client: &'a CacheClient,
}

impl<'a> RivenModule<'a> {
    // Refrece
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.refresh_types().await?;
        self.refresh_attributes().await?;
        Ok(())
    }
    pub async fn refresh_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        let riven_types = wfm.auction().get_all_riven_types().await?;
        let mut types = self.client.riven_types.lock()?;
        *types = riven_types.clone();
        Ok(riven_types)
    }
    pub async fn refresh_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let wfm = self.client.wfm.lock()?.clone();
        let rattributes = wfm.auction().get_all_riven_attribute_types().await?;
        let mut attributes = self.client.riven_attributes.lock()?;
        *attributes = rattributes.clone();
        Ok(rattributes)
    }
    pub fn get_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let items = self.client.riven_types.lock()?;
        Ok(items.clone())
    }

    pub fn get_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let attributes = self.client.riven_attributes.lock()?;
        Ok(attributes.clone())
    }

    pub fn find_type(&self, url_name: &str) -> Result<Option<RivenTypeInfo>, AppError> {
        let types = self.client.riven_types.lock()?;
        let riven_type = types.iter().find(|&x| x.url_name == url_name).cloned();
        if !riven_type.is_some() {
            logger::warning_con(
                "CacheRivens",
                format!("Riven Type: {} not found", url_name).as_str(),
            );
        }
        Ok(riven_type)
    }

    pub fn find_attribute(
        &self,
        url_name: &str,
    ) -> Result<Option<RivenAttributeInfo>, AppError> {
        let attributes = self.client.riven_attributes.lock()?;
        let riven_attribute = attributes.iter().find(|&x| x.url_name == url_name).cloned();
        if !riven_attribute.is_some() {
            logger::warning_con(
                "CacheRivens",
                format!("Riven Attribute: {} not found", url_name).as_str(),
            );            
        }
        Ok(riven_attribute)
    }
    pub fn emit(&self) {
        helper::send_message_to_window(
            "Cache:Update:Rivens",
            Some(json!({
                "types": self.client.riven_types.lock().unwrap().clone(),
                "attributes": self.client.riven_attributes.lock().unwrap().clone(),
            })),
        );
    }
}
