use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TradeClassification {
    Sale,
    Purchase,
    Trade,
    Unknown,
}

impl TradeClassification {
    pub fn to_string(&self) -> String {
        match self {
            TradeClassification::Sale => "sale".to_string(),
            TradeClassification::Purchase => "purchase".to_string(),
            TradeClassification::Trade => "trade".to_string(),
            TradeClassification::Unknown => "unknown".to_string(),
        }
    }
}
impl Serialize for TradeClassification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TradeClassification::Purchase => "purchase",
            TradeClassification::Sale => "sale",
            TradeClassification::Trade => "trade",
            TradeClassification::Unknown => "unknown",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TradeClassification {
    fn deserialize<D>(deserializer: D) -> Result<TradeClassification, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "purchase" => TradeClassification::Purchase,
            "sale" => TradeClassification::Sale,
            "trade" => TradeClassification::Trade,
            "unknown" => TradeClassification::Unknown,
            _ => panic!("Invalid transaction type"),
        })
    }
}
