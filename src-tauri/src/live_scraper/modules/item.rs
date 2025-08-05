use std::sync::{Arc, Weak};

use utils::Error;

use crate::live_scraper::LiveScraperState;

#[derive(Debug)]
pub struct ItemModule {
    client: Weak<LiveScraperState>,
}

impl ItemModule {
    /**
     * Creates a new `ItemModule` with an empty item list.
     * The `client` parameter is an `Arc<LiveScraperState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }

    pub async fn check(&self) -> Result<(), Error> {
        println!("ItemModule is checking...");
        Ok(())
    }

    /**
     * Creates a new `ItemModule` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(old: &ItemModule, client: Arc<LiveScraperState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
