use serde::{Deserialize, Serialize};

use crate::wfm_client::types::{auction::Auction, auction_owner::AuctionOwner};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockRivenDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "total_sellers")]
    pub total_sellers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profit")]
    pub profit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lowest_price")]
    pub lowest_price: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highest_price")]
    pub highest_price: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "auctions")]
    pub auctions:  Option<Vec<Auction<AuctionOwner>>>,
}

impl StockRivenDetails {
    pub fn new(
        total_sellers: Option<i64>,
        profit: Option<i64>,
        lowest_price: Option<i64>,
        highest_price: Option<i64>,
        auctions: Option<Vec<Auction<AuctionOwner>>>
    ) -> StockRivenDetails {
        StockRivenDetails {
            total_sellers,
            profit,
            lowest_price,
            highest_price,
            auctions,
        }
    }
}