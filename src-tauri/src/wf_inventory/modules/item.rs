use std::sync::Arc;

use crate::wf_inventory::WFInventoryState;

#[derive(Debug)]
pub struct ItemModule;

impl ItemModule {
    /**
     * Creates a new `ItemModule` with an empty item list.
     * The `client` parameter is an `Arc<WFInventoryState>` that allows the module
     * to access the live scraper state.
     */
    pub fn new(_client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self)
    }
}
