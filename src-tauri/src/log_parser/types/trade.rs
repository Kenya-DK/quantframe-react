use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    log_parser::enums::{
        trade_classification::TradeClassification, trade_item_type::TradeItemType,
    },
    utils::modules::{error::AppError, states},
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
    pub fn get_valid_items(&self, trade_type: &TradeClassification) -> Vec<TradeItem> {
        let items = match trade_type.clone() {
            TradeClassification::Purchase => &self.offered_items,
            TradeClassification::Sale => &self.received_items,
            _ => &vec![],
        };
        items
            .iter()
            .filter(|p| p.item_type != TradeItemType::Unknown)
            .cloned()
            .collect()
    }
    pub fn is_item_in_trade(
        &self,
        trade_type: &TradeClassification,
        unique_name: &str,
        quantity: i64,
    ) -> bool {
        let items = self.get_valid_items(trade_type);
        items
            .iter()
            .any(|p| p.unique_name == unique_name && p.quantity == quantity)
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

        // Filter out unknown items
        let offered_items = self.get_valid_items(&TradeClassification::Purchase);
        let received_items = self.get_valid_items(&TradeClassification::Sale);

        if offer_plat > 1 && offered_items.len() == 1 {
            self.trade_type = TradeClassification::Purchase;
        } else if receive_plat > 1 && received_items.len() == 1 {
            self.trade_type = TradeClassification::Sale;
        } else {
            self.trade_type = TradeClassification::Trade;
        }
    }

    pub fn get_item_by_type(
        &self,
        trade_type: &TradeClassification,
        item_type: &TradeItemType,
    ) -> Option<TradeItem> {
        let items = self.get_valid_items(trade_type);
        items.iter().find(|p| &p.item_type == item_type).cloned()
    }

    pub fn calculate_set(&mut self) -> Result<(bool, String), AppError> {
        let trade_type = match self.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => {
                return Ok((false, "Not a trade".to_string()));
            }
        };

        let main_part = self.get_item_by_type(&trade_type, &TradeItemType::MainBlueprint);
        if main_part.is_none() || self.get_valid_items(&self.trade_type).len() < 1 {
            return Ok((false, "Not a set".to_string()));
        }

        let main_part = main_part.unwrap();
        let cache = states::cache()?;
        // Get the set for the main part
        let main_part = match cache
            .all_items()
            .get_by(&main_part.unique_name, "--item_by unique_name")?
        {
            Some(set_part) => set_part,
            None => {
                return Ok((
                    false,
                    format!("Main part not found for by: {}", main_part.unique_name),
                ))
            }
        };

        // Get the set for the main part
        let part_of_set = match main_part.part_of_set {
            Some(set) => cache.all_items().get_by(&set, "--item_by unique_name")?,
            None => {
                return Ok((
                    false,
                    format!("Set not found for by: {}", main_part.unique_name),
                ))
            }
        };
        let part_of_set = part_of_set.unwrap();

        // Get the components for the set
        let components = part_of_set.get_tradable_components();
        if components.is_empty() {
            return Ok((
                false,
                format!("Components is empty for: {}", main_part.unique_name),
            ));
        }

        for component in components {
            if !self.is_item_in_trade(&trade_type, &component.unique_name, component.item_count) {
                return Ok((
                    false,
                    format!("Component not found for: {}", component.display()),
                ));
            }
        }
        println!("Set: {:?}", part_of_set.wfm_item_url);
        Ok((true, part_of_set.wfm_item_url.unwrap()))
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
