use crate::{
    qf_client::{
        client::QFClient,
        types::{paginated::Paginated, syndicates_price::SyndicatesPrice},
    },
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
};
use eyre::eyre;
use serde_json::Value;

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
        match self.client.get_bytes("items/price_download").await {
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
        match self.client.get::<String>("items/price_md5", true).await {
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

    pub async fn get_syndicates_prices(
        &self,
        page: i64,
        limit: i64,
        sort: Option<Value>,
    ) -> Result<Paginated<SyndicatesPrice>, AppError> {
        let mut params = vec![];
        if let Some(sort) = sort {
            params.push(("sort", sort.to_string()));
        }
        params.push(("page", page.to_string()));
        params.push(("limit", limit.to_string()));
        let params = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        let url = format!("items/syndicates_prices?{}", params);
        match self
            .client
            .get::<Paginated<SyndicatesPrice>>(&url, false)
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetSyndicatesPrices"),
                    error,
                    eyre!("There was an error fetching the syndicates prices"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
