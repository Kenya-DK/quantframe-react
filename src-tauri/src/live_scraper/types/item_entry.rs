use std::hash::{Hash, Hasher};

use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stock_id")]
    pub stock_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wish_list_id")]
    pub wish_list_id: Option<i64>,

    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_type")]
    pub sub_type: Option<SubType>,

    // Trading Stats.
    #[serde(default)]
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(default)]
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: i64,

    #[serde(default)]
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: i64,

    #[serde(rename = "operation")]
    #[serde(default)]
    pub operation: Vec<String>,

    #[serde(rename = "order_type")]
    #[serde(default)]
    pub order_type: String,
}

impl Hash for ItemEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wfm_url.hash(state);
        self.sub_type.hash(state);
    }
}

impl ItemEntry {
    pub fn new(
        stock_id: Option<i64>,
        wish_list_id: Option<i64>,
        wfm_url: String,
        sub_type: Option<SubType>,
        priority: i64,
        buy_quantity: i64,
        sell_quantity: i64,
        operation: Vec<String>,
        order_type: &str,
    ) -> ItemEntry {
        ItemEntry {
            stock_id,
            wish_list_id,
            wfm_url,
            sub_type,
            priority,
            buy_quantity,
            sell_quantity,
            operation,
            order_type: order_type.to_string(),
        }
    }
}
