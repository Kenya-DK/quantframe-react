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
    component: String,
    path: PathBuf,
    data: CacheRiven,
}

impl RivenModule {
    pub fn new(client: CacheClient) -> Self {
        RivenModule {
            client,
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
        by: &str,
    ) -> Result<Option<CacheRivenWfmAttribute>, AppError> {
        let items = self.data.wfm_attributes.clone();
        let args = match helper::validate_args(
            by,
            vec![
                "--attribute_by",
                "--attribute_by:upgrades| --weapon_unique --upgrade_by",
            ],
        ) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--attribute_by").unwrap();

        let riven_attribute = if mode == "name" {
            items.iter().find(|x| x.effect == input).cloned()
        } else if mode == "url_name" {
            items.iter().find(|x| x.url_name == input).cloned()
        } else if mode == "upgrades" {
            let re = Regex::new(r"<.*?>").unwrap();
            let unique = args.get("--weapon_unique").unwrap();
            let upgrades = self.get_weapon_upgrades(unique);
            if upgrades.is_none() {
                return Err(AppError::new(
                    "find_rive_attribute_by",
                    eyre!("Failed to get weapon upgrades for: {}", unique),
                ));
            }

            let upgrades = upgrades
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<RivenStat>>();

            let upgrade_by = args.get("--upgrade_by").unwrap();
            match upgrade_by.as_str() {
                "short_string" => {
                    let upgrade = upgrades
                        .iter()
                        .find(|x| re.replace_all(&x.short_string, "").to_string() == input);
                    if upgrade.is_none() {
                        return Err(AppError::new(
                            "find_rive_attribute_by",
                            eyre!("Attribute not found: {}", input),
                        ));
                    }
                    let upgrade = upgrade.unwrap();
                    items.iter().find(|x| x.url_name == upgrade.wfm_id).cloned()
                }
                // If not found return an error
                _ => {
                    return Err(AppError::new(
                        "find_rive_attribute_by",
                        eyre!("Invalid upgrade_by value: {}", upgrade_by),
                    ));
                }
            }
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
        let args = match helper::validate_args(by, vec!["--weapon_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--weapon_by").unwrap();

        let riven_type = if mode == "name" {
            let lang: String = args.get("--weapon_lang").unwrap().clone();
            items
                .iter()
                .find(|x| x.i18_n[lang.as_str()].name == input)
                .cloned()
        } else if mode == "url_name" {
            items.iter().find(|x| x.wfm_url_name == input).cloned()
        } else if mode == "unique_name" {
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

        // Validate the riven type
        match self.find_rive_type_by(&input.raw, by) {
            Ok(weapon) => {
                if weapon.is_none() {
                    return Err(AppError::new(
                        component,
                        eyre!("Invalid riven type: {}", input.wfm_url),
                    ));
                }
                let weapon = weapon.unwrap();
                input.wfm_id = weapon.wfm_id.clone();
                input.wfm_url = weapon.wfm_url_name.clone();
                input.weapon_type = weapon.wfm_group.clone();
                input.weapon_unique_name = weapon.unique_name.clone();
                input.weapon_name = weapon.i18_n["en"].name.clone();
            }
            Err(e) => {
                return Err(e);
            }
        }

        if by.contains(&"--ignore_attributes".to_string()) {
            return Ok(input.clone());
        }

        let by = format!("{} --weapon_unique {}", by, input.weapon_unique_name);

        for att in input.attributes.iter_mut() {
            match self.find_rive_attribute_by(&att.url_name, &by) {
                Ok(attribute) => {
                    if attribute.is_none() {
                        return Err(AppError::new(
                            component,
                            eyre!("Invalid riven attribute: {}", att.url_name),
                        ));
                    }
                    let attribute = attribute.unwrap();
                    att.url_name = attribute.url_name.clone();
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(input.clone())
    }
}
