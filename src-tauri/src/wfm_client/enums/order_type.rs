use serde::{Deserialize, Serialize};

use crate::logger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
    Unknown(String),
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
