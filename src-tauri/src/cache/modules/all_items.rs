use std::path::PathBuf;

use eyre::eyre;
use serde_json::json;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_arcane::CacheArcane, cache_item_base::CacheItemBase},
    },
    helper,
    utils::modules::{error::AppError, logger},
};

#[derive(Clone, Debug)]
pub struct AllItemsModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    pub items: Vec<CacheItemBase>,
}

impl AllItemsModule {
    pub fn new(client: CacheClient) -> Self {
        AllItemsModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "AllItems".to_string(),
            items: Vec::new(),
        }
    }
    fn update_state(&self) {
        self.client.update_all_items(self.clone());
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }

    fn add_items(&mut self, items: Vec<CacheItemBase>) {
        for item in items {
            // Check if item already exists
            if self.items.iter().any(|i| i.unique_name == item.unique_name) {
                continue;
            }
        }
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        self.items.append(&mut self.client.arcane().get_all());
        self.items.append(&mut self.client.arch_gun().get_all());
        self.items.append(&mut self.client.arch_melee().get_all());
        self.items.append(&mut self.client.archwing().get_all());
        self.items.append(&mut self.client.fish().get_all());
        self.items.append(&mut self.client.melee().get_all());
        self.items.append(&mut self.client.misc().get_all());
        self.items.append(&mut self.client.mods().get_all());
        self.items.append(&mut self.client.pet().get_all());
        self.items.append(&mut self.client.primary().get_all());
        self.items.append(&mut self.client.relics().get_all());
        self.items.append(&mut self.client.resource().get_all());
        self.items.append(&mut self.client.secondary().get_all());
        self.items.append(&mut self.client.sentinel().get_all());
        self.items.append(&mut self.client.skin().get_all());
        self.items.append(&mut self.client.warframe().get_all());
        self.update_state();
        Ok(())
    }
    pub fn get_by(&self, input: &str, by: &str) -> Result<Option<CacheItemBase>, AppError> {
        let mut items = self.items.clone();
        let args = match helper::validate_args(by, vec!["--item_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--item_by").unwrap();
        let category = args.get("--category");
        let case_insensitive = args.get("--ignore_case").is_some();
        // let lang = args.get("--item_lang").unwrap_or(&"en".to_string());
        let remove_string = args.get("--remove_string");

        if let Some(category) = category {
            items = items
                .iter()
                .filter(|x| x.category.as_str() == category)
                .cloned()
                .collect();
        }

        let item = if mode == "name" {
            items
                .iter()
                .find(|x| helper::is_match(&x.name, input, case_insensitive, remove_string))
                .cloned()
        } else if mode == "unique_name" {
            items
                .iter()
                .find(|x| helper::is_match(&x.unique_name, input, case_insensitive, remove_string))
                .cloned()
        } else {
            return Err(AppError::new(
                &self.get_component("GetBy"),
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(item)
    }
}
