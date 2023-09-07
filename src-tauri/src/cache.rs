use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::error::{AppError, GetErrorInfo};
use crate::structs::Item;
use crate::wfm_client::WFMClientState;
use crate::{helper, logger};
use eyre::eyre;

#[derive(Clone, Debug)]
pub struct CacheState {
    pub items: Arc<Mutex<Vec<Item>>>,
    log_file: PathBuf,
    wfm: Arc<Mutex<WFMClientState>>,
}
impl CacheState {
    pub fn new(wfm: Arc<Mutex<WFMClientState>>) -> Self {
        CacheState {
            items: Arc::new(Mutex::new(vec![])),
            log_file: PathBuf::from("cache"),
            wfm,
        }
    }

    pub async fn update_cache(&self) -> Result<bool, AppError> {
        match self.update_tradable_items().await {
            Ok(_) => {}
            Err(e) => {
                let component = e.component();
                let cause = e.cause();
                let backtrace = e.backtrace();
                let log_level = e.log_level();
                crate::logger::dolog(
                    log_level,
                    component.as_str(),
                    format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
                    true,
                    Some(format!("error_{}_{}.log", component, chrono::Local::now().format("%Y-%m-%d")).as_str()),
                );               
            }
        }
        Ok(true)
    }
    pub async fn update_tradable_items(&self) -> Result<bool, AppError> {
        let wfm = self.wfm.lock()?.clone();
        let items = wfm.get_tradable_items().await?;
        let response: HashMap<String, Value> =
            reqwest::get("https://relics.run/history/item_data/item_info.json")
                .await
                .map_err(|e| AppError("CacheState", eyre!(e.to_string())))?
                .json()
                .await
                .map_err(|e| AppError("CacheState", eyre!(e.to_string())))?;

        let mut new_items: Vec<Item> = Vec::new();
        // Link items with relic data on item_id
        for item in items.clone() {
            let relic_data = response.get(&item.id.clone());
            if relic_data.is_some() {
                let mut new = item.clone();
                let set_items = get_string_arry_from_json(relic_data.unwrap(), "set_items");
                let tags = get_string_arry_from_json(relic_data.unwrap(), "tags");
                let subtypes = get_string_arry_from_json(relic_data.unwrap(), "subtypes");
                let mod_max_rank = relic_data.unwrap().get("mod_max_rank").unwrap().as_i64();
                new.set_items = Some(set_items);
                new.tags = Some(tags);
                new.subtypes = Some(subtypes);
                new.mod_max_rank = mod_max_rank;
                new_items.push(new.clone());
            }
        }
        let mut sitems = self.items.lock()?;
        *sitems = new_items.clone();
        helper::send_message_to_window("update_tradable_items", Some(json!(new_items)));
        // self.items.store(items, Ordering::SeqCst);
        // self.items = items;
        Ok(true)
    }
    pub fn get_item_by_url_name(&self, url_name: &str) -> Option<Item> {
        let items = self.items.lock().unwrap();
        let item = items.iter().find(|i| i.url_name == url_name);
        if item.is_some() {
            return Some(item.unwrap().clone());
        }
        None
    }
}

fn get_string_arry_from_json(json: &Value, key: &str) -> Vec<String> {
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
