use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::log_parser::enums::trade_item_type::TradeItemType;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeItem {
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "unique_name")]
    #[serde(default)]
    pub unique_name: String,

    #[serde(rename = "sub_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "item_type")]
    pub item_type: TradeItemType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<(String, Value)>,
}
impl TradeItem {
    pub fn from_string(line: &str, is_platinum: bool) -> TradeItem {
        let mut raw = line.to_owned();

        if raw.contains(", leftItem=/") {
            raw.truncate(raw.find(", leftItem=/").unwrap());
        }

        // Get the quantity of the item
        let mut quantity = 1;
        if raw.contains(" x ") {
            let cloned = raw.clone();
            let parts: Vec<&str> = cloned.split(" x ").collect();
            raw = parts[0].to_string();
            quantity = parts[1].parse().unwrap_or(1);
        }
        raw = raw.trim().to_string();

        TradeItem {
            raw,
            quantity,
            unique_name: "".to_string(),
            item_type: if is_platinum {
                TradeItemType::Platinum
            } else {
                TradeItemType::Unknown
            },
            sub_type: None,
            error: None,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.raw.len() > 0
    }
    pub fn display(&self) -> String {
        format!("Name: {} | Quantity: {}", self.raw, self.quantity)
    }
}
