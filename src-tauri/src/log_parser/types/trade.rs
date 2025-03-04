use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::log_parser::enums::{
    trade_classification::TradeClassification, trade_item_type::TradeItemType,
};

use super::trade_item::TradeItem;

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
    pub fn get_received_plat(&self) -> i64 {
        self.received_items
            .iter()
            .filter(|p| p.item_type == TradeItemType::Platinum)
            .map(|p| p.quantity)
            .sum::<i64>()
    }
    pub fn get_offered_plat(&self) -> i64 {
        self.offered_items
            .iter()
            .filter(|p| p.item_type == TradeItemType::Platinum)
            .map(|p| p.quantity)
            .sum::<i64>()
    }
    pub fn calculate(&mut self) {
        let offer_plat = self.get_offered_plat();
        let receive_plat = self.get_received_plat();
        if offer_plat > 0 {
            self.platinum = offer_plat;
        }
        if receive_plat > 0 {
            self.platinum = receive_plat;
        }
        if offer_plat > 1 && self.offered_items.len() == 1 {
            self.trade_type = TradeClassification::Purchase;
        } else if receive_plat > 1 && self.received_items.len() == 1 {
            self.trade_type = TradeClassification::Sale;
        } else {
            self.trade_type = TradeClassification::Trade;
        }
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
