use std::{collections::VecDeque, fmt::Display};

use entity::dto::*;
use qf_api::errors::ApiError as QFRequestError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::{Error, LogLevel};
use wf_market::types::{order, Order, OrderWithUser};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDetails {
    pub order_id: String,
    pub item_id: String,
    pub sub_type: Option<SubType>,
    pub item_name: String,
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

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(default)]
    pub operations: Vec<String>,

    #[serde(rename = "orders")]
    #[serde(default)]
    pub orders: Vec<OrderWithUser>,
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
        self.operations = operation.iter().map(|&s| s.to_string()).collect();
        self
    }
    pub fn add_operation(&mut self, operation: impl Into<String>) {
        self.operations.push(operation.into());
    }
    pub fn has_operation(&self, operation: impl Into<String>) -> bool {
        let operation = operation.into();
        self.operations.iter().any(|op| op == &operation)
    }
    pub fn set_order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = order_id.into();
        self
    }
    pub fn set_item_id(mut self, item_id: impl Into<String>) -> Self {
        self.item_id = item_id.into();
        self
    }
    pub fn set_item_name(mut self, item_name: impl Into<String>) -> Self {
        self.item_name = item_name.into();
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
}
// Default implementation for OrderDetails
impl Default for OrderDetails {
    fn default() -> Self {
        OrderDetails {
            order_id: String::from("N/A"),
            item_id: String::from("N/A"),
            sub_type: None,
            item_name: String::from("Unknown Item"),
            quantity: 1,
            closed_avg: 0.0,
            profit: 0.0,
            lowest_price: -1,
            highest_price: -1,
            operations: vec!["Create".to_string()],
            orders: vec![],
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
            write!(f, "Operations: {}", self.operations.join(", "))
        }
    }
}

// Extension trait for order
pub trait OrderExt {
    fn get_details(&self) -> OrderDetails;
    fn update_details(&mut self, details: OrderDetails) -> Self;
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
}
