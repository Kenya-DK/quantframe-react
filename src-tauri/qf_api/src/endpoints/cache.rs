use std::
    sync::{Arc, Weak}
;

use reqwest::Method;

use crate::{
    client::Client,
    enums::{ApiResponse, ResponseFormat},
    errors::ApiError,
};

#[derive(Debug)]
pub struct CacheRoute {
    client: Weak<Client>,
}

impl CacheRoute {
    /**
     * Creates a new `CacheRoute` with an empty Authentication list.
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
                "/cache/md5",
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
                "/cache/download",
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
     * Creates a new `CacheRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &CacheRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
