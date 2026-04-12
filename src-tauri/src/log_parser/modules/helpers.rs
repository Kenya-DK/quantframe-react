use std::collections::HashMap;

use chrono::{DateTime, Utc};
use entity::dto::*;
use serde_json::json;
use utils::*;

use crate::{enums::TradeItemType, log_parser::*};

/* =======================
    HELPER METHODS
======================= */
pub fn to_date(text: &str) -> DateTime<Utc> {
    match text.parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        Err(e) => {
            println!("Failed to parse date line '{}': {}", text, e);
            Utc::now()
        }
    }
}
/* =======================
    TRADE METHODS
======================= */
pub fn apply_item_info(trade: &mut PlayerTrade) {
    let tags = trade
        .offered_items
        .iter()
        .chain(trade.received_items.iter())
        .flat_map(|item| item.properties.get_property_value("tags", vec![]))
        .collect::<Vec<String>>();
    trade.properties.set_property_value("tags", tags);
    let names = trade
        .offered_items
        .iter()
        .chain(trade.received_items.iter())
        .map(|item| {
            item.properties
                .get_property_value("item_name", String::new())
        })
        .collect::<Vec<String>>();
    trade.properties.set_property_value("names", names);
}
/* =======================
    LOGIN METHODS
======================= */

/* =======================
   PURCHASE METHODS
======================= */

/* =======================
    TRANSACTION METHODS
======================= */

/* =======================
    HELPER METHODS
======================= */
