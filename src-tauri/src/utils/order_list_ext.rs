use utils::Error;
use wf_market::{
    enums::OrderType,
    types::{Order, OrderList},
};

use crate::{
    cache::client::CacheState,
    utils::{modules::states, order_ext::OrderDetails, OrderExt, SubTypeExt},
};

/// Extension trait for order list
pub trait OrderListExt {
    fn apply_trade_info(&mut self) -> Result<(), Error>;
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
    fn extract_order_summary(&self, order_type: OrderType) -> Vec<(i64, f64, String, String)>;
}

impl OrderListExt for OrderList<Order> {
    fn apply_trade_info(&mut self) -> Result<(), Error> {
        let cache = states::cache_client().expect("Cache should always be available");

        for order in self
            .buy_orders
            .iter_mut()
            .chain(self.sell_orders.iter_mut())
        {
            if let Some(price) = cache
                .item_price()
                .find_by_id(&order.item_id, order.subtype.to_entity())?
            {
                order.update_details(
                    order
                        .get_details()
                        .set_closed_avg(price.avg_price)
                        .set_profit(price.profit)
                        .set_order_id(order.id.clone()),
                );
            }
        }

        Ok(())
    }
    fn extract_order_summary(&self, order_type: OrderType) -> Vec<(i64, f64, String, String)> {
        let orders = match order_type {
            OrderType::Buy => &self.buy_orders,
            OrderType::Sell => &self.sell_orders,
        };
        orders
            .iter()
            .map(|order| {
                let platinum = order.platinum as i64;
                let profit = order.get_details().profit;
                let wfm_id = order.item_id.clone();
                let id = order.id.clone();
                (platinum, profit, wfm_id, id)
            })
            .collect::<Vec<(i64, f64, String, String)>>()
    }
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        for order in self
            .buy_orders
            .iter_mut()
            .chain(self.sell_orders.iter_mut())
        {
            order.apply_item_info(cache)?;
        }

        Ok(())
    }
}
