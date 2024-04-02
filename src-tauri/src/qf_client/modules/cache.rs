use crate::{
    qf_client::{client::QFClient, structs::User},
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
};
use eyre::eyre;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct CacheModule {
    pub client: QFClient,
    pub debug_id: String,
    component: String,
}

impl CacheModule {
    pub fn new(client: QFClient) -> Self {
        CacheModule {
            client,
            debug_id: "qf_client_cache".to_string(),
            component: "Cache".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    // fn update_state(&self) {
    //     self.client.update_cache_module(self.clone());
    // }
    pub async fn get_zip(&self) -> Result<Vec<u8>, AppError> {
        match self.client.get_bytes("cache/zip/download").await {
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

    pub async fn get_cache_id(&self) -> Result<String, AppError> {
        match self.client.get::<Value>("cache/zip/md5").await {
            Ok(ApiResult::Success(payload, _headers)) => {
                let md5 = payload["md5"].as_str().unwrap().to_string();
                return Ok(md5);
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
