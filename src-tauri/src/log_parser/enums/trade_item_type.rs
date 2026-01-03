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
    RivenUnVeiled,
    Imprint,
    Set,
    Unknown,
}
