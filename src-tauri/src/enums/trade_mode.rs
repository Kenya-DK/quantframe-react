use serde::{Deserialize, Serialize};

use utils::{log_critical_opt, LoggerOptions};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TradeMode {
    Buy,
    Sell,
    WishList,
    Unknown(String),
}
impl TradeMode {
    pub fn to_string(&self) -> String {
        match self {
            TradeMode::Buy => "buy".to_string(),
            TradeMode::Sell => "sell".to_string(),
            TradeMode::WishList => "wishlist".to_string(),
            TradeMode::Unknown(i) => {
                log_critical_opt!(
                    "TradeMode",
                    &LoggerOptions::default().set_file("enums.log"),
                    "Unknown TradeMode: {}",
                    i
                );
                "unknown".to_string()
            }
        }
    }
}
impl Serialize for TradeMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
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
