use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarframeRootObject {
    #[serde(rename = "PlayerLevel", default)]
    pub mastery_rank: i64,

    #[serde(rename = "PremiumCredits", default)]
    pub platinum: i64,

    #[serde(rename = "RegularCredits", default)]
    pub credits: i64,

    #[serde(rename = "TradesRemaining", default)]
    pub trades_remaining: i64,

    #[serde(rename = "RawUpgrades", default)]
    pub raw_upgrades: Vec<MiscItem>,

    #[serde(rename = "Upgrades", default)]
    pub upgrades: Vec<MiscItem>,

    #[serde(rename = "Recipes", default)]
    pub recipes: Vec<MiscItem>,
}
impl Default for WarframeRootObject {
    fn default() -> Self {
        Self {
            platinum: 0,
            credits: 0,
            trades_remaining: 0,
            raw_upgrades: vec![],
            upgrades: vec![],
            recipes: vec![],
        }
    }
}
