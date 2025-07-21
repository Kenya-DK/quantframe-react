use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::{Duration, Instant},
};

use reqwest::Method;
use serde_json::{Value, json};

use crate::{client::Client, errors::ApiError, types::*};

#[derive(Debug)]
pub struct AlertRoute {
    client: Weak<Client>,
}

impl AlertRoute {
    /**
     * Creates a new `AlertRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    pub async fn get_alerts(&self) -> Result<Paginated<Alert>, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        match client
            .as_ref()
            .call_api::<Paginated<Alert>>(
                Method::GET,
                "/alert?page=1&limit=25&enabled=true",
                None,
                None,
            )
            .await
        {
            Ok((alerts, _, _)) => Ok(alerts),
            Err(e) => return Err(e),
        }
    }
    /**
     * Creates a new `AlertRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(_old: &AlertRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
