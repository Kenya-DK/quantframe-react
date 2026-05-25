use std::sync::{Arc, Mutex};

use utils::{get_location, info, Error, LoggerOptions, MultiKeyMap};

use crate::{cache::*, types::ChatLink};

#[derive(Debug)]
pub struct AllItemsModule {
    // Lookup maps
    lookup: Mutex<MultiKeyMap<CacheItemBase>>,
}

impl AllItemsModule {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            lookup: Mutex::new(MultiKeyMap::new()),
        })
    }

    pub fn load(&self, client: &CacheState) -> Result<(), Error> {
        let mut lookup = self.lookup.lock().unwrap();

        // Helper closure to reduce duplication
        let mut insert_item = |item: &CacheItemBase| {
            let mut keys = vec![format!("Unique:{}", item.unique_name)];

            if let Some(wfm_url) = &item.wfm_url {
                keys.push(format!("Wfm:{wfm_url}"));
            }

            let mut push_name_key = |name: &str| {
                let key = format!("Name:{name}");
                if lookup.get(&key).is_none() && !keys.iter().any(|existing| existing == &key) {
                    keys.push(key);
                }
            };

            push_name_key(&item.name);
            for prev_name in &item.previous_names {
                push_name_key(prev_name);
            }

            lookup.insert_value(item.clone(), keys);
        };
        for item in client.arcane().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.archgun().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.archmelee().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.archwing().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.fish().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.gear().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.melee().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.misc().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.mods().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.pet().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.relics().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.resource().get_all_items()? {
            insert_item(&item.base);
        }
        for item in client.primary().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.secondary().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.sentinel().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.sentinel_weapon().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.skin().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        for item in client.warframe().get_all_items()? {
            insert_item(&CacheItemBase::from(item.base));
        }
        info(
            "Cache:AllItemsModule:load",
            &format!("Loaded {} items", lookup.len()),
            &LoggerOptions::default(),
        );
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    /// Get Item by various identifiers
    ///  # Arguments
    /// - `id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_by(&self, id: impl Into<String>) -> Result<CacheItemBase, Error> {
        let id: String = id.into();
        const VALID_PREFIXES: [&str; 3] = ["Name:", "Unique:", "Wfm:"];

        if !VALID_PREFIXES.iter().any(|prefix| id.starts_with(prefix)) {
            return Err(Error::new(
                "Cache:GetItemById",
                format!(
                    "Invalid id '{}'. Expected prefixes: {}",
                    id,
                    VALID_PREFIXES.join(", ")
                ),
                get_location!(),
            ));
        }
        let item_lookup = self.lookup.lock().unwrap();
        if let Some(item) = item_lookup.get(&id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:GetItemById",
                format!("Item not found for id '{}'", id),
                get_location!(),
            ))
        }
    }
    /// Get Chat Link for Item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_chat_link(&self, _item_id: impl Into<String>) -> Result<ChatLink, Error> {
        // let client = self.client.upgrade().expect("Client should not be dropped");
        // let item_id: String = item_id.into();
        // let item = self
        //     .get_by(&item_id)
        //     .map_err(|e| e.with_location(get_location!()))?;
        // let mut name = item.name.clone();

        // let (trade_name, tags) =
        //     if let Some(trade_module) = client.tradable_item().get_by(&item_id).ok() {
        //         (trade_module.name.clone(), trade_module.tags.clone())
        //     } else {
        //         (String::new(), Vec::new())
        //     };
        // let mut suffix = String::new();

        // if tags.iter().any(|tag| tag == "blueprint") {
        //     suffix.push_str("Blueprint");
        // } else if tags.iter().any(|tag| tag == "set") {
        //     suffix.push_str("Set");
        // } else if tags.iter().any(|tag| tag == "relic") {
        //     name = format!("{}", trade_name);
        // }
        return Ok(ChatLink::new(
            "NOT IMPLEMENTED".to_string(),
            "NOT IMPLEMENTED".to_string(),
        ));
    }
}
