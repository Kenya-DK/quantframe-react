use std::sync::{Arc, Mutex, Weak};

use utils::{get_location, info, Error, LoggerOptions, MultiKeyMap};

use crate::{cache::*, types::ChatLink};

#[derive(Debug)]
pub struct AllItemsModule {
    client: Weak<CacheState>,
    items: Mutex<Vec<CacheItemBase>>,

    // Lookup maps
    item_lookup: Mutex<MultiKeyMap<CacheItemBase>>,
}

impl AllItemsModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
            item_lookup: Mutex::new(MultiKeyMap::new()),
        })
    }

    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let mut items = vec![];
        items.append(&mut client.arcane().collect_all_items());
        items.append(&mut client.archgun().collect_all_items());
        items.append(&mut client.archmelee().collect_all_items());
        items.append(&mut client.archwing().collect_all_items());
        items.append(&mut client.fish().collect_all_items());
        items.append(&mut client.melee().collect_all_items());
        items.append(&mut client.misc().collect_all_items());
        items.append(&mut client.mods().collect_all_items());
        items.append(&mut client.pet().collect_all_items());
        items.append(&mut client.primary().collect_all_items());
        items.append(&mut client.relics().collect_all_items());
        items.append(&mut client.resource().collect_all_items());
        items.append(&mut client.secondary().collect_all_items());
        items.append(&mut client.sentinel().collect_all_items());
        items.append(&mut client.sentinel_weapon().collect_all_items());
        items.append(&mut client.skin().collect_all_items());
        items.append(&mut client.warframe().collect_all_items());
        for item in items.iter() {
            let mut item_lookup = self.item_lookup.lock().unwrap();
            item_lookup.insert_value(
                item.clone(),
                vec![
                    item.unique_name.clone(),
                    item.name.clone(),
                    format!("{}|{}", item.category, item.name),
                    format!("{}|{}", item.category, item.unique_name),
                ],
            );
        }
        info(
            "Cache:AllItemsModule:load",
            &format!("Loaded {} items", items.len()),
            &LoggerOptions::default(),
        );
        Ok(())
    }
    /* -------------------------------------------------------------
        Lookup Functions
    ------------------------------------------------------------- */
    /// Get Item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_by(&self, item_id: impl Into<String>) -> Result<CacheItemBase, Error> {
        let item_id: String = item_id.into();
        let item_lookup = self.item_lookup.lock().unwrap();
        if let Some(item) = item_lookup.get(&item_id) {
            Ok(item.clone())
        } else {
            Err(Error::new(
                "Cache:GetItemById",
                format!("Item not found for id '{}'", item_id),
                get_location!(),
            ))
        }
    }
    /// Get Chat Link for Item by various identifiers
    ///  # Arguments
    /// - `item_id`: The identifier to search for (name, unique_name, category+name, category+unique_name)
    ///
    pub fn get_chat_link(&self, item_id: impl Into<String>) -> Result<ChatLink, Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let item_id: String = item_id.into();
        let item = self
            .get_by(&item_id)
            .map_err(|e| e.with_location(get_location!()))?;
        let mut name = item.name.clone();

        let (trade_name, tags) =
            if let Some(trade_module) = client.tradable_item().get_by(&item_id).ok() {
                (trade_module.name.clone(), trade_module.tags.clone())
            } else {
                (String::new(), Vec::new())
            };
        let mut suffix = String::new();

        if tags.iter().any(|tag| tag == "blueprint") {
            suffix.push_str("Blueprint");
        } else if tags.iter().any(|tag| tag == "set") {
            suffix.push_str("Set");
        } else if tags.iter().any(|tag| tag == "relic") {
            name = format!("{}", trade_name);
        }

        if item.part_of_set.is_some() {
            let main_part = self.get_chat_link(item.part_of_set.unwrap())?;
            if name != "Blueprint" {
                return Ok(ChatLink::new(
                    format!("{} {}", main_part.link, name),
                    &suffix,
                ));
            } else {
                return Ok(ChatLink::new(format!("{}", main_part.link), &suffix));
            }
        }
        return Ok(ChatLink::new(name.trim(), &suffix));
    }
    /**
     * Creates a new `AllItemsModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &AllItemsModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            items: Mutex::new(old.items.lock().unwrap().clone()),
            item_lookup: Mutex::new(old.item_lookup.lock().unwrap().clone()),
        })
    }
}
