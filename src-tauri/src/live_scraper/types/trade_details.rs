use entity::dto::{PriceHistory, SubType};
use reqwest::header::TRANSFER_ENCODING;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utils::Properties;
use wf_market::types::{Auction, Order};

use crate::{
    cache::{CacheState, CacheTradableItem},
    types::OperationSet,
    utils::OrderExt,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeDetails {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    pub sub_type: Option<SubType>,

    #[serde(rename = "image", default)]
    pub image: String,

    #[serde(rename = "quantity", default)]
    pub quantity: u32,

    #[serde(rename = "profit", default)]
    pub profit: i64,

    #[serde(rename = "closed_avg", default)]
    pub closed_avg: f64,

    #[serde(rename = "lowest_price", default)]
    pub lowest_price: i64,

    #[serde(rename = "highest_price", default)]
    pub highest_price: i64,

    #[serde(rename = "update_string", default)]
    pub update_string: String,

    #[serde(rename = "operation", default, flatten)]
    pub operations: OperationSet,

    #[serde(rename = "trades", default)]
    pub trades: Value,

    #[serde(rename = "price_history", default)]
    pub price_history: Vec<PriceHistory>,

    // Extra properties
    #[serde(flatten)]
    pub properties: Properties,
}
impl TradeDetails {
    pub fn add_price_history(&mut self, price_history: PriceHistory) {
        let mut items = self.price_history.clone();

        let last_item = items.last().cloned();
        if last_item.is_none() || last_item.unwrap().price != price_history.price {
            // Limit to 5 elements
            if items.len() >= 5 {
                items.remove(0);
            }
            items.push(price_history);
            self.price_history = items;
        }
    }
    pub fn apply_trade_item_info(&mut self, info: &CacheTradableItem) {
        self.wfm_id = info.wfm_id.clone();
        self.name = info.name.clone();
        self.image = info.image_url.clone();
        self.properties
            .set_property_value("t_type", json!(info.sub_type));
    }

    pub fn try_from_auction(order: &Auction, cache: &CacheState) -> Self {
        TradeDetails::default()
    }
}

impl Default for TradeDetails {
    fn default() -> Self {
        Self {
            id: String::new(),
            wfm_id: String::new(),
            sub_type: None,
            quantity: 0,
            profit: 0,
            closed_avg: 0.0,
            lowest_price: 0,
            highest_price: 0,
            update_string: String::new(),
            operations: OperationSet::from(vec!["Create"]),
            trades: Value::Null,
            name: String::new(),
            image: String::new(),
            price_history: Vec::new(),
            properties: Properties::default(),
        }
    }
}
