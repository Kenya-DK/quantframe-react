use crate::{
    qf_client::{
        client::QFClient,
        types::{
            item_price::ItemPrice, item_price_chat::ItemPriceChat, paginated::Paginated,
            paginated_with_include::PaginatedWithInclude, syndicates_price::SyndicatesPrice,
        },
    },
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
    pub async fn get_prices(
        &self,
        page: i64,
        limit: i64,
        from_date: String,
        to_date: String,
        order_type: Option<String>,
        wfm_url: Option<String>,
        sub_type: Option<SubType>,
        include: Option<String>,
        group_by: Option<String>,
        sort: Option<Value>,
    ) -> Result<PaginatedWithInclude<ItemPrice, ItemPriceChat>, AppError> {
        let mut params = vec![];
        params.push(("page", page.to_string()));
        params.push(("limit", limit.to_string()));
        params.push(("from_date", from_date));
        params.push(("to_date", to_date));
        if let Some(order_type) = order_type {
            params.push(("order_type", order_type));
        }
        if let Some(wfm_url) = wfm_url {
            params.push(("wfm_url", wfm_url));
        }
        if let Some(include) = include {
            params.push(("include", include));
        }
        if let Some(group_by) = group_by {
            params.push(("group_by", group_by));
        }
        if let Some(sort) = sort {
            params.push(("sort", sort.to_string()));
        }
        if let Some(sub_type) = sub_type {
            params.push(("sub_type", json!(sub_type).to_string()));
        }

        let params = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        let url = format!("items/prices?{}", params);
        match self
            .client
            .get::<PaginatedWithInclude<ItemPrice, ItemPriceChat>>(&url, false)
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
