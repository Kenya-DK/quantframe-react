use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::log_parser::enums::trade_classification::TradeClassification;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlayerTrade {
    #[serde(rename = "playerName")]
    pub player_name: String,
    #[serde(rename = "tradeTime")]
    pub trade_time: String,
    #[serde(rename = "type")]
    pub trade_type: TradeClassification,
    #[serde(rename = "platinum")]
    pub platinum: i64,
    #[serde(rename = "offeredItems")]
    pub offered_items: Vec<TradeItem>,
    #[serde(rename = "receivedItems")]
    pub received_items: Vec<TradeItem>,

    // Used for debugging
    #[serde(rename = "file_logs")]
    pub file_logs: Vec<String>,
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
}

impl Default for PlayerTrade {
    fn default() -> Self {
        PlayerTrade {
            player_name: "".to_string(),
            trade_time: "".to_string(),
            trade_type: TradeClassification::Unknown,
            platinum: 0,
            offered_items: vec![],
            received_items: vec![],
            file_logs: vec![],
            logs: vec![],
        }
    }
}

impl PlayerTrade {
    pub fn new(
        player_name: String,
        trade_time: String,
        trade_type: TradeClassification,
        platinum: i64,
        offered_items: Vec<TradeItem>,
        received_items: Vec<TradeItem>,
    ) -> Self {
        PlayerTrade {
            player_name,
            trade_time,
            trade_type,
            platinum,
            offered_items,
            received_items,
            file_logs: vec![],
            logs: vec![],
        }
    }
    pub fn get_received_plat(&self) -> i64 {
        self.received_items
            .iter()
            .filter(|p| p.name == "Platinum")
            .map(|p| p.quantity)
            .sum::<i64>()
    }
    pub fn get_offered_plat(&self) -> i64 {
        self.offered_items
            .iter()
            .filter(|p| p.name == "Platinum")
            .map(|p| p.quantity)
            .sum::<i64>()
    }
    pub fn display(&self) -> String {
        format!(
            "Player: {} | Time: {} | Type: {:?} | Platinum: {} | Offered: {} | Received: {}",
            self.player_name,
            self.trade_time,
            self.trade_type,
            self.platinum,
            self.offered_items.len(),
            self.received_items.len()
        )
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "unique_name")]
    #[serde(default)]
    pub unique_name: String,

    #[serde(rename = "item_type")]
    #[serde(default)]
    pub item_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_type")]
    pub sub_type: Option<SubType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wfm_url")]
    pub wfm_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<(String, Value)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "properties")]
    pub properties: Option<Value>,
}
impl TradeItem {
    pub fn new(
        name: String,
        quantity: i64,
        unique_name: String,
        sub_type: Option<SubType>,
        wfm_id: Option<String>,
        wfm_url: Option<String>,
        source: String,
        properties: Option<Value>,
    ) -> Self {
        TradeItem {
            name,
            quantity,
            unique_name,
            sub_type,
            wfm_id,
            wfm_url,
            error: None,
            item_type: source,
            properties,
        }
    }
    pub fn display(&self) -> String {
        format!(
            "Name: {} | Quantity: {} | Unique Name: {} | Source: {}",
            self.name, self.quantity, self.unique_name, self.item_type
        )
    }
}
