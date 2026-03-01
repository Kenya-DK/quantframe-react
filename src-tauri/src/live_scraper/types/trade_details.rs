use entity::dto::{PriceHistory, SubType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::Properties;

use crate::types::OperationSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeDetails {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    pub sub_type: Option<SubType>,

    #[serde(rename = "image", default)]
    pub image: String,

    #[serde(rename = "quantity", default)]
    pub quantity: u32,

    #[serde(rename = "profit", default)]
    pub profit: i64,

    #[serde(rename = "closed_avg", default)]
    pub closed_avg: f64,

    #[serde(rename = "lowest_price", default)]
    pub lowest_price: i64,

    #[serde(rename = "highest_price", default)]
    pub highest_price: i64,

    #[serde(rename = "update_string", default)]
    pub update_string: String,

    #[serde(rename = "operation", default, flatten)]
    pub operations: OperationSet,

    #[serde(rename = "trades", default)]
    pub trades: Value,

    #[serde(rename = "price_history", default)]
    pub price_history: Vec<PriceHistory>,

    // Extra properties
    #[serde(flatten)]
    pub properties: Properties,
}
impl TradeDetails {}

impl Default for TradeDetails {
    fn default() -> Self {
        Self {
            id: String::new(),
            wfm_id: String::new(),
            sub_type: None,
            quantity: 0,
            profit: 0,
            closed_avg: 0.0,
            lowest_price: 0,
            highest_price: 0,
            update_string: String::new(),
            operations: OperationSet::from(vec!["Create"]),
            trades: Value::Null,
            name: String::new(),
            image: String::new(),
            price_history: Vec::new(),
            properties: Properties::default(),
        }
    }
}
