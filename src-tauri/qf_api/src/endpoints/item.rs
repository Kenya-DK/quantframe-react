use std::sync::{Arc, Weak};

use reqwest::Method;

use crate::{
    client::Client,
    enums::{ApiResponse, ResponseFormat},
    errors::ApiError,
    types::*,
};

#[derive(Debug)]
pub struct ItemRoute {
    client: Weak<Client>,
}

impl ItemRoute {
    /**
     * Creates a new `ItemRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    pub async fn get_prices(
        &self,
        query: ItemPricePaginationQueryDto,
    ) -> Result<Paginated<ItemPrice>, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match client
            .as_ref()
            .call_api::<Paginated<ItemPrice>>(
                Method::GET,
                &format!("/items/prices?{}", query.get_query()),
                None,
                None,
                ResponseFormat::Json,
            )
            .await
        {
            Ok((ApiResponse::Json(alerts), _, _)) => Ok(alerts),
            Err(e) => return Err(e),
            _ => Err(ApiError::Unknown("Unexpected response format".to_string())),
        }
    }
    /**
     * Creates a new `ItemRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &ItemRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
