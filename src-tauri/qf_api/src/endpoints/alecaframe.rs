use std::sync::{Arc, Weak};

use reqwest::Method;

use crate::{
    client::Client,
    enums::{ApiResponse, ResponseFormat},
    errors::ApiError,
    types::*,
};

#[derive(Debug)]
pub struct AlecaframeRoute {
    client: Weak<Client>,
}

impl AlecaframeRoute {
    /**
     * Creates a new `AlecaframeRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    /// Retrieves the decryption keys for Alecaframe.
    /// This method makes a GET request to the `/alecaframe/decrypt-keys` endpoint and returns the keys in a `DecryptKeys` struct.
     /// 
     /// # Errors
     /// Returns an `ApiError` if the request fails or if the response format is unexpected.
    pub async fn get_decrypt_keys(&self) -> Result<DecryptKeys, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        match client
            .as_ref()
            .call_api::<DecryptKeys>(
                Method::GET,
                "/alecaframe/decrypt-keys",
                None,
                None,
                ResponseFormat::Json,
            )
            .await
        {
            Ok((ApiResponse::Json(decrypt_keys), _, _)) => Ok(decrypt_keys),
            Err(e) => return Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }
    
    /**
     * Creates a new `AlecaframeRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &AlecaframeRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
