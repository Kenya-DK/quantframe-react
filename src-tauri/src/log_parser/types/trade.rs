use entity::enums::stock_type::StockType;
use serde::{Deserialize, Serialize};

use crate::{
    log_parser::enums::{
        trade_classification::TradeClassification, trade_item_type::TradeItemType,
    },
    utils::{
        enums::log_level::LogLevel,
        modules::{error::AppError, states, trading_helper::trace},
    },
    DATABASE,
};

use super::{create_stock_entity::CreateStockEntity, trade_item::TradeItem};

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

    pub fn is_set(&self) -> Result<(bool, String), AppError> {
        let trade_type = match self.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => {
                return Ok((false, "".to_string()));
            }
        };

        let main_part = self.get_item_by_type(&trade_type, &TradeItemType::MainBlueprint);
        if main_part.is_none() || self.get_valid_items(&self.trade_type).len() < 1 {
            return Ok((false, "".to_string()));
        }
        let main_part = main_part.unwrap();
        let cache = states::cache()?;
        // Get the set for the main part
        let main_part = match cache.all_items().get_by(
            &main_part.unique_name,
            "--item_by unique_name --category Component",
        )? {
            Some(set_part) => set_part,
            None => {
                trace(&format!(
                    "Main part not found for by: {}",
                    main_part.unique_name
                ));
                return Ok((false, "".to_string()));
            }
        };
        trace(&format!("Found main part {} for set", main_part.display()));

        // Get the set unique name if it exists
        let set_unique_name = match main_part.part_of_set {
            Some(set) => set,
            None => {
                trace(&format!(
                    "Part of set not found for by: {:?}",
                    main_part.part_of_set
                ));
                return Ok((false, "".to_string()));
            }
        };
        trace(&format!("Set unique name: {}", set_unique_name));
        // Get the set for the main part
        let set = match cache
            .all_items()
            .get_by(&set_unique_name, "--item_by unique_name")?
        {
            Some(set) => set,
            None => {
                trace(&format!("Set not found for by: {}", set_unique_name));
                return Ok((false, "".to_string()));
            }
        };
        trace(&format!("Found set {} for main part", set.display()));

        // Get the components for the set
        let components = set.get_tradable_components();
        if components.is_empty() {
            trace(&format!("Components not found for set: {}", set.display()));
            return Ok((false, "".to_string()));
        }

        for component in components {
            let found =
                self.is_item_in_trade(&trade_type, &component.unique_name, component.item_count);
            trace(&format!(
                "Component: {} | Found: {}",
                component.display(),
                found
            ));
            if !found {
                return Ok((false, "".to_string()));
            }
        }
        trace(&format!("Full set found: {}", set.display()));
        return Ok((true, set.unique_name));
    }

    pub async fn to_stock(&self) -> Result<CreateStockEntity, AppError> {
        let db = DATABASE.get().unwrap();
        let trade_type = match self.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => {
                return Err(AppError::new_with_level(
                    "PlayerTrade:ToStock",
                    eyre::eyre!("Wrong Trade Type: {:?}", self.trade_type),
                    LogLevel::Warning,
                ))
            }
        };
        let mut stock = CreateStockEntity::new("", self.platinum);
        let items = self.get_valid_items(&trade_type);
        // Check if the trade is a set
        let (is_set, set_name) = self.is_set()?;
        if is_set {
            stock.raw = set_name.to_string();
            stock.entity_type = StockType::Item;
        } else if items.len() == 1 {
            let item = items.first().unwrap();
            stock.raw = item.unique_name.clone();
            stock.quantity = item.quantity;
            stock.sub_type = item.sub_type.clone();
            // Set Stock Type
            stock.entity_type = match item.item_type {
                TradeItemType::RivenUnVeiled => StockType::Riven,
                _ => StockType::Item,
            };

            if stock.entity_type == StockType::Item
                && self.trade_type == TradeClassification::Purchase
            {
                stock.bought = Some(self.platinum / item.quantity);
            }
            if stock.entity_type == StockType::Riven {
                // Split by '/' and collect into a Vec
                let parts: Vec<&str> = item.unique_name.trim_matches('/').split('/').collect();
                // Get the last two elements
                match parts.get(parts.len() - 2) {
                    Some(riven_type) => {
                        stock.raw = riven_type.to_string();
                    }
                    None => {
                        return Err(AppError::new_with_level(
                            "PlayerTrade:ToStock",
                            eyre::eyre!("Riven type not found: {}", item.unique_name),
                            LogLevel::Warning,
                        ));
                    }
                }
                match parts.get(parts.len() - 1) {
                    Some(mod_name) => {
                        stock.mod_name = mod_name.to_string();
                    }
                    None => {
                        return Err(AppError::new_with_level(
                            "PlayerTrade:ToStock",
                            eyre::eyre!("Riven mod name not found: {}", item.unique_name),
                            LogLevel::Warning,
                        ));
                    }
                }
            }
        } else {
            let msg;
            if items.len() == 0 {
                msg = "No valid items found".to_string();
            } else if !set_name.is_empty() {
                msg = format!("Set Not valid: {}", set_name);
            } else {
                msg = format!("Multiple items found: {}, Skipping", items.len());
            }
            trace(&msg);
            return Err(AppError::new_with_level(
                "PlayerTrade:ToStock",
                eyre::eyre!("{} | Trade Type: {:?}", msg, trade_type),
                LogLevel::Warning,
            ));
        }
        stock.validate_entity(
            "--item_by unique_name --weapon_by name --weapon_lang en --ignore_attributes",
        )?;
        // Check if the stock is a wishlist item
        if stock.is_wish_list_item(db).await? {
            stock.entity_type = StockType::WishList;
        }
        return Ok(stock);
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
