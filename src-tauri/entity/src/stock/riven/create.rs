use serde::{Deserialize, Serialize};

use super::attribute::RivenAttribute;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockRiven {
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(rename = "wfm_id")]
    #[serde(default = "String::default")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url")]
    #[serde(default = "String::default")]
    pub wfm_url: String,

    #[serde(rename = "mod_name")]
    pub mod_name: String,

    #[serde(default = "String::default")]
    #[serde(rename = "weapon_name")]
    pub weapon_name: String,

    #[serde(rename = "weapon_unique_name")]
    #[serde(default = "String::default")]
    pub weapon_unique_name: String,

    #[serde(rename = "weapon_type")]
    #[serde(default = "String::default")]
    pub weapon_type: String,

    #[serde(rename = "mastery_rank")]
    pub mastery_rank: i64,

    #[serde(rename = "re_rolls")]
    pub re_rolls: i64,

    #[serde(rename = "polarity")]
    pub polarity: String,

    #[serde(rename = "attributes")]
    pub attributes: Vec<RivenAttribute>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,

    #[serde(rename = "rank")]
    pub rank: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_order_id: Option<String>,
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
        bought: Option<i64>,
        wfm_order_id: Option<String>,
        stock_id: Option<i64>,
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
            bought,
            rank,
            stock_id,
            wfm_order_id,
        }
    }
    pub fn new_valid(
        raw: String,
        wfm_id: String,
        wfm_url: String,
        weapon_name: String,
        weapon_unique_name: String,
        weapon_type: String,
        mod_name: String,
        mastery_rank: i64,
        re_rolls: i64,
        polarity: String,
        attributes: Vec<RivenAttribute>,
        minimum_price: Option<i64>,
        bought: Option<i64>,
        rank: i64,
        order_id: Option<String>,
        stock_id: Option<i64>,
    ) -> Self {
        CreateStockRiven {
            raw,
            wfm_id,
            wfm_url,
            weapon_name,
            weapon_unique_name,
            weapon_type,
            mod_name,
            mastery_rank,
            re_rolls,
            polarity,
            attributes,
            minimum_price,
            bought,
            rank,
            stock_id,
            wfm_order_id: order_id,
        }
    }
    pub fn to_stock(&self) -> super::stock_riven::Model {
        super::stock_riven::Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.wfm_order_id.clone(),
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
