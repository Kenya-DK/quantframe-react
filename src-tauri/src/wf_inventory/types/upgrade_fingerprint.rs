use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeStat {
    #[serde(rename = "Tag", default)]
    pub tag: String,

    #[serde(rename = "Value", default)]
    pub value: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeFingerprint {
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Value>,

    #[serde(rename = "lvlReq", default)]
    pub mastery_rank: i64,

    #[serde(rename = "rerolls", default)]
    pub rerolls: i64,

    #[serde(rename = "lvl", default)]
    pub mod_rank: i64,

    #[serde(rename = "compat", default)]
    pub compatibility: String,

    #[serde(rename = "pol", default)]
    pub polarity: String,

    #[serde(rename = "buffs", default)]
    pub buffs: Vec<UpgradeStat>,

    #[serde(rename = "curses", default)]
    pub curses: Vec<UpgradeStat>,
}

impl Default for UpgradeFingerprint {
    fn default() -> Self {
        Self {
            polarity: String::new(),
            compatibility: String::new(),
            mod_rank: 0,
            rerolls: 0,
            mastery_rank: 0,
            buffs: Vec::new(),
            curses: Vec::new(),
            challenge: None,
        }
    }
}

impl UpgradeFingerprint {
    pub fn is_riven_unveiled(&self) -> bool {
        self.challenge.is_some()
    }
    pub fn riven_stat_totals(&self) -> (usize, usize) {
        (self.buffs.len(), self.curses.len())
    }
}

impl From<&String> for UpgradeFingerprint {
    fn from(s: &String) -> Self {
        serde_json::from_str(s).unwrap_or_default()
    }
}
