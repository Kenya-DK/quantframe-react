use std::{collections::HashMap, path::PathBuf, sync::Arc};

use eyre::eyre;
use serde_json::json;

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
    wfm_client::types::{riven_attribute_info::RivenAttributeInfo, riven_type_info::RivenTypeInfo},
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

    pub fn get_riven_raw_mod(
        &self,
        internal_id: &str,
    ) -> Option<&CacheRivenDataByRivenInternalID> {
        let riven = self.data.riven_internal_id.get(internal_id);
        riven
    }

    pub fn get_weapon_stat(
        &self,
        internal_id: &str,
    ) -> Option<&CacheWeaponStat> {
        let weapon = self.data.weapon_stat.get(internal_id);
        weapon
    }
    
    pub fn get_weapon_upgrades(
        &self,
        internal_id: &str,
    ) -> Option<HashMap<String, RivenStat>> {
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

    pub fn get_wfm_riven_type_by_name(&self, name: &str, i18_n: &str) -> Option<CacheRivenWfmWeapon> {
        let items = self.data.wfm_weapons.clone();
        let item = items.iter().find(|item| item.i18_n[i18_n].name == name);
        match item {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    pub fn find_riven_type_by_url_name(&self, url_name: &str) -> Option<CacheRivenWfmWeapon> {
        let items = self.data.wfm_weapons.clone();
        let item = items.iter().find(|item| item.wfm_url_name == url_name);
        match item {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    pub fn get_wfm_riven_attributes(&self) -> Result<Vec<CacheRivenWfmAttribute>, AppError> {
        let attributes = self.data.wfm_attributes.clone();
        Ok(attributes)
    }

    pub fn find_riven_attribute_by_url_name(
        &self,
        url_name: &str,
    ) -> Option<CacheRivenWfmAttribute> {
        let attributes = self.data.wfm_attributes.clone();
        let attribute = attributes
            .iter()
            .find(|attribute| attribute.url_name == url_name);
        match attribute {
            Some(attribute) => Some(attribute.clone()),
            None => None,
        }
    }

}
