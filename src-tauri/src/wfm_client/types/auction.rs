use entity::{enums::stock_type::StockType, stock::item, sub_type::SubType};
use serde::{Deserialize, Serialize};

use crate::{log_parser::types::create_stock_entity::CreateStockEntity, utils::modules::error::AppError};

use super::auction_item::AuctionItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auction<T> {
    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "minimal_reputation")]
    pub minimal_reputation: i64,

    #[serde(rename = "item")]
    pub item: AuctionItem,

    #[serde(rename = "buyout_price")]
    pub buyout_price: Option<i64>,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "starting_price")]
    pub starting_price: i64,

    #[serde(rename = "owner")]
    pub owner: T,

    // #[serde(rename = "platform")]
    // pub platform: String,
    #[serde(rename = "closed")]
    pub closed: bool,

    // #[serde(rename = "top_bid")]
    // pub top_bid: Option<serde_json::Value>,

    // #[serde(rename = "winner")]
    // pub winner: Option<serde_json::Value>,

    // #[serde(rename = "is_marked_for")]
    // pub is_marked_for: Option<serde_json::Value>,

    // #[serde(rename = "marked_operation_at")]
    // pub marked_operation_at: Option<serde_json::Value>,

    // #[serde(rename = "created")]
    // pub created: String,

    // #[serde(rename = "updated")]
    // pub updated: String,

    // #[serde(rename = "note_raw")]
    // pub note_raw: String,
    #[serde(rename = "is_direct_sell")]
    pub is_direct_sell: bool,

    #[serde(rename = "id")]
    pub id: String,
    // #[serde(rename = "private")]
    // pub private: bool,
}
impl Auction<String> {
    pub fn convert_to_create_stock(&self,buyout_price: i64) -> Result<CreateStockEntity, AppError> {
        let item = self.item.clone();
        if item.item_type != "riven" {
            return Err(AppError::new("Auction",eyre::eyre!("Item type is not riven")));
        }
        let mut stock = CreateStockEntity::new(&item.weapon_url_name.unwrap_or("".to_string()),buyout_price);
        // Get Item
       stock.entity_type = StockType::Riven;
       stock.mod_name = item.name.clone().unwrap_or("".to_string());
       stock.mastery_rank = item.mastery_level.unwrap_or(0);
       stock.re_rolls = item.re_rolls.unwrap_or(0);
       stock.polarity = item.polarity.clone().unwrap_or("".to_string());
       stock.attributes = item.attributes.clone().unwrap_or(vec![]);
       stock.sub_type = Some(SubType {
            rank: item.mod_rank,
            variant: None,
            cyan_stars: None,
            amber_stars: None,
        });
        Ok(stock)
    }  
}
