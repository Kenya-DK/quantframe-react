use serde::{Deserialize, Serialize};

use super::attribute::RivenAttribute;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockRiven {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    #[serde(rename = "mod_name")]
    pub mod_name: String,
    #[serde(rename = "weapon_type")]
    pub weapon_type: String,
    #[serde(rename = "mastery_rank")]
    pub mastery_rank: i64,
    #[serde(rename = "re_rolls")]
    pub re_rolls: i64,
    #[serde(rename = "polarity")]
    pub polarity: String,
    #[serde(rename = "attributes")]
    pub attributes: Vec<RivenAttribute>,
    #[serde(rename = "minimum_price")]
    pub minimum_price: i32,
    #[serde(rename = "rank")]
    pub rank: i32,
}
