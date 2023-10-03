use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{auth::AuthState, error::AppError, logger, cache::client::CacheClient};
pub struct RivenModule<'a> {
    pub client: &'a CacheClient,
    pub riven_types: Arc<Mutex<Vec<RivenTypeInfo>>>,
    pub riven_attributes: Arc<Mutex<Vec<RivenAttributeInfo>>>,
}

impl<'a> RivenModule<'a> {
    // Refrece
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.refresh_types().await?;
        self.refresh_attributes().await?;        
    }
    pub async fn refresh_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let (payload, _headers) = self.client.get("riven/items", Some("items")).await?;
        let mut types = self.riven_types.lock()?;
        *types = payload.clone();
        Ok(payload)
    }
    pub async fn refresh_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let (payload, _headers) = self
            .client
            .get("riven/attributes", Some("attributes"))
            .await?;
        let mut attributes = self.riven_attributes.lock()?;
        *attributes = payload.clone();
        Ok(payload)
    }    
    pub async fn get_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let items = self.items.lock()?;
        Ok(items.clone())
    }

    pub async fn get_attributes(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let attributes = self.riven_attributes.lock()?;
        Ok(attributes.clone())
    }

    pub async fn find_type(&self, url_name: &str) -> Result<Option<RivenTypeInfo>, AppError> {
        let types = self.riven_types.lock()?;
        let riven_type = types.iter().find(|&x| x.url_name == url_name);
        if riven_type.is_some() {
            Ok(riven_type.unwrap().clone())
        } else {
            logger::warning_con(
                "CacheRivens",
                format!("Riven Type: {} not found", url_name).as_str()
            );
            Ok(None)
        }
    }

    pub async fn find_attribute(&self, url_name: &str) -> Result<Option<RivenAttributeInfo>, AppError> {
        let attributes = self.riven_attributes.lock()?;
        let riven_attribute = attributes.iter().find(|&x| x.url_name == url_name);
        if riven_attribute.is_some() {
            Ok(riven_attribute.unwrap().clone())
        } else {
            logger::warning_con(
                "CacheRivens",
                format!("Riven Attribute: {} not found", url_name).as_str()
            );
            Ok(None)
        }
    }
    pub fn emit(&self) {
        helper::send_message_to_window(
            "Cache:Update:Rivens",
            Some(json!({
                "types": self.riven_types.lock().unwrap().clone(),
                "attributes": self.riven_attributes.lock().unwrap().clone(),
            })),
        );
    }
}
