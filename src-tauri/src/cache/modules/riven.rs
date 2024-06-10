use std::{collections::HashMap, path::PathBuf};

use entity::stock::riven::create::CreateStockRiven;
use eyre::eyre;
use regex::Regex;


use crate::{
    cache::{
        client::CacheClient,
        types::cache_riven::{
            CacheRiven, CacheRivenDataByRivenInternalID, CacheRivenWfmAttribute,
            CacheRivenWfmWeapon, CacheWeaponStat, RivenStat,
        },
    },
    helper, logger,
    utils::modules::error::AppError,
};
#[derive(Clone, Debug)]
pub struct RivenModule {
    pub client: CacheClient,
    debug_id: String,
    component: String,
    path: PathBuf,
    data: CacheRiven,
}

impl RivenModule {
    pub fn new(client: CacheClient) -> Self {
        RivenModule {
            client,
            debug_id: "ch_client_auction".to_string(),
            path: PathBuf::from("riven/rivens.json"),
            data: CacheRiven::new(),
            component: "RivenModule".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_riven_module(self.clone());
    }
    pub fn load(&mut self) -> Result<(), AppError> {
        let content = self.client.read_text_from_file(&self.path)?;
        let data: CacheRiven = serde_json::from_str(&content).map_err(|e| {
            AppError::new(
                self.get_component("Load").as_str(),
                eyre!(format!("Failed to parse RivenModule from file: {}", e)),
            )
        })?;
        self.data = data.clone();
        self.update_state();
        Ok(())
    }

    pub fn get_riven_raw_mod(&self, internal_id: &str) -> Option<&CacheRivenDataByRivenInternalID> {
        let riven = self.data.riven_internal_id.get(internal_id);
        riven
    }

    pub fn get_weapon_stat(&self, internal_id: &str) -> Option<&CacheWeaponStat> {
        let weapon = self.data.weapon_stat.get(internal_id);
        weapon
    }

    pub fn get_weapon_upgrades(&self, internal_id: &str) -> Option<HashMap<String, RivenStat>> {
        // Get the weapon stat
        let weapon_stat = self.get_weapon_stat(internal_id);
        if weapon_stat.is_none() {
            logger::warning_con(
                self.get_component("get_weapon_upgrades").as_str(),
                format!("Failed to get weapon stat for internal_id: {}", internal_id).as_str(),
            );
            return None;
        }
        let weapon_stat = weapon_stat.unwrap();
        let raw_riven = self.get_riven_raw_mod(&weapon_stat.riven_uid);
        if raw_riven.is_none() {
            logger::warning_con(
                self.get_component("get_weapon_upgrades").as_str(),
                format!("Failed to get raw riven for internal_id: {}", internal_id).as_str(),
            );
            return None;
        }
        let raw_riven = raw_riven.unwrap();
        let upgrades = raw_riven.riven_stats.clone();
        upgrades
    }

    // WFM Rivens Methods
    pub fn get_wfm_riven_types(&self) -> Result<Vec<CacheRivenWfmWeapon>, AppError> {
        let items = self.data.wfm_weapons.clone();
        Ok(items)
    }

    pub fn get_wfm_riven_attributes(&self) -> Result<Vec<CacheRivenWfmAttribute>, AppError> {
        let attributes = self.data.wfm_attributes.clone();
        Ok(attributes)
    }


    pub fn find_rive_attribute_by(
        &self,
        input: &str,
        by: &str
    ) -> Result<Option<CacheRivenWfmAttribute>, AppError> {
        let items = self.data.wfm_attributes.clone();
        let args = helper::parse_args_from_string(by);
        let mode = args.get("--attribute_by");
        if mode.is_none() {
            return Err(AppError::new(
                "get_rive_attribute_by",
                eyre!("Missing attribute_by argument"),
            ));
        }
        let mode = mode.unwrap();

        let riven_attribute = if mode =="name" {
            items.iter().find(|x| x.effect == input).cloned()
        } else if by == "url_name" {
            items.iter().find(|x| x.url_name == input).cloned()
        } else {
            return Err(AppError::new(
                "get_rive_attribute_by",
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(riven_attribute)
    }

    pub fn find_rive_type_by(
        &self,
        input: &str,
        by: &str,
    ) -> Result<Option<CacheRivenWfmWeapon>, AppError> {
        let items = self.data.wfm_weapons.clone();
        let args = helper::parse_args_from_string(by);
        let mode = args.get("--weapon_by");
        if mode.is_none() {
            return Err(AppError::new(
                "get_rive_type_by",
                eyre!("Missing weapon_by argument"),
            ));
        }
        let mode = mode.unwrap();

        let riven_type = if mode =="name" {
            let lang = args.get("--weapon_lang");
            if lang.is_none() {
                return Err(AppError::new(
                    "get_rive_type_by",
                    eyre!("Missing weapon_lang argument"),
                ));
            }
            items.iter().find(|x| x.i18_n[lang.unwrap()].name == input).cloned()
        } else if by == "url_name" {
            items.iter().find(|x| x.wfm_url_name == input).cloned()
        } else if by == "unique_name" {
             items.iter().find(|x| x.unique_name == input).cloned()
        } else {
            return Err(AppError::new(
                "get_rive_type_by",
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(riven_type)
    }

    pub fn validate_create_riven(
        &self,
        input: &mut CreateStockRiven,
        by: &str,
    ) -> Result<CreateStockRiven, AppError> {
        let component = "ValidateCreateRiven";

        let args = helper::parse_args_from_string(by);
        let mode = args.get("--weapon_by");
        if mode.is_none() {
            return Err(AppError::new(
                component,
                eyre!("Missing weapon_by argument"),
            ));
        }
        let mode = mode.unwrap();

        let attribute_by = args.get("--attribute_by");
        if attribute_by.is_none() {
            return Err(AppError::new(
                component,
                eyre!("Missing attribute_by argument"),
            ));
        }
        let attribute_by = attribute_by.unwrap();

        let weapon = self.find_rive_type_by(&input.wfm_url, by)?;
        if weapon.is_none() {
            return Err(AppError::new(
                component,
                eyre!("Invalid mode value: {}", mode),
            ));
        }
        let weapon = weapon.unwrap();
        input.wfm_id = weapon.wfm_id.clone();
        input.wfm_url = weapon.wfm_url_name.clone();
        input.weapon_type = weapon.wfm_group.clone();
        input.weapon_unique_name = weapon.unique_name.clone();
        input.weapon_name = weapon.i18_n["en"].name.clone();

        let upgrades = self.get_weapon_upgrades(&weapon.unique_name);
        if upgrades.is_none() {
            return Err(AppError::new(
                component,
                eyre!("Failed to get weapon upgrades for: {}", weapon.unique_name),
            ));
        }
        let upgrades = upgrades.unwrap().values().cloned().collect::<Vec<RivenStat>>();

        for att in input.attributes.iter_mut() {
            if attribute_by == "name" || attribute_by == "url_name" {
                
            } else if attribute_by == "upgrades" {
                let re = Regex::new(r"<.*?>").unwrap();
                let upgrade = upgrades.iter().find(|x| re.replace_all(&x.short_string, "").to_string() == att.url_name);
                if upgrade.is_none() {
                    return Err(AppError::new(
                        component,
                        eyre!("Attribute not found: {}", att.url_name),
                    ));
                }
                att.url_name = upgrade.unwrap().wfm_id.clone();
            }  else {
                return Err(AppError::new(
                    component,
                    eyre!("Invalid attribute_by value: {}", attribute_by),
                ));
            }
        }
        Ok(input.clone())
    }
}
