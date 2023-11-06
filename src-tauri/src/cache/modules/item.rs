use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use eyre::eyre;
use serde_json::{json, Value};

use crate::{
    cache::client::CacheClient,
    error::AppError,
    helper, logger,
    structs::{Item, ItemDetails},
};

pub struct ItemModule<'a> {
    pub client: &'a CacheClient,
}

impl<'a> ItemModule<'a> {
    // Refrece
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.refresh_types().await?;
        Ok(())
    }

    pub async fn refresh_types(&self) -> Result<Vec<Item>, AppError> {
        let wfm = self.client.wfm.lock()?.clone();

        helper::emit_undate_initializ_status("Downloading Item Data from Warframe.Market...", None);
        let wfm_items = wfm.items().get_all_items().await?;
        helper::send_message_to_window(
            "set_initializstatus",
            Some(json!({"status": "Downloading Item Data from Github..."})),
        );
        let response2: Vec<Value> =
            reqwest::get("https://github.com/WFCD/warframe-items/raw/master/data/json/All.json")
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?
                .json()
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?;

        helper::emit_undate_initializ_status("Downloading Item Data from Relics.Run...", None);
        let response: HashMap<String, Value> =
            reqwest::get("https://relics.run/history/item_data/item_info.json")
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?
                .json()
                .await
                .map_err(|e| AppError::new("CacheItems", eyre!(e.to_string())))?;

        helper::emit_undate_initializ_status("Storing Looping through Item Data...", None);
        let mut items: Vec<Item> = Vec::new();
        for item in wfm_items.clone() {
            let relic_data = response.get(&item.id.clone());
            if relic_data.is_some() {
                let mut new = item.clone();
                let set_items = self.get_string_arry_from_json(relic_data.unwrap(), "set_items");
                let tags = self.get_string_arry_from_json(relic_data.unwrap(), "tags");
                let subtypes = self.get_string_arry_from_json(relic_data.unwrap(), "subtypes");
                let mod_max_rank = relic_data.unwrap().get("mod_max_rank").unwrap().as_i64();
                let mut mr_requirement: Option<i64> = Some(0);
                // Find the item response2 by name property
                let item_details = response2
                    .iter()
                    .find(|&x| x["name"].as_str().unwrap() == item.item_name);

                if item_details.is_some() {
                    // Get Mastery Requirement
                    let item_details = item_details.unwrap();
                    if item_details["masteryReq"].is_i64() {
                        mr_requirement = item_details["masteryReq"].as_i64();
                    }
                }

                new.set_items = Some(set_items);
                new.tags = Some(tags.clone());
                new.subtypes = Some(subtypes);
                new.mod_max_rank = mod_max_rank;
                new.trade_tax = Some(helper::calculate_trade_tax(tags, mod_max_rank));
                new.mr_requirement = mr_requirement;
                // Only send for every 10th item
                if items.len() % 100 == 0 {
                    helper::emit_undate_initializ_status(
                        format!(
                            "Storing Item Data: {} {}/{}",
                            item.item_name,
                            items.len(),
                            wfm_items.len()
                        )
                        .as_str(),
                        None,
                    );
                }

                items.push(new.clone());
            }
        }

        let arced_mutex = Arc::clone(&self.client.cache_data);
        let mut my_lock = arced_mutex.lock()?;
        my_lock.item.items = items.clone();
        Ok(items)
    }

    pub fn get_types(&self) -> Result<Vec<Item>, AppError> {
        let items = self.client.cache_data.lock()?.clone().item.items;
        Ok(items)
    }

    pub fn find_type(&self, url_name: &str) -> Result<Option<Item>, AppError> {
        let types = self.client.cache_data.lock()?.clone().item.items;
        let item_type = types.iter().find(|&x| x.url_name == url_name).cloned();
        if !item_type.is_some() {
            logger::warning_con(
                "CacheItems",
                format!("Item Type: {} not found", url_name).as_str(),
            );
        }
        Ok(item_type)
    }

    pub fn emit(&self) {
        let types = self.client.cache_data.lock().unwrap().clone().item.items;
        helper::send_message_to_window(
            "Cache:Update:Items",
            Some(serde_json::to_value(types).unwrap()),
        );
    }
    fn get_string_arry_from_json(&self, json: &Value, key: &str) -> Vec<String> {
        let mut string_vec = vec![];
        if let Some(array) = json.get(key).unwrap().as_array() {
            string_vec = array
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
        string_vec
    }
}
