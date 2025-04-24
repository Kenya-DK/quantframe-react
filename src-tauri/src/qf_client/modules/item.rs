use crate::{
    qf_client::{client::QFClient, types::paginated::Paginated},
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
};
use entity::sub_type::SubType;
use eyre::eyre;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct ItemModule {
    pub client: QFClient,
    pub debug_id: String,
    component: String,
}

impl ItemModule {
    pub fn new(client: QFClient) -> Self {
        ItemModule {
            client,
            debug_id: "qf_item".to_string(),
            component: "Item".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    // fn update_state(&self) {
    //     self.client.update_cache_module(self.clone());
    // }
    pub async fn get_price_json_file(&self) -> Result<Vec<u8>, AppError> {
        match self.client.get_bytes("items/price/download").await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetZip"),
                    format!("{} bytes were fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetZip"),
                    error,
                    eyre!("There was an error fetching the cache zip"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get_price_cache_id(&self) -> Result<String, AppError> {
        match self.client.get::<String>("items/price/md5", true).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetCacheId"),
                    error,
                    eyre!("There was an error fetching the cache id"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
