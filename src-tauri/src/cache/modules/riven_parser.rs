use std::{
    clone,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, Weak},
};

use entity::{
    enums::{RivenAttributeGrade, RivenGrade},
    stock_item::update,
    stock_riven,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::*;
use wf_market::types::Riven;

use crate::{
    cache::*,
    enums::*,
    types::*,
    utils::{modules::states, ItemAttributeDetails},
};

#[derive(Debug)]
pub struct RivenParserModule {
    client: Weak<CacheState>,

    // Lookup maps
    tag_lookup: Mutex<MultiKeyMap<CacheRivenUpgrade>>,
}

impl RivenParserModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            tag_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }
    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let riven_info = client.riven().get_items()?;
        let mut tag_lookup = self.tag_lookup.lock().unwrap();
        for (key, val) in riven_info.upgrade_types {
            for a in val.iter() {
                tag_lookup.insert_value(
                    a.clone(),
                    vec![
                        format!("{}|{}", key, a.wfm_url),
                        format!("{}|{}", key, a.modifier_tag),
                    ],
                );
            }
        }
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    pub fn get_riven_upgrade_by(
        &self,
        riven_type: impl Into<String>,
        tag: impl Into<String>,
    ) -> Result<CacheRivenUpgrade, Error> {
        let riven_type = riven_type.into();
        let tag = tag.into();
        let tag_lookup = self.tag_lookup.lock().unwrap();
        if let Some(upgrade) = tag_lookup.get(&format!("{}|{}", riven_type, tag)) {
            Ok(upgrade.clone())
        } else {
            Err(Error::new(
                "RivenParserModule::get_riven_upgrade_by",
                format!(
                    "Riven upgrade not found for type '{}' and tag '{}'",
                    riven_type, tag
                ),
                get_location!(),
            ))
        }
    }
    /**
     * Creates a new `RivenParserModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenParserModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            tag_lookup: Mutex::new(old.tag_lookup.lock().unwrap().clone()),
        })
    }
}
