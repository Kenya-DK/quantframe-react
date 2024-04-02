use serde::{Deserialize, Serialize};

use crate::logger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StockStatus {
    Pending,
    Live,
    ToLowProfit,
    NoSellers,
    NoBuyers,
    InActive,
    SMALimit,
    OrderLimit,
    Overpriced,
    Underpriced,
    Unknown(String),
}
impl StockStatus {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            StockStatus::Pending => "pending",
            StockStatus::Live => "live",
            StockStatus::ToLowProfit => "to_low_profit",
            StockStatus::NoSellers => "no_sellers",
            StockStatus::NoBuyers => "no_buyers",
            StockStatus::InActive => "inactive",
            StockStatus::SMALimit => "sma_limit",
            StockStatus::OrderLimit => "order_limit",
            StockStatus::Overpriced => "overpriced",
            StockStatus::Underpriced => "underpriced",
            StockStatus::Unknown(ref i) => i,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            StockStatus::Pending => StockStatus::Pending.as_str().to_string(),
            StockStatus::Live => StockStatus::Live.as_str().to_string(),
            StockStatus::ToLowProfit => StockStatus::ToLowProfit.as_str().to_string(),
            StockStatus::NoSellers => StockStatus::NoSellers.as_str().to_string(),
            StockStatus::NoBuyers => StockStatus::NoBuyers.as_str().to_string(),
            StockStatus::InActive => StockStatus::InActive.as_str().to_string(),
            StockStatus::SMALimit => StockStatus::SMALimit.as_str().to_string(),
            StockStatus::OrderLimit => StockStatus::OrderLimit.as_str().to_string(),
            StockStatus::Overpriced => StockStatus::Overpriced.as_str().to_string(),
            StockStatus::Underpriced => StockStatus::Underpriced.as_str().to_string(),
            StockStatus::Unknown(i) => i.to_string(),
        }
    }
    pub fn from_string(s: &str) -> StockStatus {
        match s {
            "pending" => StockStatus::Pending,
            "live" => StockStatus::Live,
            "to_low_profit" => StockStatus::ToLowProfit,
            "no_sellers" => StockStatus::NoSellers,
            "no_buyers" => StockStatus::NoBuyers,
            "inactive" => StockStatus::InActive,
            "sma_limit" => StockStatus::SMALimit,
            "order_limit" => StockStatus::OrderLimit,
            "overpriced" => StockStatus::Overpriced,
            "underpriced" => StockStatus::Underpriced,
            s => StockStatus::Unknown(s.to_string()),
        }
    }
}
impl Serialize for StockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            StockStatus::Pending => "pending",
            StockStatus::Live => "live",
            StockStatus::ToLowProfit => "to_low_profit",
            StockStatus::NoSellers => "no_sellers",
            StockStatus::NoBuyers => "no_buyers",
            StockStatus::InActive => "inactive",
            StockStatus::SMALimit => "sma_limit",
            StockStatus::OrderLimit => "order_limit",
            StockStatus::Overpriced => "overpriced",
            StockStatus::Underpriced => "underpriced",
            StockStatus::Unknown(i) => {
                logger::critical_file(
                    "StockStatus",
                    format!("Unknown StockStatus: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for StockStatus {
    fn deserialize<D>(deserializer: D) -> Result<StockStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pending" => StockStatus::Pending,
            "live" => StockStatus::Live,
            "to_low_profit" => StockStatus::ToLowProfit,
            "no_sellers" => StockStatus::NoSellers,
            "no_buyers" => StockStatus::NoBuyers,
            "inactive" => StockStatus::InActive,
            "sma_limit" => StockStatus::SMALimit,
            "order_limit" => StockStatus::OrderLimit,
            "overpriced" => StockStatus::Overpriced,
            "underpriced" => StockStatus::Underpriced,
            s => StockStatus::Unknown(
                s.parse()
                    .map_err(|_| serde::de::Error::custom(format!("Unknown StockStatus: {}", s)))?,
            ),
        })
    }
}
