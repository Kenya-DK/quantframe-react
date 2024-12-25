use std::path::PathBuf;

use entity::stock::riven::create::CreateStockRiven;
use eyre::eyre;
use regex::Regex;

use crate::{
    cache::{
        client::CacheClient,
        types::cache_riven::{
            CacheRiven, CacheRivenAttribute, CacheRivenUpgrade, CacheRivenWFMAttribute,
            CacheRivenWeapon,
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
            path: PathBuf::from("items/Riven.json"),
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

    // WFM Rivens Methods
    pub fn get_wfm_riven_types(&self) -> Result<Vec<CacheRivenWeapon>, AppError> {
        let items = self.data.weapons.clone();
        Ok(items)
    }

    pub fn get_wfm_riven_attributes(&self) -> Result<Vec<CacheRivenWFMAttribute>, AppError> {
        let attributes = self.data.available_attributes.clone();
        Ok(attributes)
    }

    pub fn get_weapon_upgrades(
        &self,
        internal_id: &str,
    ) -> Result<Option<Vec<CacheRivenUpgrade>>, AppError> {
        // Get the weapon stat
        let weapon_stat = self.find_rive_type_by(internal_id, "--weapon_by unique_name")?;
        if weapon_stat.is_none() {
            logger::warning_con(
                self.get_component("get_weapon_upgrades").as_str(),
                format!("Failed to get weapon stat for internal_id: {}", internal_id).as_str(),
            );
            return Ok(None);
        }
        let weapon_stat = weapon_stat.unwrap();
        let raw_riven =
            self.find_raw_riven_attribute_by(&weapon_stat.upgrade_type, "--weapon_by unique_name")?;
        if raw_riven.is_none() {
            logger::warning_con(
                self.get_component("get_weapon_upgrades").as_str(),
                format!("Failed to get raw riven for internal_id: {}", internal_id).as_str(),
            );
            return Ok(None);
        }
        let raw_riven = raw_riven.unwrap();
        let upgrades = raw_riven.upgrades.clone();
        Ok(Some(upgrades))
    }

    pub fn find_wfm_riven_attribute_by(
        &self,
        input: &str,
        by: &str,
    ) -> Result<Option<CacheRivenWFMAttribute>, AppError> {
        let items = self.data.available_attributes.clone();
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
            let upgrades = self.get_weapon_upgrades(unique)?;
            if upgrades.is_none() {
                return Err(AppError::new(
                    "find_rive_attribute_by",
                    eyre!("Failed to get weapon upgrades for: {}", unique),
                ));
            }

            let upgrades = upgrades.unwrap();
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
                    items
                        .iter()
                        .find(|x| x.url_name == upgrade.wfm_url)
                        .cloned()
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

    pub fn find_raw_riven_attribute_by(
        &self,
        input: &str,
        by: &str,
    ) -> Result<Option<CacheRivenAttribute>, AppError> {
        let items = self.data.rivens_attributes.clone();
        let args = match helper::validate_args(by, vec!["--weapon_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--weapon_by").unwrap();

        let riven_type = if mode == "unique_name" {
            items.iter().find(|x| x.unique_name == input).cloned()
        } else {
            return Err(AppError::new(
                "get_rive_type_by",
                eyre!("Invalid by value: {}", by),
            ));
        };
        Ok(riven_type)
    }

    pub fn find_rive_type_by(
        &self,
        input: &str,
        by: &str,
    ) -> Result<Option<CacheRivenWeapon>, AppError> {
        let items: Vec<CacheRivenWeapon> = self.data.weapons.clone();
        let args = match helper::validate_args(by, vec!["--weapon_by"]) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let mode = args.get("--weapon_by").unwrap();

        let riven_type = if mode == "name" {
            // let lang: String = args.get("--weapon_lang").unwrap().clone();
            items.iter().find(|x| x.name == input).cloned()
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
                        eyre!("Invalid riven type: {} by: {}", input.raw, by),
                    ));
                }
                let weapon = weapon.unwrap();
                input.wfm_id = weapon.wfm_id.clone();
                input.wfm_url = weapon.wfm_url_name.clone();
                input.weapon_type = weapon.wfm_group.clone();
                input.weapon_unique_name = weapon.unique_name.clone();
                input.weapon_name = weapon.name.clone();
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
            match self.find_wfm_riven_attribute_by(&att.url_name, &by) {
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
