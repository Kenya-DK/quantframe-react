use std::sync::{Arc, Weak};

use utils::*;

use crate::{utils::modules::states, wf_inventory::WFInventoryState};

#[derive(Debug)]
pub struct SyndicateModule {
    client: Weak<WFInventoryState>,
}

impl SyndicateModule {
    pub fn get_syndicates(&self) -> Result<(), Error> {
        let client = self.client.upgrade().unwrap();
        let root = client.get_root();
        let cache = states::cache_client()?;

        let items = root.affiliations;
        for item in items.iter() {
            println!("Affiliation: {}", item.id);
        }
        Ok(())
    }

    pub fn new(client: Arc<WFInventoryState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
        })
    }
}
