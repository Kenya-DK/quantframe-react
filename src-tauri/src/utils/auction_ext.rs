use std::fmt::Display;

use entity::stock_riven::{CreateStockRiven, RivenAttribute};
use serde::{Deserialize, Serialize};
use utils::{get_location, warning, Error, LoggerOptions};
use wf_market::{
    enums::AuctionType,
    types::{Auction, AuctionWithOwner},
};

use crate::{
    cache::{client::CacheState, types::CacheRivenWeapon},
    enums::FindBy,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionDetails {
    pub auction_id: String,
    #[serde(default)]
    #[serde(rename = "lowest_price")]
    pub lowest_price: i64,

    #[serde(default)]
    #[serde(rename = "highest_price")]
    pub highest_price: i64,

    #[serde(default)]
    #[serde(rename = "profit")]
    pub profit: i64,

    // Default implementation for string
    #[serde(rename = "operation")]
    #[serde(default)]
    pub operations: Vec<String>,

    #[serde(rename = "auctions")]
    #[serde(default)]
    pub auctions: Vec<AuctionWithOwner>,

    #[serde(default)]
    pub can_import: bool,

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
    pub fn set_profit(mut self, profit: i64) -> Self {
        self.profit = profit;
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

    pub fn set_can_import(mut self, can_import: bool) -> Self {
        self.can_import = can_import;
        self
    }

    pub fn set_info(mut self, info: &CacheRivenWeapon) -> Self {
        self.item_name = info.name.clone();
        self.image_url = info.wfm_icon.clone();
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
            profit: 0,
            operations: vec!["Create".to_string()],
            auctions: vec![],
            item_name: String::new(),
            image_url: String::new(),
            can_import: false,
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
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
    fn apply_item_info_by_entry(
        &mut self,
        item_info: &Option<CacheRivenWeapon>,
    ) -> Result<(), Error>;
    fn to_create(&self) -> Result<CreateStockRiven, Error>;
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
    fn apply_item_info_by_entry(
        &mut self,
        item_info: &Option<CacheRivenWeapon>,
    ) -> Result<(), Error> {
        if let Some(item_info) = item_info {
            self.update_details(self.get_details().set_info(item_info));
        }
        Ok(())
    }
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        if let Ok(item) = cache.riven().get_riven_by(FindBy::new(
            crate::enums::FindByType::Url,
            &self.item.weapon_url_name,
        )) {
            self.apply_item_info_by_entry(&item)?;
        } else {
            warning(
                "Auction",
                format!("Item info not found for url: {}", self.item.weapon_url_name),
                &LoggerOptions::default(),
            );
        }
        Ok(())
    }

    fn to_create(&self) -> Result<CreateStockRiven, Error> {
        if self.item.item_type != AuctionType::Riven {
            return Err(Error::new(
                "ToCreateStockRiven",
                "Cant only create stock riven from riven auction",
                get_location!(),
            ));
        }
        let item = CreateStockRiven::new(
            self.item.weapon_url_name.clone(),
            self.item.mod_name.clone().unwrap_or(String::new()),
            self.item.mastery_level.unwrap_or(0).into(),
            self.item.re_rolls.unwrap_or(0).into(),
            self.item.polarity.clone().unwrap_or(String::new()),
            self.item
                .attributes
                .clone()
                .unwrap_or(vec![])
                .to_vec()
                .iter()
                .map(|a| RivenAttribute::new(a.positive, a.value, a.url_name.clone()))
                .collect(),
            self.item.mod_rank.unwrap_or(0).into(),
        );
        Ok(item)
    }
}
