use serde::{Deserialize, Serialize};

use crate::wf_inventory::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarframeRootObject {
    #[serde(rename = "PlayerLevel", default)]
    pub mastery_rank: i64,

    #[serde(rename = "PremiumCredits", default)]
    pub platinum: i64,

    #[serde(rename = "RegularCredits", default)]
    pub credits: i64,

    #[serde(rename = "DailyAffiliation", default)]
    pub daily_affiliation_syndicate: i64,

    #[serde(rename = "DailyAffiliationPvp", default)]
    pub daily_affiliation_pvp: i64,

    #[serde(rename = "DailyAffiliationLibrary", default)]
    pub daily_affiliation_library: i64,

    #[serde(rename = "DailyAffiliationCetus", default)]
    pub daily_affiliation_cetus: i64,

    #[serde(rename = "DailyAffiliationQuills", default)]
    pub daily_affiliation_quills: i64,

    #[serde(rename = "DailyAffiliationVentkids", default)]
    pub daily_affiliation_ventkids: i64,

    #[serde(rename = "DailyAffiliationVox", default)]
    pub daily_affiliation_vox: i64,

    #[serde(rename = "DailyAffiliationEntrati", default)]
    pub daily_affiliation_entrati: i64,

    #[serde(rename = "DailyAffiliationZariman", default)]
    pub daily_affiliation_zariman: i64,

    #[serde(rename = "DailyAffiliationNecraloid", default)]
    pub daily_affiliation_necraloid: i64,

    #[serde(rename = "DailyAffiliationKahl", default)]
    pub daily_affiliation_kahl: i64,

    #[serde(rename = "DailyAffiliationCavia", default)]
    pub daily_affiliation_cavia: i64,

    #[serde(rename = "DailyAffiliationHex", default)]
    pub daily_affiliation_hex: i64,

    #[serde(rename = "TradesRemaining", default)]
    pub trades_remaining: i64,

    #[serde(rename = "RawUpgrades", default)]
    pub raw_upgrades: Vec<WFInvItemRaw>,

    #[serde(rename = "Upgrades", default)]
    pub upgrades: Vec<WFInvItemRaw>,

    #[serde(rename = "Recipes", default)]
    pub recipes: Vec<WFInvItemRaw>,

    #[serde(rename = "Affiliations", default)]
    pub affiliations: Vec<WFInvAffiliation>,
}
impl Default for WarframeRootObject {
    fn default() -> Self {
        Self {
            mastery_rank: 0,
            platinum: 0,
            credits: 0,
            daily_affiliation_syndicate: 0,
            daily_affiliation_pvp: 0,
            daily_affiliation_library: 0,
            daily_affiliation_cetus: 0,
            daily_affiliation_quills: 0,
            daily_affiliation_ventkids: 0,
            daily_affiliation_vox: 0,
            daily_affiliation_entrati: 0,
            daily_affiliation_zariman: 0,
            daily_affiliation_necraloid: 0,
            daily_affiliation_kahl: 0,
            daily_affiliation_cavia: 0,
            daily_affiliation_hex: 0,
            trades_remaining: 0,
            raw_upgrades: vec![],
            upgrades: vec![],
            recipes: vec![],
            affiliations: vec![],
        }
    }
}
