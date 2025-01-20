use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{cache_item_component::CacheItemComponent, cache_primary::CachePrimary},
    },
    helper,
    utils::modules::error::AppError,
};

#[derive(Clone, Debug)]
pub struct PrimaryModule {
    pub client: CacheClient,
    // debug_id: String,
    component: String,
    path: PathBuf,
    pub items: Vec<CachePrimary>,
    pub parts: HashMap<String, CacheItemComponent>,
}

impl PrimaryModule {
    pub fn new(client: CacheClient) -> Self {
        PrimaryModule {
            client,
            // debug_id: "ch_client_auction".to_string(),
            component: "Primary".to_string(),
            path: PathBuf::from("items/Primary.json"),
            items: Vec::new(),
            parts: HashMap::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_primary_module(self.clone());
    }

    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let items: Vec<CachePrimary> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse PrimaryModule from file: {}", e)),
            )
        })?;
        self.items = items.clone();
        // loop through items and add parts to parts
        for item in items {
            let components = item.get_item_components();
            for mut part in components {
                part.part_of = Some(item.convert_to_base_item());
                self.add_part(part);
            }
        }
        self.update_state();
        Ok(())
    }
    fn add_part(&mut self, item: CacheItemComponent) {
        self.parts.insert(item.unique_name.clone(), item);
    }
    pub fn get_parts(&self) -> Vec<CacheItemComponent> {
        let mut result: Vec<CacheItemComponent> = Vec::new();
        for item in self.parts.values() {
            result.push(item.clone());
        }
        result
    }
    pub fn get_by(&self, input: &str, by: &str) -> Result<Option<CachePrimary>, AppError> {
        let items = self.items.clone();
        let args = match helper::validate_args(by, vec!["--item_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--item_by").unwrap();
        let case_insensitive = args.get("--ignore_case").is_some();
        // let lang = args.get("--item_lang").unwrap_or(&"en".to_string());
        let remove_string = args.get("--remove_string");

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
