use std::fmt::Display;

use entity::dto::*;
use serde::{Deserialize, Serialize};
use utils::{warning, Error, LoggerOptions};
use wf_market::types::{Order, OrderLike, OrderWithUser};

use crate::{
    cache::{
        client::CacheState,
        types::{CacheTradableItem, SubType as CacheSubType},
    },
    types::OperationSet,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDetails {
    pub order_id: String,
    pub item_id: String,
    pub sub_type: Option<SubType>,
    pub quantity: u32,
    #[serde(rename = "profit")]
    #[serde(default)]
    pub profit: f64,

    #[serde(rename = "closed_avg")]
    #[serde(default)]
    pub closed_avg: f64,

    #[serde(default)]
    #[serde(rename = "lowest_price")]
    pub lowest_price: i64,

    #[serde(default)]
    #[serde(rename = "highest_price")]
    pub highest_price: i64,

    #[serde(default)]
    #[serde(rename = "update_string")]
    pub update_string: String,

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(flatten)]
    #[serde(default)]
    pub operations: OperationSet,

    #[serde(rename = "orders")]
    #[serde(default)]
    pub orders: Vec<OrderWithUser>,

    // Item Info
    pub item_name: String,
    pub image_url: String,
    pub trade_sub_type: Option<CacheSubType>,

    #[serde(rename = "price_history")]
    #[serde(default)]
    pub price_history: Vec<PriceHistory>,
}
impl OrderDetails {
    pub fn set_closed_avg(mut self, closed_avg: f64) -> Self {
        self.closed_avg = closed_avg;
        self
    }
    pub fn set_profit(mut self, profit: f64) -> Self {
        self.profit = profit;
        self
    }
    pub fn set_operation(mut self, operation: &[&str]) -> Self {
        self.operations.set(operation);
        self
    }
    pub fn add_operation(&mut self, operation: impl Into<String>) {
        self.operations.add(operation);
    }
    pub fn has_operation(&self, operation: impl Into<String>) -> bool {
        let operation = operation.into();
        self.operations.has(operation)
    }
    pub fn set_order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = order_id.into();
        self
    }
    pub fn set_item_id(mut self, item_id: impl Into<String>) -> Self {
        self.item_id = item_id.into();
        self
    }

    pub fn set_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }
    pub fn set_sub_type(mut self, sub_type: Option<SubType>) -> Self {
        self.sub_type = sub_type;
        self
    }
    pub fn set_lowest_price(mut self, lowest_price: i64) -> Self {
        self.lowest_price = lowest_price;
        self
    }
    pub fn set_highest_price(mut self, highest_price: i64) -> Self {
        self.highest_price = highest_price;
        self
    }
    pub fn set_orders(mut self, orders: Vec<OrderWithUser>) -> Self {
        self.orders = orders;
        self
    }
    pub fn set_info(mut self, info: &CacheTradableItem) -> Self {
        self.item_name = info.name.clone();
        self.image_url = info.image_url.clone();
        self.trade_sub_type = info.sub_type.clone();
        self
    }
    pub fn set_update_string(mut self, update_string: impl Into<String>) -> Self {
        self.update_string = update_string.into();
        self
    }
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
}

// Default implementation for OrderDetails
impl Default for OrderDetails {
    fn default() -> Self {
        let mut operations = OperationSet::new();
        operations.add("Create");
        OrderDetails {
            order_id: String::from("N/A"),
            item_id: String::from("N/A"),
            sub_type: None,
            item_name: String::from("Unknown Item"),
            image_url: String::from("https://via.placeholder.com/150"),
            quantity: 1,
            closed_avg: 0.0,
            profit: 0.0,
            lowest_price: -1,
            highest_price: -1,
            operations: operations,
            update_string: String::new(),
            orders: vec![],
            price_history: vec![],
            trade_sub_type: None,
        }
    }
}

impl Display for OrderDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderDetails: ")?;
        if !self.order_id.is_empty() {
            write!(f, "OrderID: {} | ", self.order_id)?;
        }
        if !self.item_id.is_empty() {
            write!(f, "ItemID: {} | ", self.item_id)?;
        }
        if let Some(ref sub_type) = self.sub_type {
            write!(f, "SubType: {}| ", sub_type.display())?;
        }
        if !self.item_name.is_empty() {
            write!(f, "ItemName: {} | ", self.item_name)?;
        }
        write!(f, "Quantity: {} | ", self.quantity)?;
        write!(f, "Profit: {:.2} | ", self.profit)?;
        write!(f, "ClosedAvg: {:.2} | ", self.closed_avg)?;
        write!(f, "LowestPrice: {} | ", self.lowest_price)?;
        write!(f, "HighestPrice: {} | ", self.highest_price)?;
        if self.operations.is_empty() {
            write!(f, "Operations: None")
        } else {
            write!(f, "Operations: {}", self.operations)
        }
    }
}

// Extension trait for order
pub trait OrderExt {
    fn get_details(&self) -> OrderDetails;
    fn update_details(&mut self, details: OrderDetails) -> Self;
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
    fn apply_item_info_by_entry(
        &mut self,
        item_info: &Option<CacheTradableItem>,
    ) -> Result<(), Error>;
    fn update_string(&self) -> String;
}

impl OrderExt for Order {
    fn get_details(&self) -> OrderDetails {
        if let Some(properties) = &self.properties {
            serde_json::from_value(properties.clone()).unwrap_or_else(|_| OrderDetails::default())
        } else {
            OrderDetails::default()
        }
    }

    fn update_details(&mut self, details: OrderDetails) -> Self {
        self.properties = Some(serde_json::to_value(details).unwrap());
        self.clone()
    }
    fn apply_item_info_by_entry(
        &mut self,
        item_info: &Option<CacheTradableItem>,
    ) -> Result<(), Error> {
        if let Some(item_info) = item_info {
            self.update_details(self.get_details().set_info(item_info));
        }
        Ok(())
    }
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        match cache.tradable_item().get_by(&self.item_id) {
            Ok(item) => {
                self.apply_item_info_by_entry(&Some(item))?;
            }
            Err(_) => {
                warning(
                    "Order",
                    format!(
                        "Failed to apply item info for Order ID: {} with Item ID: {}",
                        self.id, self.item_id
                    ),
                    &LoggerOptions::default(),
                );
            }
        }
        Ok(())
    }
    fn update_string(&self) -> String {
        format!("p:{}", self.to_order().platinum)
    }
}
