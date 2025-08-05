use std::sync::{Arc, Weak};

use utils::Error;

use crate::live_scraper::LiveScraperState;

#[derive(Debug)]
pub struct RivenModule {
    client: Weak<LiveScraperState>,
}

impl RivenModule {
    /**
     * Creates a new `RivenModule` with an empty item list.
     * The `client` parameter is an `Arc<LiveScraperState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
    pub async fn check(&self) -> Result<(), Error> {
        println!("RivenModule is checking...");
        Ok(())
    }
    /**
     * Creates a new `RivenModule` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(old: &RivenModule, client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
