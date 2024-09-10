use std::collections::VecDeque;

use entity::price_history::PriceHistory;
use serde::{Deserialize, Serialize};

use crate::wfm_client::types::order::Order;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "total_buyers")]
    pub total_buyers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "total_sellers")]
    pub total_sellers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profit")]
    pub profit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lowest_price")]
    pub lowest_price: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highest_price")]
    pub highest_price: Option<i64>,

    #[serde(rename = "orders")]
    pub orders: Vec<Order>,

    #[serde(rename = "price_history")]
    pub price_history: VecDeque<PriceHistory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moving_avg")]
    pub moving_avg: Option<i64>,

    #[serde(rename = "is_dirty")]
    pub is_dirty: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes")]
    pub changes: Option<String>,
}

// Default implementation for OrderDetails
impl Default for OrderDetails {
    fn default() -> Self {
        OrderDetails {
            is_dirty: true,
            total_buyers: None,
            total_sellers: None,
            lowest_price: None,
            highest_price: None,
            profit: None,
            moving_avg: None,
            orders: Vec::new(),
            price_history: VecDeque::new(),
            changes: None,
        }
    }
}

impl OrderDetails {
    pub fn new(
        total_buyers: i64,
        total_sellers: i64,
        profit: i64,
        lowest_price: i64,
        highest_price: i64,
        moving_avg: i64,
        orders: Vec<Order>,
        price_history: Vec<PriceHistory>,
    ) -> OrderDetails {
        OrderDetails {
            total_buyers: Some(total_buyers),
            total_sellers: Some(total_sellers),
            lowest_price: Some(lowest_price),
            profit: Some(profit),
            highest_price: Some(highest_price),
            moving_avg: Some(moving_avg),
            orders,
            price_history: price_history.into_iter().collect(),
            is_dirty: true,
            changes: None,
        }
    }
    // Helper to set dirty flag when values are changed
    fn set_if_changed<T: PartialEq>(current: &mut T, new_value: T, is_dirty: &mut bool) -> bool {
        if *current != new_value {
            *current = new_value;
            *is_dirty = true;
            return true;
        }
        false
    }
    pub fn set_total_buyers(&mut self, total_buyers: i64) {
        if Self::set_if_changed(
            &mut self.total_buyers,
            Some(total_buyers),
            &mut self.is_dirty,
        ) {
            self.changes = Some("total_buyers".to_string());
        }
    }

    pub fn set_lowest_price(&mut self, lowest_price: i64) {
        if Self::set_if_changed(
            &mut self.lowest_price,
            Some(lowest_price),
            &mut self.is_dirty,
        ) {
            self.changes = Some("lowest_price".to_string());
        }
    }

    pub fn set_highest_price(&mut self, highest_price: i64) {
        if Self::set_if_changed(
            &mut self.highest_price,
            Some(highest_price),
            &mut self.is_dirty,
        ) {
            self.changes = Some("highest_price".to_string());
        }
    }

    pub fn set_orders(&mut self, orders: Vec<Order>) {
        self.orders = orders;
    }

    pub fn set_moving_avg(&mut self, moving_avg: i64) {
        if Self::set_if_changed(&mut self.moving_avg, Some(moving_avg), &mut self.is_dirty) {
            self.changes = Some("moving_avg".to_string());
        }
    }

    pub fn set_profit(&mut self, profit: i64) {
        if Self::set_if_changed(&mut self.profit, Some(profit), &mut self.is_dirty) {
            self.changes = Some("profit".to_string());
        }
    }

    pub fn set_total_sellers(&mut self, total_sellers: i64) {
        if Self::set_if_changed(
            &mut self.total_sellers,
            Some(total_sellers),
            &mut self.is_dirty,
        ) {
            self.changes = Some("total_sellers".to_string());
        }
    }

    pub fn add_price_history(&mut self, price_history: PriceHistory) {
        if self
            .price_history
            .back()
            .map_or(true, |last| last.price != price_history.price)
        {
            // Limit to 5 elements
            if self.price_history.len() >= 5 {
                self.price_history.pop_front();
            }
            self.price_history.push_back(price_history);
            self.is_dirty = true;
            self.changes = Some("price_history".to_string());
        }
    }
}
