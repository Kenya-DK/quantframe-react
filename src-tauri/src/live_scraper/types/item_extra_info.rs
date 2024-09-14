use serde::{Deserialize, Serialize};

use crate::wfm_client::types::{order::Order};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockItemDetails {
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
    #[serde(rename = "moving_avg")]
    pub moving_avg: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orders")]
    pub orders: Option<Vec<Order>>,
}

impl StockItemDetails {
    pub fn new(
        total_sellers: Option<i64>,
        profit: Option<i64>,
        lowest_price: Option<i64>,
        moving_avg: Option<i64>,
        highest_price: Option<i64>,
        orders: Option<Vec<Order>>,
    ) -> StockItemDetails {
        StockItemDetails {
            total_sellers,
            profit,
            lowest_price,
            moving_avg,
            highest_price,
            orders,
        }
    }
}
