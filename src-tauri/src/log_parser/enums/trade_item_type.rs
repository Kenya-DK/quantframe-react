use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum TradeItemType {
    Platinum,
    RivenVeiled,
    RivenUnVeiled,
    Weapon,
    Relic,
    MainBlueprint,
    Component,
    Lens,
    Arcane,
    Mod,
    FusionCore,
    Fish,
    Ayatan,
    Imprint,
    Unknown,
}
