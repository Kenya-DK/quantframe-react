use serde::{Deserialize, Serialize};
use wf_market::enums::OrderType;

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
    pub fn to_order_type(&self) -> OrderType {
        match self {
            TradeClassification::Sale => OrderType::Sell,
            TradeClassification::Purchase => OrderType::Buy,
            TradeClassification::Trade => OrderType::Buy,
            TradeClassification::Unknown => OrderType::Sell,
        }
    }
}
