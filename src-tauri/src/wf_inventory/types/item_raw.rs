use serde::{Deserialize, Serialize};

use crate::wf_inventory::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvItemRaw {
    #[serde(rename = "ItemId", default)]
    pub id: ItemId,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,

    #[serde(rename = "ItemType", default)]
    pub unique_name: String,

    #[serde(rename = "UpgradeFingerprint", skip_serializing_if = "Option::is_none")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(rename = "LastAdded", default)]
    pub last_added: ItemId,

    #[serde(rename = "XP", default)]
    pub xp: i64,
}

impl WFInvItemRaw {
    pub fn is_riven(&self) -> bool {
        if !self
            .unique_name
            .starts_with("/Lotus/Upgrades/Mods/Randomized/")
        {
            return false;
        }
        if !self.get_upgrade_fingerprint().is_riven_unveiled() && self.id.id.is_some() {
            return true;
        } else if self.id.id.is_none() {
            return true;
        } else if self.get_upgrade_fingerprint().is_riven_unveiled() {
            return true;
        }
        false
    }
    pub fn is_arcane(&self) -> bool {
        !self.unique_name.contains("/CosmeticEnhancers/Peculiars/")
            && self
                .unique_name
                .contains("/Lotus/Upgrades/CosmeticEnhancers")
    }
    pub fn is_mod(&self) -> bool {
        !self.unique_name.contains("/Beginner/")
            && (self.unique_name.contains("/CosmeticEnhancers/Peculiars/")
                || self.unique_name.contains("/Lotus/Upgrades/Mods/Railjack/")
                || !self.is_arcane())
    }
    pub fn get_upgrade_fingerprint(&self) -> UpgradeFingerprint {
        if self.upgrade_fingerprint.is_none() {
            return UpgradeFingerprint::default();
        }
        UpgradeFingerprint::from(self.upgrade_fingerprint.as_ref().unwrap())
    }
}
