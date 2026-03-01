use std::sync::{Arc, Weak};

use crate::wf_inventory::WFInventoryState;

#[derive(Debug)]
pub struct ItemModule {
    client: Weak<WFInventoryState>,
}

impl ItemModule {
    /**
     * Creates a new `ItemModule` with an empty item list.
     * The `client` parameter is an `Arc<WFInventoryState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
