use serde::{Deserialize, Serialize};

use crate::wfm_client::enums::order_type::OrderType;

use super::order::Order;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Orders {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>,
}
impl Orders {
    pub fn sort_by_platinum(&mut self) {
        self.sell_orders.sort_by(|a, b| a.platinum.cmp(&b.platinum));
        self.buy_orders.sort_by(|a, b| b.platinum.cmp(&a.platinum));
    }

    pub fn filter_by_username(&mut self, username: &str, exclude: bool)
    where
        Self: Sized,
    {
        self.sell_orders = self
            .sell_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.user.clone().map(|user| user.ingame_name.clone())
                        != Some(username.to_owned())
                } else {
                    order.user.clone().map(|user| user.ingame_name.clone())
                        == Some(username.to_owned())
                }
            })
            .cloned()
            .collect();
        self.buy_orders = self
            .buy_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.user.clone().map(|user| user.ingame_name.clone())
                        != Some(username.to_owned())
                } else {
                    order.user.clone().map(|user| user.ingame_name.clone())
                        == Some(username.to_owned())
                }
            })
            .cloned()
            .collect();
    }

    pub fn lowest_order(&self, order_type: OrderType) -> Option<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return None,
        };

        if orders.is_empty() {
            return None;
        }
        orders
            .iter()
            .min_by(|a, b| a.platinum.cmp(&b.platinum))
            .cloned()
    }

    pub fn lowest_price(&self, order_type: OrderType) -> i64 {
        let order = self.lowest_order(order_type);
        if order.is_none() {
            return 0;
        }
        order.unwrap().platinum
    }

    pub fn highest_order(&self, order_type: OrderType) -> Option<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return None,
        };

        if orders.is_empty() {
            return None;
        }
        orders
            .iter()
            .max_by(|a, b| a.platinum.cmp(&b.platinum))
            .cloned()
    }

    pub fn highest_price(&self, order_type: OrderType) -> i64 {
        let order = self.highest_order(order_type);
        if order.is_none() {
            return 0;
        }
        order.unwrap().platinum
    }

    pub fn get_price_range(&self) -> i64 {
        let lowest_price = self.lowest_price(OrderType::Sell);
        let highest_price = self.highest_price(OrderType::Buy);
        return lowest_price - highest_price;
    }

    pub fn total_count(&self) -> i64 {
        self.sell_orders.len() as i64 + self.buy_orders.len() as i64
    }
}
