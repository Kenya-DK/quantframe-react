use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

use crate::{
    live_scraper::types::order_extra_info::OrderDetails, wfm_client::enums::order_type::OrderType,
};

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

    #[serde(skip_serializing_if = "Option::is_none")]
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

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "item")]
    pub item: Option<OrderItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profit")]
    pub profit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closed_avg")]
    pub closed_avg: Option<f64>,

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(default)]
    pub operation: Vec<String>,

    // Ignore this field
    #[serde(skip_serializing)]
    #[serde(default)]
    pub tags: Vec<String>,

    // Subtype's
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cyan_stars")]
    pub cyan_stars: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amber_stars")]
    pub amber_stars: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subtype")]
    pub subtype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "info", default = "Default::default")]
    pub info: OrderDetails,
}
impl Default for Order {
    fn default() -> Self {
        Self {
            id: "N/A".to_string(),
            platinum: 0,
            visible: false,
            order_type: OrderType::Buy,
            user: None,
            last_update: "".to_string(),
            region: "".to_string(),
            platform: "".to_string(),
            creation_date: "".to_string(),
            quantity: 0,
            operation: vec!["New".to_string()],
            tags: vec![],
            item: None,
            profit: None,
            closed_avg: None,
            cyan_stars: None,
            amber_stars: None,
            subtype: None,
            mod_rank: None,
            info: Default::default(),
        }
    }
}
impl Order {
    pub fn get_subtype(&self) -> Option<SubType> {
        if self.subtype.is_none()
            && self.mod_rank.is_none()
            && self.amber_stars.is_none()
            && self.cyan_stars.is_none()
        {
            return None;
        }
        return Some(SubType::new(
            self.mod_rank,
            self.subtype.clone(),
            self.amber_stars,
            self.cyan_stars,
        ));
    }
}
