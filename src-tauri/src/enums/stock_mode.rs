use serde::{Deserialize, Serialize};

use crate::logger;

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
