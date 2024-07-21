use entity::price_history::PriceHistory;
use serde::{Deserialize, Serialize};

use crate::wfm_client::types::order::Order;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDetails {
    #[serde(rename = "total_buyers")]
    pub total_buyers: i64,

    #[serde(rename = "lowest_price")]
    pub lowest_price: i64,

    #[serde(rename = "highest_price")]
    pub highest_price: i64,

    #[serde(rename = "orders")]
    pub orders: Vec<Order>,

    #[serde(rename = "price_history")]
    pub price_history: Vec<PriceHistory>,
}

// Default implementation for OrderDetails
impl Default for OrderDetails {
    fn default() -> Self {
        OrderDetails {
            total_buyers: 0,
            lowest_price: 0,
            highest_price: 0,
            orders: Vec::new(),
            price_history: Vec::new(),
        }
    }
}

impl OrderDetails {
    pub fn new(
        total_buyers: i64,
        lowest_price: i64,
        highest_price: i64,
        orders: Vec<Order>,
        price_history: Vec<PriceHistory>,
    ) -> OrderDetails {
        OrderDetails {
            total_buyers,
            lowest_price,
            highest_price,
            orders,
            price_history,
        }
    }

    pub fn add_price_history(&mut self, price_history: PriceHistory) {
        let last_price_history = self.price_history.last();
        if last_price_history.is_none() || last_price_history.unwrap().price != price_history.price
        {
            // If the price history is over 5 items, remove the first item
            if self.price_history.len() >= 5 {
                self.price_history.remove(0);
            }
            self.price_history.push(price_history);
        }
    }
}
