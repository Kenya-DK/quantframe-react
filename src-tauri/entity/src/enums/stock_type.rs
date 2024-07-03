use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
#[derive(Eq)]
pub enum StockType {
    #[sea_orm(string_value = "riven")]
    Riven,
    #[sea_orm(string_value = "item")]
    Item,
    #[sea_orm(string_value = "unknown")]
    Unknown
}
impl StockType {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            StockType::Riven => "riven",
            StockType::Item => "item",
            StockType::Unknown => "unknown"
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            StockType::Riven => StockType::Riven.as_str().to_string(),
            StockType::Item => StockType::Item.as_str().to_string(),
            StockType::Unknown => StockType::Unknown.as_str().to_string()
        }
    }
    pub fn from_string(s: &str) -> StockType {
        match s {
            "riven" => StockType::Riven,
            "item" => StockType::Item,
            _ => StockType::Unknown
        }
    }
}
impl Serialize for StockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            StockType::Riven => StockType::Riven.as_str(),
            StockType::Item => StockType::Item.as_str(),
            StockType::Unknown => StockType::Unknown.as_str()
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for StockType {
    fn deserialize<D>(deserializer: D) -> Result<StockType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "riven" => StockType::Riven,
            "item" => StockType::Item,
            _ => StockType::Unknown
        })
    }
}
