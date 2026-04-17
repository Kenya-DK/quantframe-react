use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeItemType {
    Ayatan,
    Platinum,
    Credits,
    MainBlueprint,
    Weapon,
    Relic,
    Component,
    Arcane,
    Lens,
    Mod,
    Fish,
    FusionCore,
    RivenVeiled,
    RivenPreVeiled,
    RivenUnVeiled,
    Imprint,
    Set,
    Unknown,
}
impl TradeItemType {
    pub fn to_tax(&self) -> i64 {
        match self {
            TradeItemType::RivenVeiled
            | TradeItemType::RivenPreVeiled
            | TradeItemType::RivenUnVeiled => 8000,
            TradeItemType::Platinum => 500,
            _ => 2000,
        }
    }
}
