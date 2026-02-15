use serde::{Deserialize, Serialize};

use crate::wf_inventory::{enums::RivenType, UpgradeFingerprint};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LastAdded {
    #[serde(rename = "oid", default)]
    pub id: String,
}
impl Default for LastAdded {
    fn default() -> Self {
        Self { id: String::new() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiscItemItemId {
    #[serde(rename = "$oid", skip_serializing_if = "Option::is_none")]
    pub old_id: Option<String>,
}

impl Default for MiscItemItemId {
    fn default() -> Self {
        Self { old_id: None }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiscItem {
    #[serde(rename = "ItemId", default)]
    pub id: MiscItemItemId,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,

    #[serde(rename = "ItemType", default)]
    pub unique_name: String,

    #[serde(rename = "UpgradeFingerprint", skip_serializing_if = "Option::is_none")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(rename = "LastAdded", default)]
    pub last_added: LastAdded,

    #[serde(rename = "XP", default)]
    pub xp: i64,
}

impl MiscItem {
    pub fn is_riven(&self, riven_type: RivenType) -> bool {
        if !self
            .unique_name
            .starts_with("/Lotus/Upgrades/Mods/Randomized/")
        {
            return false;
        }
        match riven_type {
            RivenType::Veiled => {
                !self.get_upgrade_fingerprint().is_riven_unveiled() && self.id.old_id.is_some()
            }
            RivenType::PreVeiled => self.id.old_id.is_none(),
            RivenType::UnVeiled => self.get_upgrade_fingerprint().is_riven_unveiled(),
        }
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
