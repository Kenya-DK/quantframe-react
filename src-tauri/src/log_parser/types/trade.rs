use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use utils::Error;

use crate::{log_parser::*, utils::modules::states};

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

        add_to_zip(format!(
            "Calculating Trade | Offered Plat: {} | Received Plat: {}",
            offer_plat, receive_plat
        ));

        if offer_plat > 0 {
            self.platinum = offer_plat;
            add_to_zip(format!("Platinum set from Offer: {}", self.platinum));
        }
        if receive_plat > 0 {
            self.platinum = receive_plat;
            add_to_zip(format!("Platinum set from Receive: {}", self.platinum));
        }

        // Filter out unknown items
        let offered_items = self.get_valid_items(&TradeClassification::Purchase);
        let received_items = self.get_valid_items(&TradeClassification::Sale);

        add_to_zip(format!(
            "Filtered Items | Offered: {} | Received: {}",
            offered_items.len(),
            received_items.len()
        ));

        if offer_plat > 1 && offered_items.len() == 1 {
            self.trade_type = TradeClassification::Purchase;
            add_to_zip(format!(
                "Classified Trade as Purchase | Item: {:?}",
                offered_items.first()
            ));
        } else if receive_plat > 1 && received_items.len() == 1 {
            self.trade_type = TradeClassification::Sale;
            add_to_zip(format!(
                "Classified Trade as Sale | Item: {:?}",
                received_items.first()
            ));
        } else {
            self.trade_type = TradeClassification::Trade;
            add_to_zip("Classified Trade as Regular Item-for-Item Trade".to_string());
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
    pub fn is_set(&self) -> Result<(bool, String), Error> {
        let trade_type = match self.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => {
                return Ok((false, "Trade type is not Sale or Purchase".to_string()));
            }
        };

        let main_part = self.get_item_by_type(&trade_type, &TradeItemType::MainBlueprint);
        if main_part.is_none() || self.get_valid_items(&self.trade_type).len() < 1 {
            return Ok((false, "".to_string()));
        }
        let main_part = main_part.unwrap();
        let cache = states::cache_client()?;
        // Get the set for the main part

        let main_part = match cache
            .all_items()
            .get_by(format!("Component|{}", &main_part.unique_name))
        {
            Ok(part) => part,
            Err(_) => {
                add_to_zip(&format!(
                    "Main part not found for by: Component|{}",
                    &main_part.unique_name
                ));
                return Ok((false, "".to_string()));
            }
        };
        add_to_zip(&format!("Found main part {} for set", main_part.display()));

        // Get the set unique name if it exists
        let set_unique_name = match main_part.part_of_set {
            Some(set) => set,
            None => {
                add_to_zip(&format!(
                    "Part of set not found for by: {:?}",
                    main_part.part_of_set
                ));
                return Ok((false, "".to_string()));
            }
        };
        add_to_zip(&format!("Set unique name: {}", set_unique_name));
        // Get the set for the main part
        let set = match cache.all_items().get_by(&set_unique_name) {
            Ok(set) => set,
            Err(_) => {
                add_to_zip(&format!("Set not found for by: {}", set_unique_name));
                return Ok((false, "".to_string()));
            }
        };
        add_to_zip(&format!("Found set {} for main part", set.display()));

        // Get the components for the set
        let components = set.get_tradable_components();
        if components.is_empty() {
            add_to_zip(&format!("Components not found for set: {}", set.display()));
            return Ok((false, "".to_string()));
        }

        for component in components {
            let found =
                self.is_item_in_trade(&trade_type, &component.unique_name, component.item_count);
            add_to_zip(&format!(
                "Component: {} | Found: {}",
                component.display(),
                found
            ));
            if !found {
                return Ok((false, "".to_string()));
            }
        }
        add_to_zip(&format!("Full set found: {}", set.display()));
        return Ok((true, set.unique_name));
    }

    pub fn get_notify_variables(&self) -> HashMap<String, String> {
        let offered_items = self
            .get_valid_items(&TradeClassification::Purchase)
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");

        let received_items = self
            .get_valid_items(&TradeClassification::Sale)
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");

        return HashMap::from([
            (
                "<TR_TYPE>".to_string(),
                self.trade_type.to_str().to_string(),
            ),
            ("<PLAYER_NAME>".to_string(), self.player_name.clone()),
            (
                "<OF_COUNT>".to_string(),
                self.offered_items.len().to_string(),
            ),
            ("<OF_ITEMS>".to_string(), offered_items),
            (
                "<RE_COUNT>".to_string(),
                self.received_items.len().to_string(),
            ),
            ("<TIME>".to_string(), self.trade_time.clone()),
            ("<RE_ITEMS>".to_string(), received_items),
            ("<LOGS>".to_string(), self.logs.join("\n")),
            ("<TOTAL_PLAT>".to_string(), self.platinum.to_string()),
        ]);
    }
}
impl Display for PlayerTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerTrade ")?;
        if self.player_name.is_empty() {
            write!(f, "Player Name: Not provided | ")?;
        } else {
            write!(f, "Player Name: {} | ", self.player_name)?;
        }
        write!(f, "Trade Time: {} | ", self.trade_time)?;
        write!(f, "Type: {:?} | ", self.trade_type)?;
        write!(f, "Platinum: {} | ", self.platinum)?;
        write!(f, "Offered Items: {} | ", self.offered_items.len())?;
        write!(f, "Received Items: {} | ", self.received_items.len())?;
        write!(f, "Logs: {} entries", self.logs.len())?;
        Ok(())
    }
}
