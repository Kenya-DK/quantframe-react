use entity::stock::riven::{attribute::RivenAttribute, create::CreateStockRiven};
use serde::{Deserialize, Serialize};

use crate::{live_scraper::types::riven_extra_info::AuctionDetails, utils::modules::error::AppError};

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

    #[serde(rename = "closed")]
    pub closed: bool,

    #[serde(rename = "is_direct_sell")]
    pub is_direct_sell: bool,

    #[serde(rename = "id")]
    pub id: String,

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(default)]
    pub operation: Vec<String>,
    
    #[serde(rename = "info", default = "Default::default")]
    pub info: AuctionDetails,
}
// Implementing the Auction default
impl Default for Auction<String> {
    fn default() -> Self {
        Auction {
            visible: false,
            minimal_reputation: 0,
            item: AuctionItem::default(),
            buyout_price: None,
            note: "".to_string(),
            starting_price: 0,
            owner: "".to_string(),
            closed: false,
            is_direct_sell: false,
            id: "N/A".to_string(),
            operation: vec![],
            info: AuctionDetails::default(),
        }
    }
}
impl<T> Auction<T> {
    pub fn convert_to_create_stock(&self, buyout_price: i64) -> Result<CreateStockRiven, AppError> {
        let item = self.item.clone();
        if item.item_type != "riven" {
            return Err(AppError::new("Auction",eyre::eyre!("Item type is not riven")));
        }
        let stock_item = CreateStockRiven::new(
            item.weapon_url_name.unwrap_or("".to_string()),
            item.name.clone().unwrap_or("".to_string()),
            item.mastery_level.unwrap_or(8),
            item.re_rolls.unwrap_or(0),
            item.polarity.clone().unwrap_or("".to_string()),
            item.attributes.clone().unwrap_or(vec![]),
            item.mod_rank.unwrap_or(0),
            Some(buyout_price),
            Some(self.id.clone()),
            None
        );
        Ok(stock_item)
    }  
    pub fn set_similarity_riven(&mut self, attributes: Vec<RivenAttribute>) {
        let mut shared_count = 0;

        let right_attributes = self.item.attributes.as_ref().unwrap();
        let extra_attributes: Vec<_> = right_attributes
            .iter()
            .filter(|attr| {
                !attributes
                    .iter()
                    .any(|attr2| attr2.url_name == attr.url_name && attr2.positive == attr.positive)
            })
            .cloned()
            .collect();

        let missing_attributes: Vec<_> = attributes
            .iter()
            .filter(|attr| {
                !right_attributes
                    .iter()
                    .any(|attr2| attr2.url_name == attr.url_name && attr2.positive == attr.positive)
            })
            .cloned()
            .collect();

        let mut unique_attributes = std::collections::HashSet::new();
        attributes.iter().for_each(|attr| {
            unique_attributes.insert(format!("{}_{}", attr.url_name, attr.positive));
        });

        let total_unique_attributes_count = unique_attributes.len();

        for attr2 in right_attributes {
            let key = format!("{}_{}", attr2.url_name, attr2.positive);
            if unique_attributes.contains(&key) {
                shared_count += 1;
                unique_attributes.remove(&key); // Ensure the attribute is only counted once
            }
        }

        let similarity = Some(shared_count as f64 / total_unique_attributes_count as f64 * 100.0);
        self.item.similarity = similarity;
        self.item.extra_attributes = Some(extra_attributes);
        self.item.missing_attributes = Some(missing_attributes);
    }
}
