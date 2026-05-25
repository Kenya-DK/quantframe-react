use std::{collections::HashMap, fmt::Display, vec};

use crate::cache::CacheItemBase;
use crate::enums::TradeItemType;
use crate::{log_parser::*, utils::modules::states};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use utils::Properties;

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

    #[serde(flatten)]
    pub properties: Properties,
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
            properties: Properties::default(),
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
                    "Classified Trade as Purchase | Item: {}",
                    offered_items
                        .first()
                        .unwrap_or(&TradeItem::default())
                        .item_name()
                ),
                None,
            );
        } else if receive_plat > 1 && received_items.len() == 1 {
            self.trade_type = TradeClassification::Sale;
            log(
                format!(
                    "Classified Trade as Sale | Item: {}",
                    received_items
                        .first()
                        .unwrap_or(&TradeItem::default())
                        .item_name()
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

    pub fn is_set(&mut self, trade_type: TradeClassification) {
        // Build a lightweight list of tradable item identifiers + quantities from the selected side.
        let items = self
            .get_valid_items(&trade_type, vec![])
            .iter()
            .map(|item| CacheItemBase::new(&item.unique_name, item.quantity))
            .collect::<Vec<_>>();

        // A set requires multiple components, so skip if we only have one (or zero) items.
        if items.len() <= 1 {
            log(
                format!(
                    "Skipping side {:?} for set detection: not enough items",
                    trade_type
                ),
                None,
            );
            return;
        }

        // Resolve cache client for recipe and item lookups.
        let cache = match states::cache_client() {
            Ok(cache) => cache,
            Err(_) => {
                log("Cache client not initialized".to_string(), None);
                return;
            }
        };

        // Helper to query recipes, optionally enforcing "recipe-only" ingredient checks.
        let get_buildable_set = |recipe_only| cache.recipe().can_craft(&items, true, recipe_only);

        // First pass: strict match. If that fails, do a second relaxed pass.
        let mut recipes = match get_buildable_set(false) {
            Ok(set) => set,
            Err(err) => {
                log(format!("Error checking buildable set: {}", err), None);

                return;
            }
        };

        log(
            format!("Set detection strict pass | matches: {}", recipes.len()),
            None,
        );

        if recipes.is_empty() {
            log(
                "Set detection strict pass empty; trying relaxed pass".to_string(),
                None,
            );
            recipes = match get_buildable_set(true) {
                Ok(set) => set,
                Err(err) => {
                    log(
                        format!("Error checking buildable set (recipe-only): {}", err),
                        None,
                    );

                    return;
                }
            };

            log(
                format!("Set detection relaxed pass | matches: {}", recipes.len()),
                None,
            );
        }

        // Pick the mutable trade side we will transform (purchase uses offered, sale uses received).
        let target_items = match trade_type {
            TradeClassification::Purchase => &mut self.offered_items,
            TradeClassification::Sale => &mut self.received_items,
            _ => return,
        };

        let initial_target_len = target_items.len();
        let mut converted_sets = 0;

        log(
            format!(
                "Starting set detection | side: {:?} | candidate_recipes: {}",
                trade_type,
                recipes.len()
            ),
            None,
        );

        // Replace each detected recipe + its components with a single synthetic Set item.
        for recipe in &recipes {
            log(format!("Found buildable recipe: {}", recipe.base), None);

            let set_item_data = match cache
                .all_items()
                .get_by(format!("Unique:{}", recipe.result_type))
            {
                Ok(item) => item,

                Err(err) => {
                    log(format!("Set item not found in cache: {}", err), None);

                    continue;
                }
            };

            log(
                format!("Resolved set item from cache: {}", set_item_data),
                None,
            );

            // Remove blueprint entry itself.
            if recipe.override_unique_name.is_empty() {
                target_items.retain(|item| item.unique_name != recipe.base.unique_name);
            } else {
                target_items.retain(|item| item.unique_name != recipe.override_unique_name);
            }

            // Remove every recipe component (either direct ingredient item or sub-recipe blueprint).
            for ingredient in &recipe.ingredients {
                target_items.retain(|item| {
                    item.unique_name != ingredient.from_recipe
                        && item.unique_name != ingredient.base.unique_name
                });

                log(
                    format!("Removed component from trade: {}", ingredient),
                    None,
                );
            }

            // Add the resolved crafted set item back into the trade list.
            let mut set_item = TradeItem::new(
                &set_item_data.unique_name,
                1,
                TradeItemType::Set,
                set_item_data.sub_type.clone(),
            );

            set_item
                .properties
                .set_property_value("tags", vec!["set".to_string()]);

            set_item
                .properties
                .set_property_value("item_name", set_item_data.name.clone());

            target_items.push(set_item);
            converted_sets += 1;

            log(format!("Set item inserted {}", set_item_data), None);
        }

        log(
            format!(
                "Set detection completed | side: {:?} | converted_sets: {} | target_items: {} -> {}",
                trade_type,
                converted_sets,
                initial_target_len,
                target_items.len()
            ),
            None,
        );
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
        if self.trade_type == TradeClassification::Purchase && self.received_items.len() == 1 {
            write!(
                f,
                " | Item: {}",
                self.received_items.first().unwrap().item_name()
            )?;
        } else if self.trade_type == TradeClassification::Sale && self.offered_items.len() == 1 {
            write!(
                f,
                " | Item: {}",
                self.offered_items.first().unwrap().item_name()
            )?;
        }
        Ok(())
    }
}
