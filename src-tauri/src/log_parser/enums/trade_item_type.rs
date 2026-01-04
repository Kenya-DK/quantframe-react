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
impl TradeItemType {
    pub fn to_string(&self) -> String {
        match self {
            TradeItemType::Ayatan => "Ayatan".to_string(),
            TradeItemType::Platinum => "Platinum".to_string(),
            TradeItemType::Credits => "Credits".to_string(),
            TradeItemType::MainBlueprint => "MainBlueprint".to_string(),
            TradeItemType::Weapon => "Weapon".to_string(),
            TradeItemType::Relic => "Relic".to_string(),
            TradeItemType::Component => "Component".to_string(),
            TradeItemType::Arcane => "Arcane".to_string(),
            TradeItemType::Lens => "Lens".to_string(),
            TradeItemType::Mod => "Mod".to_string(),
            TradeItemType::Fish => "Fish".to_string(),
            TradeItemType::FusionCore => "FusionCore".to_string(),
            TradeItemType::RivenVeiled => "RivenVeiled".to_string(),
            TradeItemType::RivenUnVeiled => "RivenUnVeiled".to_string(),
            TradeItemType::Imprint => "Imprint".to_string(),
            TradeItemType::Set => "Set".to_string(),
            TradeItemType::Unknown => "Unknown".to_string(),
        }
    }
}
