use crate::logger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderMode {
    Buy,
    Sell,
    Both,
    Unknown(String),
}
impl OrderMode {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            OrderMode::Buy => "buy",
            OrderMode::Sell => "sell",
            OrderMode::Both => "both",
            OrderMode::Unknown(ref i) => i,
        }
    }
}
impl Serialize for OrderMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            OrderMode::Buy => "buy",
            OrderMode::Sell => "sell",
            OrderMode::Both => "both",
            OrderMode::Unknown(i) => {
                logger::critical_file(
                    "OrderMode",
                    format!("Unknown OrderMode: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for OrderMode {
    fn deserialize<D>(deserializer: D) -> Result<OrderMode, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "buy" => OrderMode::Buy,
            "sell" => OrderMode::Sell,
            "both" => OrderMode::Both,
            s => OrderMode::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for Color, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
    BuySell,
    Unknown(String),
}
impl OrderType {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
            OrderType::BuySell => "buy_sell",
            OrderType::Unknown(ref i) => i,
        }
    }
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
            OrderType::BuySell => "buy_sell",
            OrderType::Unknown(i) => {
                logger::critical_file(
                    "OrderType",
                    format!("Unknown OrderMode: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<OrderType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "buy" => OrderType::Buy,
            "sell" => OrderType::Sell,
            s => OrderType::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for Color, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StockMode {
    All,
    Riven,
    Item,
    Unknown(String),
}
impl StockMode {
    // Create method to convert `OrderType` to a `&str`
    // pub fn as_str(&self) -> &str {
    //     match *self {
    //         StockMode::All => "all",
    //         StockMode::Item => "item",
    //         StockMode::Riven => "riven",
    //         StockMode::Unknown(ref i) => i,
    //     }
    // }
}
impl Serialize for StockMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            StockMode::All => "all",
            StockMode::Item => "item",
            StockMode::Riven => "riven",
            StockMode::Unknown(i) => {
                logger::critical_file(
                    "OrderMode",
                    format!("Unknown OrderMode: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for StockMode {
    fn deserialize<D>(deserializer: D) -> Result<StockMode, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => StockMode::All,
            "item" => StockMode::Item,
            "riven" => StockMode::Riven,
            s => StockMode::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for Color, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
    Critical,
    Unknown(String),
}
impl LogLevel {
    // Create method to convert `OrderType` to a `&str`
    // pub fn as_str(&self) -> &str {
    //     match *self {
    //         LogLevel::Info => "info",
    //         LogLevel::Warning => "warning",
    //         LogLevel::Error => "error",
    //         LogLevel::Debug => "debug",
    //         LogLevel::Trace => "trace",
    //         LogLevel::Critical => "critical",
    //         LogLevel::Unknown(ref i) => i,
    //     }
    // }
}
impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
            LogLevel::Critical => "critical",
            LogLevel::Unknown(i) => {
                logger::critical_file(
                    "OrderMode",
                    format!("Unknown OrderMode: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<LogLevel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info" => LogLevel::Info,
            "warning" => LogLevel::Warning,
            "error" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            "critical" => LogLevel::Critical,
            s => LogLevel::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for LogLevel, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeClassification {
    Sale,
    Purchase,
    Trade,
    Unknown,
}
impl TradeClassification {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            TradeClassification::Sale => "sale",
            TradeClassification::Purchase => "purchase",
            TradeClassification::Trade => "trade",
            TradeClassification::Unknown => "unknown",
        }
    }
    pub fn display(&self) -> &str {
        match *self {
            TradeClassification::Sale => "Sale",
            TradeClassification::Purchase => "Purchase",
            TradeClassification::Trade => "Trade",
            TradeClassification::Unknown => "Unknown",
        }
    }
}

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
