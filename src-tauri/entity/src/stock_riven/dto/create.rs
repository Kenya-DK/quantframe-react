use serde::{Deserialize, Serialize};

use super::attribute::RivenAttribute;
use crate::stock_riven::*;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockRiven {
    // Properties use for validation
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(rename = "mastery_rank")]
    pub mastery_rank: i64,

    #[serde(rename = "re_rolls")]
    pub re_rolls: i64,

    #[serde(rename = "polarity")]
    pub polarity: String,

    #[serde(rename = "attributes")]
    pub attributes: Vec<RivenAttribute>,
    #[serde(rename = "rank")]
    pub rank: i64,

    #[serde(rename = "mod_name")]
    pub mod_name: String,

    // Set By validation method
    #[serde(rename = "wfm_id")]
    #[serde(default = "String::default")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url")]
    #[serde(default = "String::default")]
    pub wfm_url: String,

    #[serde(default = "String::default")]
    #[serde(rename = "weapon_name")]
    pub weapon_name: String,

    #[serde(rename = "weapon_unique_name")]
    #[serde(default = "String::default")]
    pub weapon_unique_name: String,

    #[serde(rename = "weapon_type")]
    #[serde(default = "String::default")]
    pub weapon_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,

    #[serde(rename = "is_validated")]
    #[serde(default = "bool::default")]
    pub is_validated: bool,
}

impl CreateStockRiven {
    pub fn new(
        raw: String,
        mod_name: String,
        mastery_rank: i64,
        re_rolls: i64,
        polarity: String,
        attributes: Vec<RivenAttribute>,
        rank: i64,
    ) -> Self {
        CreateStockRiven {
            raw,
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            mod_name,
            weapon_name: "".to_string(),
            weapon_unique_name: "".to_string(),
            weapon_type: "".to_string(),
            mastery_rank,
            re_rolls,
            polarity,
            attributes,
            minimum_price: None,
            bought: None,
            rank,
            is_validated: false,
        }
    }
    pub fn set_bought(mut self, bought: i64) -> Self {
        self.bought = Some(bought);
        self
    }
    pub fn to_model(&self) -> Model {
        Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.weapon_name.clone(),
            self.weapon_type.clone(),
            self.weapon_unique_name.clone(),
            self.rank,
            self.mod_name.clone(),
            super::attribute::RivenAttributeVec(self.attributes.clone()),
            self.mastery_rank,
            self.re_rolls,
            self.polarity.clone(),
            self.bought.unwrap_or(0),
            self.minimum_price,
            false,
            "".to_string(),
        )
    }
}
