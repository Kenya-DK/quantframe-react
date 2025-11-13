use std::sync::{Arc, Mutex, Weak};

use utils::{get_location, info, Error, LoggerOptions};

use crate::{cache::*, enums::*, types::ChatLink};

#[derive(Debug)]
pub struct AllItemsModule {
    client: Weak<CacheState>,
    items: Mutex<Vec<CacheItemBase>>,
}

impl AllItemsModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            items: Mutex::new(Vec::new()),
            client: Arc::downgrade(&client),
        })
    }

    pub fn load(&self) -> Result<(), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let mut items_lock = self.items.lock().unwrap();
        items_lock.clear();
        items_lock.append(&mut client.arcane().collect_all_items());
        items_lock.append(&mut client.archgun().collect_all_items());
        items_lock.append(&mut client.archmelee().collect_all_items());
        items_lock.append(&mut client.archwing().collect_all_items());
        items_lock.append(&mut client.fish().collect_all_items());
        items_lock.append(&mut client.melee().collect_all_items());
        items_lock.append(&mut client.misc().collect_all_items());
        items_lock.append(&mut client.mods().collect_all_items());
        items_lock.append(&mut client.pet().collect_all_items());
        items_lock.append(&mut client.primary().collect_all_items());
        items_lock.append(&mut client.relics().collect_all_items());
        items_lock.append(&mut client.resource().collect_all_items());
        items_lock.append(&mut client.secondary().collect_all_items());
        items_lock.append(&mut client.sentinel().collect_all_items());
        items_lock.append(&mut client.skin().collect_all_items());
        items_lock.append(&mut client.warframe().collect_all_items());
        info(
            "Cache:AllItemsModule:load",
            &format!("Loaded {} items", items_lock.len()),
            &LoggerOptions::default(),
        );
        Ok(())
    }
    pub fn get_items(&self) -> Result<Vec<CacheItemBase>, Error> {
        let items = self
            .items
            .lock()
            .expect("Failed to lock items mutex")
            .clone();
        Ok(items)
    }
    pub fn get_by(&self, find_by: FindBy) -> Result<Option<CacheItemBase>, Error> {
        let mut items = self.get_items()?;

        if !find_by.category.is_empty() {
            items = items
                .into_iter()
                .filter(|item| item.category == find_by.category)
                .collect();
        }
        match find_by.find_by {
            FindByType::Name => {
                return Ok(items.into_iter().find(|item| find_by.is_match(&item.name)))
            }
            FindByType::UniqueName => {
                return Ok(items
                    .into_iter()
                    .find(|item| find_by.is_match(&item.unique_name)))
            }
            _ => Err(Error::new(
                "Cache:TradableItem:get_by",
                "Unsupported FindBy type",
                get_location!(),
            )),
        }
    }
    pub fn get_chat_link(&self, find_by: FindBy) -> Result<ChatLink, Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let item = self.get_by(find_by.clone())?;
        if item.is_none() {
            return Err(Error::new(
                "Cache:GetWFName",
                "Item not found",
                get_location!(),
            ));
        }
        let item = item.unwrap();
        let name = item.name.clone();

        let tags = if let Some(trade_module) = client
            .tradable_item()
            .get_by(find_by.clone())
            .ok()
            .flatten()
        {
            trade_module.tags.clone()
        } else {
            Vec::new()
        };
        let mut suffix = String::new();

        if tags.iter().any(|tag| tag == "blueprint") {
            suffix.push_str("Blueprint");
        } else if tags.iter().any(|tag| tag == "set") {
            suffix.push_str("Set");
        }

        if item.part_of_set.is_some() {
            let main_part = self.get_chat_link(FindBy::new(
                FindByType::UniqueName,
                item.part_of_set.unwrap(),
            ))?;
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
        })
    }
}
