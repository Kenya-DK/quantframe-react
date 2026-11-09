use serde::{Deserialize, Serialize};
use utils::{Error, Properties, SubType};

use crate::cache::{CacheState, CacheSyndicateTitle};
use crate::wf_inventory::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvItemBase {
    #[serde(rename = "id", default)]
    pub id: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "unique_name", default)]
    pub unique_name: String,

    #[serde(rename = "wfm_url", default)]
    pub wfm_url: String,

    #[serde(rename = "quantity", default)]
    pub quantity: i64,

    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    // Extra properties that may be needed for specific item types, stored as a JSON object
    #[serde(flatten)]
    pub properties: Properties,
}

impl WFInvItemBase {
    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query)
            || self.unique_name.to_lowercase().contains(&query)
    }
    pub fn uuid(&self) -> String {
        match &self.sub_type {
            Some(sub_type) => format!("{}-{}", self.unique_name, sub_type.shot_display()),
            None => self.unique_name.clone(),
        }
    }
}

impl Default for WFInvItemBase {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            unique_name: String::new(),
            wfm_url: String::new(),
            quantity: 1,
            sub_type: None,
            properties: Properties::default(),
        }
    }
}

//
// Conversions
//
impl WFInvItemBase {
    pub fn from_affiliation(
        affiliation: &WFInvAffiliation,
        cache: &CacheState,
    ) -> Result<Self, Error> {
        let syndicate = cache.syndicate().get_by(affiliation.id.clone())?;
        let title = syndicate
            .get_title_by_level(affiliation.rank as i32)
            .unwrap_or(CacheSyndicateTitle::default());
        let mut item = WFInvItemBase::default();

        let standing = affiliation.standing;
        let min_standing = if affiliation.rank < 0 {
            title.max_standing
        } else {
            title.min_standing
        };
        item.id = affiliation.id.clone();
        item.name = syndicate.name.clone();
        item.unique_name = syndicate.unique_name.clone();
        item.sub_type = Some(SubType::rank(affiliation.rank as i64));

        item.quantity = standing - min_standing;
        item.properties
            .set_property_value("total".to_string(), standing);
        item.properties
            .set_property_value("min_standing".to_string(), title.min_standing);
        item.properties
            .set_property_value("max_standing".to_string(), title.max_standing);
        item.properties
            .set_property_value("can_select".to_string(), syndicate.can_select);
        item.properties.set_property_value(
            "background_colour".to_string(),
            syndicate.background_colour.clone(),
        );
        item.properties
            .set_property_value("colour".to_string(), syndicate.icon_colour.clone());
        Ok(item)
    }
    pub fn from_mod(raw: &WFInvItemRaw, cache: &CacheState) -> Result<Self, Error> {
        let mut item = WFInvItemBase::default();
        item.id = raw.id.id.clone().unwrap_or_default();
        item.unique_name = raw.unique_name.clone();
        item.quantity = raw.quantity;

        let mod_rank = raw.get_upgrade_fingerprint().mod_rank;
        item.sub_type = Some(SubType::rank(mod_rank));

        // Resolve name/wfm_url from the mod cache; fall back to the raw path when the
        // cache does not cover the item (e.g. Railjack cards).
        if let Ok(mod_data) = cache.mods().get_by(raw.unique_name.clone()) {
            item.name = mod_data.base.name.clone();
            item.wfm_url = mod_data.base.wfm_url.unwrap_or_default();
            item.properties
                .set_property_value("max_rank", mod_data.fusion_limit);
        } else {
            item.name = raw.unique_name.rsplit('/').next().unwrap_or("").to_string();
        }
        Ok(item)
    }
}
