use serde::{Deserialize, Serialize};

use crate::wfm_client::enums::order_type::OrderType;

use super::{order_item::OrderItem, user::User};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "platinum")]
    pub platinum: i64,

    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "order_type")]
    pub order_type: OrderType,

    #[serde(rename = "user")]
    pub user: Option<User>,

    #[serde(rename = "last_update")]
    pub last_update: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "creation_date")]
    pub creation_date: String,

    #[serde(rename = "subtype")]
    pub subtype: Option<String>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "item")]
    pub item: Option<OrderItem>,

    #[serde(rename = "profit")]
    pub profit: Option<f64>,

    #[serde(rename = "closed_avg")]
    pub closed_avg: Option<f64>,
}
