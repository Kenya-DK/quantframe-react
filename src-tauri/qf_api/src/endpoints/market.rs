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
pub struct MarketRoute {
    client: Weak<Client>,
}

impl MarketRoute {
    /**
     * Creates a new `MarketRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    pub async fn get_user_activity(&self, query: UserActivityQueryDto) -> Result<Value, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        match client
            .as_ref()
            .call_api::<Value>(
                Method::GET,
                &format!("/wfm/users_active_history?{}", query.get_query()),
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
     * Creates a new `MarketRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &MarketRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
