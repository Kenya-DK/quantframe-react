use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::{Duration, Instant},
};

use reqwest::Method;
use serde_json::{Value, json};

use crate::{
    client::Client,
    enums::{ApiResponse, ResponseFormat},
    errors::ApiError,
    types::*,
};

#[derive(Debug)]
pub struct ItemPriceRoute {
    client: Weak<Client>,
}

impl ItemPriceRoute {
    /**
     * Creates a new `ItemPriceRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    pub async fn get_cache_id(&self) -> Result<String, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        match client
            .as_ref()
            .call_api::<String>(
                Method::GET,
                "/items/price/md5",
                None,
                None,
                ResponseFormat::String,
            )
            .await
        {
            Ok((ApiResponse::String(md5), _, _)) => Ok(md5),
            Err(e) => return Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }

    pub async fn download_cache(&self) -> Result<Vec<u8>, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        match client
            .as_ref()
            .call_api::<Vec<u8>>(
                Method::GET,
                "/items/price/download",
                None,
                None,
                ResponseFormat::Bytes,
            )
            .await
        {
            Ok((ApiResponse::Bytes(data), _, _)) => Ok(data),
            Err(e) => Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }
    /**
     * Creates a new `ItemPriceRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &ItemPriceRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
