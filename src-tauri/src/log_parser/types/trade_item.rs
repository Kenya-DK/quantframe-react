use std::fmt::Display;

use entity::dto::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::*;

use crate::cache::*;
use crate::enums::TradeItemType;
use crate::{cache::types::CacheTradableItem, utils::modules::states};

use super::trade_detection::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeItem {
    #[serde(rename = "raw")]
    pub raw: String,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "unique_name")]
    #[serde(default)]
    pub unique_name: String,

    #[serde(rename = "sub_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "item_type")]
    pub item_type: TradeItemType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<(String, Value)>,

    // Extra properties
    #[serde(flatten)]
    pub properties: Properties,
}
impl TradeItem {
    pub fn new(
        unique_name: impl Into<String>,
        quantity: i64,
        item_type: TradeItemType,
        sub_type: Option<SubType>,
    ) -> TradeItem {
        let unique_name = unique_name.into();
        TradeItem {
            raw: unique_name.clone(),
            quantity,
            unique_name,
            item_type,
            sub_type,
            error: None,
            properties: Properties::default(),
        }
    }
    pub fn from_string(
        line: &str,
        next_line: &str,
        detection: &TradeDetection,
    ) -> (DetectionStatus, TradeItem) {
        let mut line = line.to_owned();
        let mut next_line = next_line.to_owned();
        let mut raw = line.to_owned();
        // let match_text = ", leftItem=/";
        let match_text = ", title= ";
        // Check if the item is platinum
        let (is_currency_combined, is_currency_status, is_currency_type) =
            detection.is_currency(&line, &next_line, false, false);

        if is_currency_status.is_combined() {
            line = is_currency_combined.clone();
            raw = line.clone();
            next_line = "".to_string();
        }

        // Check if the item is the last item
        let (mut last_item_combined, last_item_status) =
            detection.is_last_item(&line, &next_line, false, false);
        if last_item_status == DetectionStatus::NextLine {
            next_line.truncate(next_line.find(match_text).unwrap());
        } else if last_item_status == DetectionStatus::Line {
            line.truncate(line.find(match_text).unwrap());
            raw = line.clone();
        } else if last_item_status.is_combined() {
            last_item_combined.truncate(last_item_combined.find(match_text).unwrap());
            line = last_item_combined.clone();
            raw = line.clone();
        }

        let status = if last_item_status.is_combined() || is_currency_status.is_combined() {
            DetectionStatus::Combined
        } else if last_item_status.is_found() || is_currency_status.is_found() {
            DetectionStatus::Line
        } else {
            DetectionStatus::None
        };

        // Get the quantity of the item
        let (raw, quantity) = parse_quantity(&raw);

        let mut item = TradeItem {
            raw,
            quantity,
            unique_name: "".to_string(),
            item_type: is_currency_type,
            sub_type: None,
            error: None,
            properties: Properties::default(),
        };
        if matches!(
            item.item_type,
            TradeItemType::Platinum | TradeItemType::Credits
        ) {
            return (status, item);
        }
        // Validate the item
        let status = item
            .validate(&next_line, detection)
            .unwrap_or(DetectionStatus::None);
        if !status.is_found() {
            item.error = Some(("Item not found".to_string(), Value::Null));
        }
        if item.item_type == TradeItemType::Mod && item.sub_type.is_none() {
            item.error = Some(("Mod Rank not found".to_string(), Value::Null));
            item.unique_name = "".to_string();
            item.item_type = TradeItemType::Unknown;
        }
        (status, item)
    }

    /* -------------------------------------------------------------
        Shared Helpers
    ------------------------------------------------------------- */

    /* -------------------------------------------------------------
        Detection's for specific item types
    ------------------------------------------------------------- */

    pub fn is_arcane(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let arcane = cache.arcane();

        let (combine, status) = contains_unicode(&line, next_line, false);

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        let index = combine.rfind(' ').unwrap_or(0);
        let name = &combine[..index];

        let apply_arcane = |this: &mut Self, info: &CacheArcane| {
            this.unique_name = info.base.unique_name.clone();
            // Default to Max Rank since we cant determine the rank from the logs...
            this.sub_type = Some(SubType::rank(info.fusion_limit as i64));
            this.item_type = TradeItemType::Arcane;
        };

        if let Ok(info) = arcane.get_by(name) {
            apply_arcane(self, &info);
            return Ok(DetectionStatus::Line);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_imprint(
        &mut self,
        line: &str,
        next_line: &str,
        detection: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let pets = cache.pet();

        // Imprint of |NAME|
        let (stripped, status) = strip_prefix(
            detection.imprint_name.as_str(),
            &line.to_lowercase(),
            &next_line.to_lowercase(),
            false,
        );

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        if let Ok(info) = pets.get_by("/WFSpecial/CreaturePet/Imprint") {
            self.unique_name = info.base.unique_name.clone();
            self.sub_type = Some(SubType::variant(&stripped));
            self.item_type = TradeItemType::Imprint;
            self.properties
                .set_property_value("name", info.base.name.clone());
            self.properties.set_property_value(
                "wfmUrl",
                info.base.wfm_url.clone().unwrap_or("".to_string()),
            );
            self.properties
                .set_property_value("tags", info.base.tags.clone());
            return Ok(status);
        }
        return Ok(status);
    }
    pub fn is_relic(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let relics = cache.relics();

        let apply_relic = |this: &mut Self, info: &CacheRelics| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Relic;
        };

        if let Ok(info) = relics.get_by(line) {
            apply_relic(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = relics.get_by(&combined_line) {
            apply_relic(self, &info);
            return Ok(DetectionStatus::Combined);
        }

        let (status, name, tier) = extract_item_variant(line, next_line, '[', ']')?;
        if status.is_found() {
            let relic_name = format!("{} {}", name, tier.to_lowercase());
            if let Ok(info) = relics.get_by(&relic_name) {
                apply_relic(self, &info);
                return Ok(status);
            }
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_misc(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let misc = cache.misc();

        let apply_misc = |this: &mut Self, info: &CacheMisc| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            if info.base.unique_name.contains("FusionTreasures") {
                this.properties.set_property_value("requireSubType", true);
                this.sub_type = Some(SubType::ayatan(0, 0));
            }
            this.item_type = TradeItemType::Misc;
        };

        if let Ok(info) = misc.get_by(line) {
            apply_misc(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = misc.get_by(&combined_line) {
            apply_misc(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_recipe(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let recipes = cache.recipe();

        let apply_recipe = |this: &mut Self, info: &CacheRecipe| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Recipe;
        };

        if let Ok(info) = recipes.get_by(line) {
            apply_recipe(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = recipes.get_by(&combined_line) {
            apply_recipe(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_mod(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let (status, name, rank_str) = extract_item_variant(line, next_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        let mods = cache.mods();

        let mut rank = 0;
        for s in rank_str.split(' ') {
            if let Ok(result) = s.parse::<i64>() {
                rank = result;
                break;
            }
        }

        let apply_mod = |this: &mut Self, info: &CacheMod| {
            let variant = info
                .base
                .sub_type
                .as_ref()
                .and_then(|sub_type| sub_type.variant.as_deref())
                .unwrap_or("");
            this.unique_name = info.base.unique_name.clone();
            this.item_type = TradeItemType::ModWithNoRank;
            if info.fusion_limit > 0 && variant.is_empty() {
                this.sub_type = Some(SubType::rank(rank));
                this.item_type = TradeItemType::Mod;
            } else {
                this.sub_type = info.base.sub_type.clone();
                this.properties.set_property_value("requireSubType", true);
                this.item_type = TradeItemType::RivenUnVeiled;
            }
        };

        if let Ok(info) = mods.get_by(name) {
            apply_mod(self, &info);
            return Ok(DetectionStatus::Line);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_riven_mod(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let (status, name, _) = extract_item_variant(line, next_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }
        let weapons = cache.weapon();

        let pos = name.rfind(' ').unwrap_or(1);
        if pos == 0 {
            return Ok(DetectionStatus::None);
        }

        let (weapon, _att) = name.split_at(pos);

        let apply_weapon = |this: &mut Self, info: &CacheWeaponBase| {
            this.unique_name = info.unique_name.clone();
            this.item_type = TradeItemType::RivenUnVeiled;
        };

        if let Ok(info) = weapons.get_by(weapon.trim_end()) {
            apply_weapon(self, &info);
            return Ok(DetectionStatus::Line);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_fish(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let fish = cache.fish();

        let (status, name, size) = extract_item_variant(line, next_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }
        // Check if size is one Char long
        if size.len() != 1 {
            return Ok(DetectionStatus::None);
        }
        let variant = match size.as_str() {
            "S" => "small",
            "M" => "medium|magnificent",
            "L" => "large",
            "B" => "basic",
            "A" => "adorned",
            _ => {
                return Ok(DetectionStatus::None);
            }
        };

        let variants = variant.split('|').collect::<Vec<&str>>();

        let apply_fish = |this: &mut Self, info: &CacheFish| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Fish;
        };

        for var in variants {
            let combined_name = format!("{}|{}", name, var);
            if let Ok(info) = fish.get_by(combined_name) {
                apply_fish(self, &info);
                return Ok(DetectionStatus::Line);
            }
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_bundle(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let bundle = cache.bundle();

        let apply_bundle = |this: &mut Self, info: &CacheBundle| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Bundle;
        };

        if let Ok(info) = bundle.get_by(line) {
            apply_bundle(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = bundle.get_by(&combined_line) {
            apply_bundle(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_weapon(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let weapons = cache.weapon();

        let apply_weapon = |this: &mut Self, info: &CacheWeaponBase| {
            this.unique_name = info.unique_name.clone();
            this.item_type = TradeItemType::Weapon;
        };

        if let Ok(info) = weapons.get_by(line) {
            apply_weapon(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = weapons.get_by(&combined_line) {
            apply_weapon(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_quest(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let quests = cache.quest();

        let apply_quest = |this: &mut Self, info: &CacheQuest| {
            this.unique_name = info.base.unique_name.clone();
            this.item_type = TradeItemType::Quest;
        };

        if let Ok(info) = quests.get_by(line) {
            apply_quest(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = quests.get_by(&combined_line) {
            apply_quest(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_resource(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let resources = cache.resource();

        let apply_resource = |this: &mut Self, info: &CacheResource| {
            this.unique_name = info.base.unique_name.clone();
            this.item_type = TradeItemType::Resource;
        };

        if let Ok(info) = resources.get_by(line) {
            apply_resource(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = resources.get_by(&combined_line) {
            apply_resource(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_gear(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let gear = cache.gear();

        let apply_gear = |this: &mut Self, info: &CacheGear| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Gear;
        };

        if let Ok(info) = gear.get_by(line) {
            apply_gear(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = gear.get_by(&combined_line) {
            apply_gear(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_skin(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let skins = cache.skin();

        let apply_skin = |this: &mut Self, info: &CacheSkin| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Skin;
        };

        if let Ok(info) = skins.get_by(line) {
            apply_skin(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = skins.get_by(&combined_line) {
            apply_skin(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_sentinel(
        &mut self,
        line: &str,
        next_line: &str,
        _: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let sentinels = cache.sentinel();

        let apply_sentinel = |this: &mut Self, info: &CacheSentinel| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
            this.item_type = TradeItemType::Sentinel;
        };

        if let Ok(info) = sentinels.get_by(line) {
            apply_sentinel(self, &info);
            return Ok(DetectionStatus::Line);
        }

        let combined_line = format!("{}{}", line, next_line);

        if let Ok(info) = sentinels.get_by(&combined_line) {
            apply_sentinel(self, &info);
            return Ok(DetectionStatus::Combined);
        }
        Ok(DetectionStatus::None)
    }
    pub fn validate(
        &mut self,
        next_line: &str,
        detection: &TradeDetection,
    ) -> Result<DetectionStatus, Error> {
        for check in [
            Self::is_relic,
            Self::is_recipe,
            Self::is_arcane,
            Self::is_misc,
            Self::is_fish,
            Self::is_riven_mod,
            Self::is_mod,
            Self::is_bundle,
            Self::is_weapon,
            Self::is_quest,
            Self::is_skin,
            Self::is_resource,
            Self::is_imprint,
            Self::is_gear,
            Self::is_sentinel,
        ] {
            let status = check(self, &self.raw.clone(), next_line, detection)?;
            if status.is_found() {
                return Ok(status);
            }
        }
        Ok(DetectionStatus::None)
    }
    pub fn get_trade_item_info(&self) -> Result<CacheTradableItem, Error> {
        let ch = states::cache_client().expect("Cache not found");
        let info = ch.tradable_item().get_by(&self.unique_name)?;
        Ok(info)
    }
    pub fn item_name(&self) -> String {
        match self.get_trade_item_info() {
            Ok(info) => info.name.clone(),
            Err(_) => self.raw.clone(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.raw.is_empty()
    }
}

impl Default for TradeItem {
    fn default() -> Self {
        TradeItem {
            raw: "N/A".to_string(),
            quantity: 1,
            unique_name: "".to_string(),
            item_type: TradeItemType::Unknown,
            sub_type: None,
            error: None,
            properties: Properties::default(),
        }
    }
}

impl Display for TradeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TradeItem ")?;
        if self.raw.is_empty() {
            write!(f, "Raw: Not provided | ")?;
        } else {
            write!(f, "Raw: {} | ", self.raw)?;
        }
        write!(f, "Quantity: {} | ", self.quantity)?;
        if self.unique_name.is_empty() {
            write!(f, "Unique Name: Not provided | ")?;
        } else {
            write!(f, "Unique Name: {} | ", self.unique_name)?;
        }
        if let Some(sub_type) = &self.sub_type {
            write!(f, "Sub Type: {} | ", sub_type.display())?;
        } else {
            write!(f, "Sub Type: Not provided | ")?;
        }
        if let Some((error, _)) = &self.error {
            write!(f, "Error: {}", error)?;
        } else {
            write!(f, "Error: None")?;
        }
        Ok(())
    }
}

pub fn parse_quantity(raw: &str) -> (String, i64) {
    if let Some((name, qty)) = raw.split_once(" x ") {
        let quantity = qty.trim().parse().unwrap_or(1);
        (name.to_string(), quantity)
    } else {
        (raw.to_string(), 1)
    }
}
