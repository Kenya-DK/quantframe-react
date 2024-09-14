use serde::{Deserialize, Serialize};

use crate::logger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderMode {
    Buy,
    Sell,
    Both,
    Unknown(String),
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
