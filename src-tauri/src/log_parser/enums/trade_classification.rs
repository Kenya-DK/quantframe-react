use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeClassification {
    Sale,
    Purchase,
    Trade,
    Unknown,
}

impl TradeClassification {
    pub fn to_str(&self) -> &str {
        match self {
            TradeClassification::Sale => "Sale",
            TradeClassification::Purchase => "Purchase",
            TradeClassification::Trade => "Trade",
            TradeClassification::Unknown => "Unknown",
        }
    }
}
