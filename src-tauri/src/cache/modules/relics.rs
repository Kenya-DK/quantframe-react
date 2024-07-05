use std::{collections::HashMap, path::PathBuf};

use eyre::eyre;
use regex::Regex;
use serde_json::json;

use crate::{
    cache::{
        client::CacheClient,
        types::{
            cache_item_component::CacheItemComponent,
            cache_relics::CacheRelics,
            cache_riven::{
                CacheRiven, CacheRivenDataByRivenInternalID, CacheRivenWfmAttribute,
                CacheRivenWfmWeapon, CacheWeaponStat, RivenStat,
            },
        },
    },
    helper, logger,
    utils::modules::error::AppError,
};
#[derive(Clone, Debug)]
pub struct RelicsModule {
    pub client: CacheClient,
    debug_id: String,
    component: String,
    path: PathBuf,
    pub relics: Vec<CacheRelics>,
    pub relic_drops_real_names: HashMap<String, CacheItemComponent>,
}

impl RelicsModule {
    pub fn new(client: CacheClient) -> Self {
        RelicsModule {
            client,
            debug_id: "ch_client_auction".to_string(),
            component: "RelicsModule".to_string(),
            relic_drops_real_names: HashMap::new(),
            path: PathBuf::from("item/Relics.json"),
            relics: Vec::new(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_relics_module(self.clone());
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

        for warframe in self.client.parts().get_parts("Warframe") {
            let mut external_name = warframe.get_real_external_name();
            if (external_name.to_lowercase().contains("prime")
                || !(external_name != "Forma Blueprint"))
                && !external_name.contains("Nitain Extract")
                && !external_name.contains("Orokin Cell")
            {
                if (external_name.contains("Chassis")
                    || external_name.contains("Neuroptics")
                    || external_name.contains("Systems"))
                    && !external_name.contains("Blueprint")
                {
                    external_name += " Blueprint";
                }
                if external_name.contains("Odonata Prime")
                    && (external_name.contains("Harness")
                        || external_name.contains("Systems")
                        || external_name.contains("Wings"))
                    && !external_name.contains("Blueprint")
                {
                    external_name += " Blueprint";
                }
                self.relic_drops_real_names
                    .insert(external_name.to_lowercase().clone(), warframe.clone());
            }
        }
        for weapon in self.client.parts().get_parts("Weapon") {
            let mut external_name = weapon.get_real_external_name();
            if (external_name.to_lowercase().contains("prime")
                || !(external_name != "Forma Blueprint"))
                && !external_name.contains("Nitain Extract")
                && !external_name.contains("Orokin Cell")
            {
                if (external_name.contains("Chassis")
                    || external_name.contains("Neuroptics")
                    || external_name.contains("Systems"))
                    && !external_name.contains("Blueprint")
                {
                    external_name += " Blueprint";
                }
                self.relic_drops_real_names
                    .insert(external_name.to_lowercase().clone(), weapon.clone());
            }
        }
        for skin in self.client.parts().get_parts("Skin") {
            let external_name = skin.get_real_external_name();
            if external_name.to_lowercase().contains("kavasa prime") {
                self.relic_drops_real_names
                    .insert(external_name.to_lowercase().clone(), skin.clone());
            }
        }
        self.update_state();
        Ok(())
    }

    pub fn get_relic_by_name(&self, name: &str) -> Option<CacheItemComponent> {
        self.relic_drops_real_names
            .get(&name.to_lowercase())
            .cloned()
    }

    pub fn get_by(&self, input: &str, by: &str) -> Result<Option<CacheRelics>, AppError> {
        let items = self.relics.clone();
        let args = match helper::validate_args(by, vec!["--item_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--item_by").unwrap();
        let case_insensitive = args.get("--case_insensitive").is_some();
        // let lang = args.get("--item_lang").unwrap_or(&"en".to_string());
        let remove_string = args.get("--remove_string");

        let item = if mode == "name" {
            items
                .iter()
                .find(|x| helper::create_key(&x.name, case_insensitive, remove_string) == input)
                .cloned()
        } else if mode == "unique_name" {
            items
                .iter()
                .find(|x| {
                    helper::create_key(&x.unique_name, case_insensitive, remove_string) == input
                })
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