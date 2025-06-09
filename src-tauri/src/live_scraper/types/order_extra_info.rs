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
    pub profit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<i64>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closed_avg")]
    pub closed_avg: Option<f64>,

    // Ignore this field
    #[serde(skip_serializing)]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "is_dirty")]
    pub is_dirty: bool,

    // Default value for changes
    #[serde(rename = "changes")]
    #[serde(default)]
    pub changes: Vec<String>,
}

// Default implementation for OrderDetails
impl Default for OrderDetails {
    fn default() -> Self {
        OrderDetails {
            is_dirty: true,
            total_buyers: None,
            total_sellers: None,
            closed_avg: None,
            lowest_price: None,
            range: None,
            highest_price: None,
            profit: None,
            moving_avg: None,
            orders: Vec::new(),
            tags: Vec::new(),
            price_history: VecDeque::new(),
            changes: Vec::new(),
        }
    }
}

impl OrderDetails {
    pub fn reset_changes(&mut self) {
        self.changes = Vec::new();
        self.is_dirty = false;
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
        self.total_buyers = Some(total_buyers);
    }

    pub fn set_lowest_price(&mut self, lowest_price: i64) {
        if Self::set_if_changed(
            &mut self.lowest_price,
            Some(lowest_price),
            &mut self.is_dirty,
        ) {
            self.changes.push("lowest_price".to_string());
        }
    }

    pub fn set_highest_price(&mut self, highest_price: i64) {
        let highest_price = Some(highest_price);
        if Self::set_if_changed(&mut self.highest_price, highest_price, &mut self.is_dirty) {
            self.changes.push("highest_price".to_string());
        }
    }

    pub fn set_orders(&mut self, orders: Vec<Order>) {
        self.orders = orders;
    }

    pub fn set_moving_avg(&mut self, moving_avg: i64) {
        self.moving_avg = Some(moving_avg);
    }

    pub fn set_range(&mut self, range: i64) {
        let range = Some(range);
        if Self::set_if_changed(&mut self.range, range, &mut self.is_dirty) {
            self.changes.push("range".to_string());
        }
    }

    pub fn set_total_sellers(&mut self, total_sellers: i64) {
        self.total_sellers = Some(total_sellers);
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
            self.changes.push("price_history".to_string());
        }
    }
    pub fn set_closed_avg(&mut self, closed_avg: f64) {
        let closed_avg = Some(closed_avg);
        if Self::set_if_changed(&mut self.closed_avg, closed_avg, &mut self.is_dirty) {
            self.changes.push("closed_avg".to_string());
        }
    }
    pub fn set_profit(&mut self, profit: f64) {
        let profit = Some(profit);
        if Self::set_if_changed(&mut self.profit, profit, &mut self.is_dirty) {
            self.changes.push("profit".to_string());
        }
    }
}
