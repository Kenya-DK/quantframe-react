use std::{collections::VecDeque, fmt::Display};

use entity::dto::*;
use qf_api::errors::ApiError as QFRequestError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::{Error, LogLevel};
use wf_market::types::{Auction, AuctionWithOwner};

use crate::{cache::client::CacheState, enums::FindBy};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionDetails {
    pub auction_id: String,
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

    #[serde(rename = "auctions")]
    #[serde(default)]
    pub auctions: Vec<AuctionWithOwner>,

    // Item Info
    pub item_name: String,
    pub image_url: String,
}
impl AuctionDetails {
    pub fn set_auction_id(mut self, auction_id: impl Into<String>) -> Self {
        self.auction_id = auction_id.into();
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
    pub fn set_auctions(mut self, auctions: Vec<AuctionWithOwner>) -> Self {
        self.auctions = auctions;
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
    pub fn set_image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = image_url.into();
        self
    }
    pub fn set_item_name(mut self, item_name: impl Into<String>) -> Self {
        self.item_name = item_name.into();
        self
    }
}
// Default implementation for AuctionDetails
impl Default for AuctionDetails {
    fn default() -> Self {
        AuctionDetails {
            auction_id: String::new(),
            lowest_price: 0,
            highest_price: 0,
            operations: vec!["Create".to_string()],
            auctions: vec![],
            item_name: String::new(),
            image_url: String::new(),
        }
    }
}

impl Display for AuctionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuctionDetails: ")?;
        write!(f, "Auction ID: {}", self.auction_id)?;
        if self.operations.is_empty() {
            write!(f, "Operations: None")
        } else {
            write!(f, "Operations: {}", self.operations.join(", "))
        }
    }
}

// Extension trait for auction
pub trait AuctionExt {
    fn get_details(&self) -> AuctionDetails;
    fn update_details(&mut self, details: AuctionDetails) -> Self;
}

impl AuctionExt for Auction {
    fn get_details(&self) -> AuctionDetails {
        if let Some(properties) = &self.properties {
            serde_json::from_value(properties.clone()).unwrap_or_else(|_| AuctionDetails::default())
        } else {
            AuctionDetails::default()
        }
    }

    fn update_details(&mut self, details: AuctionDetails) -> Self {
        self.properties = Some(serde_json::to_value(details).unwrap());
        self.clone()
    }
}
impl AuctionExt for AuctionWithOwner {
    fn get_details(&self) -> AuctionDetails {
        if let Some(properties) = &self.auction.properties {
            serde_json::from_value(properties.clone()).unwrap_or_else(|_| AuctionDetails::default())
        } else {
            AuctionDetails::default()
        }
    }

    fn update_details(&mut self, details: AuctionDetails) -> Self {
        self.auction.properties = Some(serde_json::to_value(details).unwrap());
        self.clone()
    }
}
