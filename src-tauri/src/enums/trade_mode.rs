use serde::{Deserialize, Serialize};

use crate::{logger, utils::modules::logger::LoggerOptions};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TradeMode {
    All,
    Buy,
    Sell,
    WishList,
    Unknown(String),
}
impl Serialize for TradeMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TradeMode::All => "all",
            TradeMode::Buy => "buy",
            TradeMode::Sell => "sell",
            TradeMode::WishList => "wishlist",
            TradeMode::Unknown(i) => {
                logger::critical(
                    "TradeMode",
                    format!("Unknown TradeMode: {}", i).as_str(),
                    LoggerOptions::default().set_file("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TradeMode {
    fn deserialize<D>(deserializer: D) -> Result<TradeMode, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "buy" => TradeMode::Buy,
            "sell" => TradeMode::Sell,
            "both" => TradeMode::All,
            "all" => TradeMode::All,
            "wishlist" => TradeMode::WishList,
            s => TradeMode::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for Color, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}
