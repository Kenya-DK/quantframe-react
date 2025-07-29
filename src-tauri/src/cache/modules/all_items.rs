use std::{
    path::PathBuf,
    sync::{Arc, Weak},
};

use utils::{find_by, info, read_json_file, Error, LoggerOptions};

use crate::cache::client::CacheState;

#[derive(Debug)]
pub struct AllItemsModule {
    path: PathBuf,
    client: Weak<CacheState>,
}

impl AllItemsModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/ItemPrices.json"),
            client: Arc::downgrade(&client),
        })
    }

    pub fn load(&self) -> Result<(), Error> {
        Ok(())
    }
}
