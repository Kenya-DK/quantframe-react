use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;

use crate::{
    cache::{
        client::CacheClient,
        types::{
            cache_item_base::CacheItemBase, cache_item_component::CacheItemComponent,
            cache_relics::CacheRelics,
        },
    },
    helper,
    utils::modules::error::AppError,
};
#[derive(Clone, Debug)]
pub struct RelicsModule {
    pub client: CacheClient,
    component: String,
    path: PathBuf,
    pub relics: Vec<CacheRelics>,
}

impl RelicsModule {
    pub fn new(client: CacheClient) -> Self {
        RelicsModule {
            client,
            component: "RelicsModule".to_string(),
            path: PathBuf::from("items/Relics.json"),
            relics: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_relics_module(self.clone());
    }
    pub fn get_all(&self) -> Vec<CacheItemBase> {
        let mut items: Vec<CacheItemBase> = Vec::new();
        items.append(
            &mut self
                .relics
                .iter()
                .map(|item| item.convert_to_base_item())
                .collect(),
        );
        items
    }
    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let relics: Vec<CacheRelics> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse ArcaneModule from file: {}", e)),
            )
        })?;
        self.relics = relics;
        self.update_state();
        Ok(())
    }

    pub fn get_by(&self, input: &str, by: &str) -> Result<Option<CacheRelics>, AppError> {
        let items = self.relics.clone();
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
