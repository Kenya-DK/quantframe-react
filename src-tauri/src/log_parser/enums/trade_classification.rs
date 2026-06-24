use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TradeClassification {
    Sale,
    Purchase,
    Trade,
    Any,
    Unknown,
}

impl TradeClassification {
    pub fn to_string(&self) -> String {
        match self {
            TradeClassification::Sale => "sale".to_string(),
            TradeClassification::Purchase => "purchase".to_string(),
            TradeClassification::Trade => "trade".to_string(),
            TradeClassification::Any => "any".to_string(),
            TradeClassification::Unknown => "unknown".to_string(),
        }
    }
    pub fn display(&self) -> String {
        match self {
            TradeClassification::Sale => "Sale".to_string(),
            TradeClassification::Purchase => "Purchase".to_string(),
            TradeClassification::Trade => "Trade".to_string(),
            TradeClassification::Any => "Any".to_string(),
            TradeClassification::Unknown => "Unknown".to_string(),
        }
    }
    pub fn classify_trade(
        offer_plat: i64,
        receive_plat: i64,
        offered: usize,
        received: usize,
    ) -> TradeClassification {
        match (offer_plat > 1, receive_plat > 1) {
            (true, _) if offered == 1 => TradeClassification::Purchase,
            (_, true) if received == 1 => TradeClassification::Sale,
            _ => TradeClassification::Trade,
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
            TradeClassification::Any => "any",
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
            "any" => TradeClassification::Any,
            "unknown" => TradeClassification::Unknown,
            _ => panic!("Invalid transaction type"),
        })
    }
}
