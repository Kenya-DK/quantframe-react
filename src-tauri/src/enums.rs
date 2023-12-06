use serde::{Deserialize, Serialize};

use crate::logger;

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
    Unknown(String),
}
impl OrderType {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
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
    pub fn as_str(&self) -> &str {
        match *self {
            StockMode::All => "all",
            StockMode::Item => "item",
            StockMode::Riven => "riven",
            StockMode::Unknown(ref i) => i,
        }
    }
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
    pub fn as_str(&self) -> &str {
        match *self {
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
            LogLevel::Critical => "critical",
            LogLevel::Unknown(ref i) => i,
        }
    }
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
