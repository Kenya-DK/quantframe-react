use serde::{Deserialize, Serialize};

use crate::wfm_client::enums::order_type::OrderType;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeItemType {
    Platinum,
    Riven,
    Mod,
    Arcane,
    PrimePart,
    PrimeSet,
    Other,
    Unknown,
}
