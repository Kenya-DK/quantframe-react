use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeItemType {
    Platinum,
    RivenVeiled,
    RivenUnVeiled,
    Relic,
    MainBlueprint,
    Component,
    Lens,
    Arcane,
    Mod,
    Unknown,
}
