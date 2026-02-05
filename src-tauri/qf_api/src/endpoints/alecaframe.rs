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
