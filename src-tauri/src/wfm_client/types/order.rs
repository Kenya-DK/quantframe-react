use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

use crate::{
    live_scraper::types::order_extra_info::OrderDetails, wfm_client::enums::order_type::OrderType,
};

use super::{order_item::OrderItem, user::User};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub platinum: i64, // AKA the price
    pub quantity: i64,

    #[serde(rename = "perTrade", skip_serializing_if = "Option::is_none")]
    pub per_trade: Option<i64>, // Amount of items per trade

    // #[serde(flatten)]
    // pub subtype: SubType, // Subtype for mods, ayatan sculptures, etc.
    pub visible: bool, // Whether the order is visible to other players

    #[serde(rename = "itemId")]
    pub item_id: String, // ID of the item

    #[serde(rename = "createdAt")]
    pub created_at: String, // Timestamp of when the order was created
    #[serde(rename = "updatedAt")]
    pub updated_at: String, // Timestamp of when the order was last updated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>, // Subtype of the item, if applicable

    // MODS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>, // Rank of the mod, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<i64>, // Charges remaining (Requiem mods)

    // AYATAN SCULPTURES
    #[serde(rename = "amberStars", skip_serializing_if = "Option::is_none")]
    pub amber_stars: Option<i64>, // Number of Amber Stars, if applicable
    #[serde(rename = "cyanStars", skip_serializing_if = "Option::is_none")]
    pub cyan_stars: Option<i64>, // Number of Cyan Stars, if applicable

    pub user: Option<User>, // User who created the order

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(default)]
    pub operation: Vec<String>,
    #[serde(rename = "info", default = "Default::default")]
    pub info: OrderDetails,
}
impl Default for Order {
    fn default() -> Self {
        Self {
            id: "N/A".to_string(),
            platinum: 0,
            per_trade: None,
            charges: None,
            visible: false,
            order_type: OrderType::Buy,
            quantity: 0,
            operation: vec!["New".to_string()],
            cyan_stars: None,
            amber_stars: None,
            subtype: None,
            user: None,
            item_id: "N/A".to_string(),
            created_at: "N/A".to_string(),
            updated_at: "N/A".to_string(),
            rank: None,
            info: Default::default(),
        }
    }
}
impl Order {
    pub fn get_subtype(&self) -> Option<SubType> {
        if self.subtype.is_none()
            && self.rank.is_none()
            && self.amber_stars.is_none()
            && self.cyan_stars.is_none()
        {
            return None;
        }
        return Some(SubType::new(
            self.rank,
            self.subtype.clone(),
            self.amber_stars,
            self.cyan_stars,
        ));
    }
}
