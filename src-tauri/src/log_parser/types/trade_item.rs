use std::fmt::Display;

use entity::dto::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::*;

use crate::cache::{
    CacheFish, CacheItemBase, CacheMisc, CacheMod, CacheRecipe, CacheRelics, CacheResource,
};
use crate::enums::TradeItemType;
use crate::{cache::types::CacheTradableItem, log_parser::log, utils::modules::states};

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
        let match_text = ", leftItem=/";
        // let match_text = ", title= ";
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
        let status = item.validate(&next_line).unwrap_or(DetectionStatus::None);
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

    // Helper function to extract logic used by both matching paths
    fn apply_item_info(&mut self, found: &CacheTradableItem) {
        let tags: Vec<&str> = found.tags.iter().map(|s| s.as_str()).collect();

<<<<<<< HEAD
        let variants = found
            .sub_type
            .clone()
            .unwrap_or_default()
            .variants
            .unwrap_or_default();

        if tags.contains(&"relic") {
            self.sub_type = Some(SubType::variant("intact"));
        }
=======
>>>>>>> 8b8a01cc (Refactor `TradeItem` detection for improved accuracy and modularity)
        if tags.contains(&"arcane_enhancement") {
            if let Some(max_rank) = found.sub_type.as_ref().and_then(|st| st.max_rank) {
                self.sub_type = Some(SubType::rank(max_rank));
            }
        }
<<<<<<< HEAD
        // Some mods have variants that are not indicated in the name
        if variants.contains(&"regular".to_string()) && variants.contains(&"atragraph".to_string())
        {
            self.sub_type = Some(SubType::variant("regular"));
        }
=======
        self.unique_name = found.unique_name.clone();
        self.item_type = tags_to_type(tags.clone());
>>>>>>> 8b8a01cc (Refactor `TradeItem` detection for improved accuracy and modularity)
    }

    pub fn is_variant_item(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<DetectionStatus, Error> {
        // Check if the item is a mod eg. "Serration (RIVEN RANK 0)"
        if let Some((combine, status)) = detect_enclosed_text(line, next_line, "(", ")") {
            let (name_part, rank_str) = split_base_name_and_enclosed_value(&combine, '(', ')');
            // Handle the rank or size of the fish.
            match rank_str.as_str() {
                "S" => {
                    self.sub_type = Some(SubType::variant("small"));
                }
                "M" => {
                    self.sub_type = Some(SubType::variant("medium"));
                }
                "L" => {
                    self.sub_type = Some(SubType::variant("large"));
                }
                _ => {
                    for s in rank_str.split(' ') {
                        if let Ok(result) = s.parse::<i64>() {
                            self.sub_type = Some(SubType::rank(result));
                            break;
                        }
                    }
                }
            }
            if combine.starts_with("Legendary Core") {
                self.is_trade_item("Legendary Fusion Core", next_line)?;
                self.item_type = TradeItemType::FusionCore;
                self.sub_type = None; // Legendary Fusion Core is a special case
            } else if combine.contains("(RIVEN RANK ") {
                if combine.contains(" Riven Mod")
                    || combine.contains(" RIVEN MOD")
                        && self
                            .is_trade_item(&format!("{name_part} (Veiled)"), next_line)?
                            .is_found()
                {
                    self.item_type = TradeItemType::RivenVeiled;
                } else if let Some(pos) = name_part.rfind(' ') {
                    let ch = states::cache_client().expect("Cache not found");
                    let (weapon, att) = name_part.split_at(pos);

                    match ch.weapon().get_by(format!("Name:{}", weapon.trim())) {
                        Ok(info) => {
                            self.raw = info.wfm_url.clone();
                            self.item_type = TradeItemType::RivenUnVeiled;
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            log(&msg, None);
                            self.error = Some((msg.clone(), Value::Null));
                            return Ok(DetectionStatus::None);
                        }
                    }
                    self.unique_name = att.trim().to_string();
                }
            } else {
                self.is_trade_item(&name_part, next_line)?;
            }

            return Ok(status);
        }

        if let Some((combine, status)) = detect_enclosed_text(line, next_line, "[", "]") {
            let (name_part, type_str) = split_base_name_and_enclosed_value(&combine, '[', ']');
            self.sub_type = Some(SubType::variant(&type_str.to_lowercase()));
            if self.is_trade_item(&name_part, next_line)?.is_found() {
                return Ok(status);
            }
        }

        Ok(DetectionStatus::None)
    }
    pub fn is_arcane(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        let (combine, status) = contains_unicode(&line, next_line, false);

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }
        let index = combine.rfind(' ').unwrap_or(0);
        let name_part = &combine[..index];
        if self.is_trade_item(name_part, next_line)?.is_found() {
            return Ok(status);
        }
        return Ok(DetectionStatus::None);
    }
    pub fn is_imprint(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        // Imprint of |NAME|
        let imprint_open = "imprint of ";
        let (stripped, status) = strip_prefix(
            imprint_open,
            &line.to_lowercase(),
            &next_line.to_lowercase(),
            false,
        );

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        self.item_type = TradeItemType::Imprint;
        self.unique_name = String::from("/WF_Special/CreaturePet/Imprint");
        self.sub_type = Some(SubType::variant(&stripped));
        return Ok(status);
    }
    pub fn is_relic(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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
    pub fn is_misc(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");

        let misc = cache.misc();

        let apply_misc = |this: &mut Self, info: &CacheMisc| {
            this.unique_name = info.base.unique_name.clone();
            this.sub_type = info.base.sub_type.clone();
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
    pub fn is_recipe(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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
    pub fn is_mod(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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

        if let Ok(info) = mods.get_by(format!("Name:{}", name)) {
            apply_mod(self, &info);
            return Ok(DetectionStatus::Line);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_riven_mod(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        
        let (status, name, rank_str) = extract_item_variant(line, next_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }
        let weapons = cache.weapons();   

        
        let pos = name.rfind(' ').unwrap_or(-1);
        if pos == -1 {
            return Ok(DetectionStatus::None);
        }

        let (weapon, att) = name.split_at(pos);

        let apply_weapon = |this: &mut Self, info: &CacheWeapon| {
            this.unique_name = info.base.unique_name.clone();
            this.item_type = TradeItemType::RivenVeiled;
        };

        if let Ok(info) = weapons.get_by(weapon.trim()) {
            apply_weapon(self, &info);
            return Ok(DetectionStatus::Line);
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_fish(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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
            let combined_name = format!("Name:{}|{}", name, var);
            if let Ok(info) = fish.get_by(combined_name) {
                apply_fish(self, &info);
                return Ok(DetectionStatus::Line);
            }
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_bundle(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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
    pub fn is_gear(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
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
    pub fn is_skin(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let skins = cache.skins();       

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
    pub fn is_weapon(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let weapons = cache.weapons();       

        let apply_weapon = |this: &mut Self, info: &CacheWeapon| {
            this.unique_name = info.base.unique_name.clone();
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
   
    pub fn validate(&mut self, next_line: &str) -> Result<DetectionStatus, Error> {
        for check in [
            Self::is_relic,
            Self::is_recipe,
            Self::is_arcane,
            Self::is_misc,
            Self::is_fish,
            Self::is_riven_mod,
            Self::is_mod,
            Self::is_bundle,
            Self::is_gear,
            Self::is_skin,
            // Self::is_raw_item,
            // Self::is_trade_item,
            // Self::is_variant_item,
            // Self::is_weapon,
            // Self::is_imprint,
        ] {
            let status = check(self, &self.raw.clone(), next_line)?;
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

pub fn tags_to_type(tags: Vec<&str>) -> TradeItemType {
    match () {
        _ if tags.contains(&"ayatan_sculpture") => TradeItemType::Ayatan,
        _ if tags.contains(&"weapon") => TradeItemType::Weapon,
        _ if tags.contains(&"relic") => TradeItemType::Relic,
        _ if tags.contains(&"component") => TradeItemType::Component,
        _ if tags.contains(&"lens") => TradeItemType::Lens,
        _ if tags.contains(&"arcane_enhancement") => TradeItemType::Arcane,
        _ if tags.contains(&"mod") => TradeItemType::Mod,
        _ if tags.contains(&"fish") => TradeItemType::Fish,
        _ => TradeItemType::Unknown,
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
