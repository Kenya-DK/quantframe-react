use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

use crate::{
    live_scraper::types::order_extra_info::OrderDetails,
    utils::modules::{error::AppError, states},
    wfm_client::enums::order_type::OrderType,
};

use super::order::Order;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Orders {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>,
}
impl Orders {
    pub fn new(sell_orders: Vec<Order>, buy_orders: Vec<Order>) -> Self {
        Orders {
            sell_orders,
            buy_orders,
        }
    }
    pub fn get_all_orders(&self) -> Vec<Order> {
        let mut orders = self.sell_orders.clone();
        orders.append(&mut self.buy_orders.clone());
        orders
    }

    pub fn sort_by_platinum(&mut self) {
        self.sell_orders.sort_by(|a, b| a.platinum.cmp(&b.platinum));
        self.buy_orders.sort_by(|a, b| b.platinum.cmp(&a.platinum));
    }

    pub fn filter_by_username(&mut self, username: &str, exclude: bool) -> Orders {
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

    pub fn filter_by_sub_type(self, sub_type: Option<&SubType>, exclude: bool) -> Orders {
        let sell_orders = self
            .sell_orders
            .iter()
            .filter(|order| {
                if exclude {
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
        match order_type {
            OrderType::Sell => &mut self.sell_orders.retain(|x| x.id != id),
            OrderType::Buy => &mut self.buy_orders.retain(|x| x.id != id),
            OrderType::All => {
                self.sell_orders.retain(|x| x.id != id);
                self.buy_orders.retain(|x| x.id != id);
                return;
            }
            _ => return,
        };
    }

    pub fn update_order(
        &mut self,
        order_type: OrderType,
        id: &str,
        platinum: Option<i64>,
        quantity: Option<i64>,
        visible: Option<bool>,
        info: Option<OrderDetails>,
    ) {
        let orders = match order_type {
            OrderType::Sell => &mut self.sell_orders,
            OrderType::Buy => &mut self.buy_orders,
            _ => return,
        };
        let index = orders.iter().position(|x| x.id == id);
        if index.is_none() {
            return;
        }
        let index = index.unwrap();
        if platinum.is_none() && quantity.is_none() && visible.is_none() {
            return;
        }
        if let Some(platinum) = platinum {
            orders[index].platinum = platinum;
        }
        if let Some(quantity) = quantity {
            orders[index].quantity = quantity;
        }
        if let Some(visible) = visible {
            orders[index].visible = visible;
        }
        if let Some(info) = info {
            orders[index].info = info;
        }
    }

    pub fn get_orders_by_id(&self, wfm_id: &str, order_type: OrderType) -> Vec<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return vec![],
        };
        let filtered_orders = orders
            .iter()
            .filter(|order| order.item_id == wfm_id)
            .cloned()
            .collect::<Vec<Order>>();
        filtered_orders
    }

    pub fn get_orders_ids2(
        &self,
        order_type: OrderType,
        exclude_items: Vec<String>,
    ) -> Vec<String> {
        let mut ids = vec![];
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return ids,
        };

        for order in orders.iter() {
            if !exclude_items.contains(&order.info.wfm_url) {
                ids.push(order.id.clone());
            }
        }
        ids
    }

    pub fn apply_trade_info(&mut self) -> Result<(), AppError> {
        let cache = states::cache().expect("Cache should always be available");
        for order in self
            .buy_orders
            .iter_mut()
            .chain(self.sell_orders.iter_mut())
        {
            let item_info = cache
                .tradable_items()
                .get_by(&order.item_id, "--item_by id")?;

            if let Some(item_info) = item_info {
                order.info.set_wfm_url(item_info.wfm_url_name.clone());
                order.info.set_name(item_info.name.clone());
                order.info.set_image(item_info.image_url.clone());
            } else {
                order.info.set_name("Unknown Item".to_string());
                order.info.set_image("".to_string());
            }

            match cache
                .item_price()
                .get_item_price2(&order.item_id, order.get_subtype())
            {
                Ok(info) => {
                    order.info.set_closed_avg(info.avg_price);
                    order
                        .info
                        .set_profit(order.platinum as f64 - info.min_price);
                }
                Err(_) => {
                    order.info.set_closed_avg(0.0);
                    order.info.set_profit(0.0);
                }
            }
        }
        Ok(())
    }

    pub fn find_order_by_url_sub_type(
        &self,
        wfm_id: &str,
        order_type: OrderType,
        sub_type: Option<&SubType>,
    ) -> Option<Order> {
        let orders = self.get_orders_by_id(wfm_id, order_type);
        for order in orders {
            let type_sub_type = order.get_subtype();
            if type_sub_type.as_ref() == sub_type {
                return Some(order);
            }
        }
        return None;
    }
    pub fn add_order(&mut self, order_type: OrderType, order: Order) {
        match order_type {
            OrderType::Sell => self.sell_orders.push(order),
            OrderType::Buy => self.buy_orders.push(order),
            _ => {}
        }
    }
}
