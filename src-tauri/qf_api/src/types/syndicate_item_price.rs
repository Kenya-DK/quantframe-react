use serde::{Deserialize, Serialize};

use crate::types::SubType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyndicateItemPrice {
    #[serde(default, rename = "name")]
    pub name: String,
    #[serde(default, rename = "syndicate")]
    pub syndicate: String,
    #[serde(default, rename = "standingCost")]
    pub standing_cost: i64,
    #[serde(default, rename = "wfmId")]
    pub wfm_id: String,
    #[serde(default, rename = "subType")]
    pub sub_type: Option<SubType>,
    #[serde(default, rename = "maxPrice")]
    pub max_price: f64,
    #[serde(default, rename = "minPrice")]
    pub min_price: f64,
    #[serde(default, rename = "volume")]
    pub volume: f64,
}
