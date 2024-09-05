use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

use crate::{live_scraper::types::order_extra_info::OrderDetails, wfm_client::enums::order_type::OrderType};

use super::order::Order;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Orders {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>,
}
impl Orders {
    pub fn get_all_orders(&mut self) -> Vec<Order> {
        let mut orders = self.sell_orders.clone();
        orders.append(&mut self.buy_orders.clone());
        orders
    }

    pub fn sort_by_platinum(&mut self) {
        self.sell_orders.sort_by(|a, b| a.platinum.cmp(&b.platinum));
        self.buy_orders.sort_by(|a, b| b.platinum.cmp(&a.platinum));
    }

    pub fn filter_by_username(&mut self, username: &str, exclude: bool) -> Orders
    {
        let sell_orders = self
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
        let buy_orders = self
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
        Orders {
            sell_orders,
            buy_orders,
        }
    }

    pub fn filter_by_sub_type(self, sub_type: Option<&SubType>, exclude: bool) -> Orders
    {
        let sell_orders = self
            .sell_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.get_subtype().as_ref() != sub_type
                } else {
                    order.get_subtype().as_ref() == sub_type
                }
            })
            .cloned()
            .collect();
        let buy_orders = self
            .buy_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.get_subtype().as_ref() != sub_type
                } else {
                    order.get_subtype().as_ref() == sub_type
                }
            })
            .cloned()
            .collect();
        Orders {
            sell_orders,
            buy_orders,
        }
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

    pub fn get_price_range(&self, order_type: OrderType) -> i64 {
        let lowest_price = self.lowest_price(OrderType::Sell);
        let highest_price = self.highest_price(OrderType::Buy);
        if order_type == OrderType::Sell {
            return highest_price - lowest_price;
        } else if order_type == OrderType::Buy {
            return lowest_price - highest_price;
        }
        return 0;
    }

    pub fn total_count(&self) -> i64 {
        self.sell_orders.len() as i64 + self.buy_orders.len() as i64
    }

    pub fn delete_order_by_id(&mut self, order_type: OrderType, id: &str) {
        let orders = match order_type {
            OrderType::Sell => &mut self.sell_orders,
            OrderType::Buy => &mut self.buy_orders,
            _ => return,
        };
        let index = orders.iter().position(|x| x.id == id);
        if index.is_none() {
            return;
        }
        orders.remove(index.unwrap());
    }

    pub fn update_order(&mut self, order: Order) {
        let orders = match order.order_type {
            OrderType::Sell => &mut self.sell_orders,
            OrderType::Buy => &mut self.buy_orders,
            _ => return,
        };
        let index = orders.iter().position(|x| x.id == order.id);
        if index.is_none() {
            return;
        }
        orders[index.unwrap()] = order;
    }

    pub fn get_orders_by_url(&self, wfm_url: &str, order_type: OrderType) -> Vec<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return vec![],
        };
        let filtered_orders = orders
            .iter()
            .filter(|order| order.item.is_some() && order.item.as_ref().unwrap().url_name == wfm_url)
            .cloned()
            .collect::<Vec<Order>>();
        filtered_orders
    }
   
    pub fn find_order_by_url_sub_type(
        &self,
        wfm_url: &str,
        order_type: OrderType,
        sub_type: Option<&SubType>,
    ) -> Option<Order> {
        let orders = self.get_orders_by_url(wfm_url, order_type);
        for order in orders {
            let type_sub_type = order.get_subtype();
            if type_sub_type.as_ref() == sub_type {
                return Some(order);
            }
        }
        return None;
    }
}
