use std::{collections::HashMap, fmt::Display, vec};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use utils::Error;

use crate::{log_parser::*, types::OperationSet, utils::modules::states};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlayerTrade {
    #[serde(rename = "playerName")]
    pub player_name: String,
    #[serde(rename = "tradeTime")]
    pub trade_time: DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub trade_type: TradeClassification,
    #[serde(rename = "platinum")]
    pub platinum: i64,
    #[serde(rename = "credits")]
    pub credits: i64,
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
            trade_time: chrono::Utc::now(),
            trade_type: TradeClassification::Unknown,
            platinum: 0,
            credits: 0,
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
    pub fn get_valid_items(
        &self,
        trade_type: &TradeClassification,
        mut exclude_types: Vec<TradeItemType>,
    ) -> Vec<TradeItem> {
        exclude_types.push(TradeItemType::Unknown);
        let items = match trade_type.clone() {
            TradeClassification::Purchase => &self.offered_items,
            TradeClassification::Sale => &self.received_items,
            _ => &vec![],
        };
        items
            .iter()
            .filter(|p| !exclude_types.contains(&p.item_type))
            .cloned()
            .collect()
    }
    pub fn is_item_in_trade(
        &self,
        trade_type: &TradeClassification,
        unique_name: &str,
        quantity: i64,
    ) -> bool {
        let items = self.get_valid_items(trade_type, vec![]);
        items
            .iter()
            .any(|p| p.unique_name == unique_name && p.quantity == quantity)
    }
    pub fn calculate(&mut self) {
        let offer_plat = self.get_offered_plat();
        let receive_plat = self.get_received_plat();

        log(
            format!(
                "Calculating Trade | Offered Plat: {} | Received Plat: {}",
                offer_plat, receive_plat
            ),
            None,
        );
        self.credits = self
            .offered_items
            .iter()
            .filter(|p| p.item_type == TradeItemType::Credits)
            .map(|p| p.quantity)
            .sum::<i64>();
        if offer_plat > 0 {
            self.platinum = offer_plat;
            log(format!("Platinum set from Offer: {}", self.platinum), None);
        }
        if receive_plat > 0 {
            self.platinum = receive_plat;
            log(
                format!("Platinum set from Receive: {}", self.platinum),
                None,
            );
        }

        // Filter out unknown items
        let offered_items = self
            .get_valid_items(&TradeClassification::Purchase, vec![])
            .iter()
            .filter(|p| p.item_type != TradeItemType::Credits)
            .cloned()
            .collect::<Vec<TradeItem>>();
        let received_items = self
            .get_valid_items(&TradeClassification::Sale, vec![])
            .iter()
            .filter(|p| p.item_type != TradeItemType::Credits)
            .cloned()
            .collect::<Vec<TradeItem>>();

        log(
            format!(
                "Filtered Items | Offered: {} | Received: {}",
                offered_items.len(),
                received_items.len()
            ),
            None,
        );

        if offer_plat > 1 && offered_items.len() == 1 {
            self.trade_type = TradeClassification::Purchase;
            log(
                format!(
                    "Classified Trade as Purchase | Item: {:?}",
                    offered_items.first()
                ),
                None,
            );
        } else if receive_plat > 1 && received_items.len() == 1 {
            self.trade_type = TradeClassification::Sale;
            log(
                format!(
                    "Classified Trade as Sale | Item: {:?}",
                    received_items.first()
                ),
                None,
            );
        } else {
            self.trade_type = TradeClassification::Trade;
            log(
                "Classified Trade as Regular Item-for-Item Trade".to_string(),
                None,
            );
        }
    }
    pub fn calculate_items(&mut self) {
        self.is_set(TradeClassification::Purchase);
        self.is_set(TradeClassification::Sale);
    }

    pub fn get_item_by_type(
        &self,
        trade_type: &TradeClassification,
        item_type: &TradeItemType,
    ) -> Option<TradeItem> {
        let items = self.get_valid_items(trade_type, vec![]);
        items.iter().find(|p| &p.item_type == item_type).cloned()
    }
    pub fn is_set(&mut self, trade_type: TradeClassification) {
        let main_item = match self.get_item_by_type(&trade_type, &TradeItemType::MainBlueprint) {
            Some(item) => item,
            None => return,
        };

        if self.get_valid_items(&self.trade_type, vec![]).is_empty() {
            return;
        }

        let cache = match states::cache_client() {
            Ok(c) => c,
            Err(_) => {
                log("Cache client not initialized".to_string(), None);
                return;
            }
        };

        let component_key = format!("Component|{}", main_item.unique_name);
        let component = match cache.all_items().get_by(&component_key) {
            Ok(c) => c,
            Err(_) => {
                log(format!("Main part not found: {}", component_key), None);
                return;
            }
        };

        log(
            format!("Found main part {} for set", component.display()),
            None,
        );

        let set_name = match &component.part_of_set {
            Some(name) => name,
            None => {
                log(
                    format!("Part-of-set missing for {}", component.display()),
                    None,
                );
                return;
            }
        };

        log(format!("Set unique name: {}", set_name), None);

        let set = match cache.all_items().get_by(set_name) {
            Ok(s) => s,
            Err(_) => {
                log(format!("Set not found: {}", set_name), None);
                return;
            }
        };

        log(format!("Found set {}", set.display()), None);

        let components = set.get_tradable_components();
        if components.is_empty() {
            log(format!("No components for set {}", set.display()), None);
            return;
        }

        for component in components.iter() {
            let found =
                self.is_item_in_trade(&trade_type, &component.unique_name, component.item_count);

            log(
                format!(
                    "Checking component <{}>: Found={}",
                    component.display(),
                    found
                ),
                None,
            );

            if !found {
                return;
            }
        }

        log(format!("Full set found: {}", set.display()), None);

        let target_items = match trade_type {
            TradeClassification::Purchase => &mut self.offered_items,
            TradeClassification::Sale => &mut self.received_items,
            _ => return,
        };
        for component in components {
            target_items.retain(|p| p.unique_name != component.unique_name);
            log(
                format!("Removed component from trade: {}", component.display()),
                None,
            );
        }
        target_items.push(TradeItem::new(
            &set.unique_name,
            1,
            TradeItemType::Set,
            None,
        ));
    }

    pub fn get_notify_variables(&self) -> HashMap<String, String> {
        let offered_items = self
            .get_valid_items(&TradeClassification::Purchase, vec![])
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");

        let received_items = self
            .get_valid_items(&TradeClassification::Sale, vec![])
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");

        return HashMap::from([
            ("<TR_TYPE>".to_string(), self.trade_type.to_string()),
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
            (
                "<TIME>".to_string(),
                self.trade_time
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
            ("<RE_ITEMS>".to_string(), received_items),
            ("<LOGS>".to_string(), self.logs.join("\n")),
            ("<TOTAL_PLAT>".to_string(), self.platinum.to_string()),
        ]);
    }

    pub fn set_time(&mut self, time: DateTime<chrono::Utc>) -> PlayerTrade {
        self.trade_time = time;
        self.clone()
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
