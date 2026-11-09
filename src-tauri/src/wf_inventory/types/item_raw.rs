use serde::{Deserialize, Serialize};

use crate::wf_inventory::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvItemRaw {
    #[serde(rename = "ItemId", default)]
    pub id: ItemId,

    // default quantity should be 1 if not specified
    #[serde(rename = "ItemCount", default = "default_quantity")]
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

fn default_quantity() -> i64 {
    1
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
    pub fn is_mod(&self) -> bool {
        self.unique_name.starts_with("/Lotus/Upgrades/Mods/") && !self.is_riven()
    }
    pub fn get_upgrade_fingerprint(&self) -> UpgradeFingerprint {
        if self.upgrade_fingerprint.is_none() {
            return UpgradeFingerprint::default();
        }
        UpgradeFingerprint::from(self.upgrade_fingerprint.as_ref().unwrap())
    }

    // Classification ported 1:1 from AlecaFrame's `Miscitem` class (AlecaFrameClientLib.Data.Types).
    // NOTE: these treat any non-arcane path as a mod — in AlecaFrame they classify Misc item
    // entries after the other item families were ruled out, so don't use them raw as a general
    // type detector; combine with the stricter `is_mod()` / `is_riven()` above when in doubt.

    pub fn is_warframe_part(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if !self.unique_name.contains("/WarframeRecipes/") {
            return false;
        }
        if self.unique_name.contains("Beacon") && self.unique_name.contains("Component") {
            return false;
        }
        true
    }

    pub fn is_faction_or_baro(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if self
            .unique_name
            .contains("CephalonSuda/Pistols/CSDroidArray")
        {
            return false;
        }
        if !self.unique_name.contains("/Prisma")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/SteelMeridian")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/ArbitersOfHexis")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/CephalonSuda")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/PerrinSequence")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/RedVeil")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Syndicates/NewLoka")
            && !self.unique_name.contains("/VoidTrader")
            && !self
                .unique_name
                .contains("/Lotus/Weapons/Corpus/LongGuns/CrpBFG/Vandal/VandalCrpBFG")
        {
            return self
                .unique_name
                .contains("/Lotus/Weapons/Tenno/Pistols/ConclaveLeverPistol/ConclaveLeverPistol");
        }
        true
    }

    pub fn is_weapon_part(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if self.unique_name.contains("/Weapons/Ostron")
            || self.unique_name.contains("/Weapons/Corpus")
        {
            return false;
        }
        if self.unique_name.contains("TeamAmmoBlueprint") {
            return false;
        }
        if self.unique_name.contains("/Weapons/Skins/") {
            return false;
        }
        if self.unique_name.contains("/WeaponParts/") {
            return true;
        }
        if self.unique_name.contains("/SentinelRecipes/") && self.unique_name.contains("Blueprint")
        {
            return true;
        }
        if self.unique_name.contains("/Weapons/") && self.unique_name.contains("Blueprint") {
            return true;
        }
        false
    }

    pub fn is_relic(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name.contains("/Projections/")
    }

    pub fn is_misc(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if self.unique_name.contains("/Fusers/LegendaryModFuser") {
            return true;
        }
        if self.unique_name == "/Lotus/Upgrades/Skins/Kubrows/Collars/PrimeKubrowCollarA" {
            return true;
        }
        if self.unique_name.contains("/Resources/Mechs/") {
            return true;
        }
        if self.unique_name.contains("/Items/Fish/") && !self.unique_name.contains("Boot") {
            return true;
        }
        if self.unique_name.contains("/Lotus/Upgrades/Skins/")
            && self.unique_name.contains("Helmet")
        {
            return true;
        }
        if self.unique_name == "/Lotus/Types/Keys/Nightwave/GlassmakerBossFightKey" {
            return true;
        }
        if self
            .unique_name
            .starts_with("/Lotus/Types/Items/FusionTreasures")
        {
            return true;
        }
        if self.is_scene() {
            return true;
        }
        false
    }

    pub fn is_scene(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name
            .starts_with("/Lotus/Types/Items/MiscItems/PhotoboothTile")
    }

    pub fn is_landing_craft_part(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name
            .contains("/Lotus/Types/Recipes/LandingCraftRecipes/")
    }

    pub fn is_arcane(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if self.unique_name.contains("/CosmeticEnhancers/Peculiars/") {
            return false;
        }
        self.unique_name
            .contains("/Lotus/Upgrades/CosmeticEnhancers")
    }

    pub fn is_arch_frame_part(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name
            .contains("/Lotus/Types/Recipes/ArchwingRecipes/")
    }

    pub fn is_arch_weapon_part(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name.contains("/Archwing/Primary/")
    }

    /**
     * AlecaFrame's `IsMod`: unlike `is_mod()` above it only ever runs on already-mod-family
     * paths (after Beginner exclusion, Peculiars/Railjack are mods, Arcanes are not).
     */
    pub fn is_upgrade_mod(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        if self.unique_name.contains("/Beginner/") {
            return false;
        }
        if self.unique_name.contains("/CosmeticEnhancers/Peculiars/") {
            return true;
        }
        if self.unique_name.contains("/Lotus/Upgrades/Mods/Railjack/") {
            return true;
        }
        if self.is_arcane() {
            return false;
        }
        true
    }

    /** AlecaFrame's `IsRivenMod`: path-only check; for fingerprint-aware detection use `is_riven()`. */
    pub fn is_riven_mod(&self) -> bool {
        if self.unique_name.is_empty() {
            return false;
        }
        self.unique_name
            .starts_with("/Lotus/Upgrades/Mods/Randomized/")
    }
}
