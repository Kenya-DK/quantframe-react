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
        line: impl Into<String>,
        prev_line: impl Into<String>,
        detection: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
    ) -> (DetectionStatus, TradeItem) {
        let mut line = line.into();
        let mut prev_line = prev_line.into();

        let (parse_text, last_item_status) =
            detection.is_last_item(&line, &prev_line, ignored_combinations);

        match last_item_status {
            DetectionStatus::Line => {
                line = parse_text;
            }
            ref s if *s == DetectionStatus::PreviousLine || s.is_combined() => {
                prev_line = parse_text;
            }
            _ => {}
        }

        let mut item = TradeItem::new(&line, 1, TradeItemType::Unknown, None);

        let item_status = item
            .validate(&prev_line, detection, ignored_combinations)
            .unwrap_or(DetectionStatus::None);

        if !item_status.is_found() {
            item.error = Some(("Item not found".to_string(), Value::Null));
        } else if matches!(item.item_type, TradeItemType::Mod) && item.sub_type.is_none() {
            item.error = Some(("Mod Rank not found".to_string(), Value::Null));
            item.unique_name.clear();
            item.item_type = TradeItemType::Unknown;
        }

        let status = match (last_item_status, item_status) {
            (a, b) if a == b => b,
            (a, b) if a.is_combined() || b.is_combined() => DetectionStatus::Combined,
            (_, b) => b,
        };

        (status, item)
    }

    /* -------------------------------------------------------------
        Detection's for specific item types
    ------------------------------------------------------------- */
    pub fn is_platinum_or_credits(
        &mut self,
        line: &str,
        prev_line: &str,
        detection: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        // Check if the item is platinum
        let (combine, status, item_type) =
            detection.is_currency(&line, &prev_line, ignored_combinations);

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        match item_type {
            TradeItemType::Platinum => {
                self.unique_name = "WFSpecial/Currency/Platinum".to_string();
            }
            TradeItemType::Credits => {
                self.unique_name = "WFSpecial/Currency/Credits".to_string();
            }
            _ => {}
        }

        if let Some((_, qty)) = combine.split_once(" x ") {
            self.quantity = qty.trim().parse().unwrap_or(1);
        }
        self.item_type = item_type;
        Ok(status)
    }
    pub fn is_arcane(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        let (combine, status) = contains_unicode(&line, prev_line, false);

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        let index = combine.rfind(' ').unwrap_or(0);
        let name = combine[..index].trim();
        lookup_item(
            name,
            "",
            |name| cache.arcane().get_by(name),
            ignored_combinations,
            |info: CacheArcane| {
                self.unique_name = info.base.unique_name.clone();
                // Default to Max Rank since we cant determine the rank from the logs...
                self.sub_type = Some(SubType::rank(info.fusion_limit as i64));
                self.item_type = TradeItemType::Arcane;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_imprint(
        &mut self,
        line: &str,
        prev_line: &str,
        detection: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        // Imprint of |NAME|
        let (stripped, status) = strip_prefix(
            detection.imprint_name.as_str(),
            &line.to_lowercase(),
            &prev_line.to_lowercase(),
            ignored_combinations,
        );

        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        if let Ok(info) = cache.pet().get_by("/WFSpecial/CreaturePet/Imprint") {
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
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        let mut line = line.to_string();
        let mut prev_line = prev_line.to_string();
        let relics = cache.relics();

        let (status, name, tier) = extract_item_variant(&line, &prev_line, '[', ']')?;
        if status.is_found() {
            line = format!("{} {}", name, tier.to_lowercase());
            prev_line = "".to_string();
        }

        lookup_item(
            &line,
            &prev_line,
            |name| relics.get_by(name),
            ignored_combinations,
            |info: CacheRelics| {
                self.unique_name = info.base.unique_name.clone();
                self.sub_type = info.base.sub_type.clone();
                self.item_type = TradeItemType::Relic;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_misc(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.misc().get_by(name),
            ignored_combinations,
            |info: CacheMisc| {
                self.unique_name = info.base.unique_name.clone();
                self.sub_type = info.base.sub_type.clone();
                if info.base.unique_name.contains("FusionTreasures") {
                    self.properties.set_property_value("requireSubType", true);
                    self.sub_type = Some(SubType::ayatan(0, 0));
                }
                self.item_type = TradeItemType::Misc;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_recipe(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.recipe().get_by(name),
            ignored_combinations,
            |info: CacheRecipe| {
                self.unique_name = info.base.unique_name.clone();
                self.sub_type = info.base.sub_type.clone();
                self.item_type = TradeItemType::Recipe;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_mod(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        let (status, name, rank_str) = extract_item_variant(line, prev_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        let rank = rank_str
            .split_whitespace()
            .find_map(|s| s.parse::<i64>().ok())
            .unwrap_or(0);

        lookup_item(
            name.as_str(),
            "",
            |name| cache.mods().get_by(name),
            ignored_combinations,
            |info: CacheMod| {
                let trade_item = cache.tradable_item().get_by(&info.base.unique_name).ok();

                let variant = info
                    .base
                    .sub_type
                    .as_ref()
                    .and_then(|s| s.variant.as_deref())
                    .unwrap_or("");

                let mut sub_type = SubType::default();
                self.unique_name = info.base.unique_name.clone();
                self.item_type = TradeItemType::ModWithNoRank;

                if let Some(trade_item) = trade_item {
                    if trade_item
                        .sub_type
                        .unwrap_or_default()
                        .has_variants(&["regular", "atragraph"])
                    {
                        sub_type.variant = Some("regular".to_string());
                    }
                }

                if info.fusion_limit > 0 && variant.is_empty() {
                    sub_type.rank = Some(rank);
                    self.item_type = TradeItemType::Mod;
                    self.sub_type = Some(sub_type);
                    return info.base.unique_name;
                }
                self.sub_type = info.base.sub_type.clone();
                self.properties.set_property_value("requireSubType", true);
                self.item_type = TradeItemType::RivenUnVeiled;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_riven_mod(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        let (status, name, _) = extract_item_variant(line, prev_line, '(', ')')?;
        if !status.is_found() {
            return Ok(DetectionStatus::None);
        }

        let pos = name.rfind(' ').unwrap_or(1);
        if pos == 0 {
            return Ok(DetectionStatus::None);
        }

        let (weapon, _att) = name.split_at(pos);

        lookup_item(
            weapon.trim_end(),
            "",
            |name| cache.weapon().get_by(name),
            ignored_combinations,
            |info: CacheWeaponBase| {
                self.unique_name = info.unique_name.clone();
                self.item_type = TradeItemType::RivenUnVeiled;
                info.unique_name
            },
            cache,
        )
    }
    pub fn is_fish(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        let (status, name, size) = extract_item_variant(line, prev_line, '(', ')')?;
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

        for var in variant.split('|') {
            let combined_name = format!("{}|{}", name, var);
            let found_status = lookup_item(
                combined_name.as_str(),
                "",
                |name| cache.fish().get_by(name),
                ignored_combinations,
                |info: CacheFish| {
                    self.unique_name = info.base.unique_name.clone();
                    self.sub_type = info.base.sub_type.clone();
                    self.item_type = TradeItemType::Fish;
                    info.base.unique_name
                },
                cache,
            )?;

            if found_status.is_found() {
                return Ok(status);
            }
        }

        Ok(DetectionStatus::None)
    }
    pub fn is_bundle(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.bundle().get_by(name),
            ignored_combinations,
            |info: CacheBundle| {
                self.sub_type = info.base.sub_type.clone();
                self.item_type = TradeItemType::Bundle;
                self.unique_name = info.base.unique_name.clone();
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_weapon(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.weapon().get_by(name),
            ignored_combinations,
            |info: CacheWeaponBase| {
                self.unique_name = info.unique_name.clone();
                self.item_type = TradeItemType::Weapon;
                info.unique_name
            },
            cache,
        )
    }
    pub fn is_quest(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.quest().get_by(name),
            ignored_combinations,
            |info: CacheQuest| {
                self.unique_name = info.base.unique_name.clone();
                self.item_type = TradeItemType::Quest;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_resource(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.resource().get_by(name),
            ignored_combinations,
            |info: CacheResource| {
                self.unique_name = info.base.unique_name.clone();
                self.item_type = TradeItemType::Resource;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_gear(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.gear().get_by(name),
            ignored_combinations,
            |info: CacheGear| {
                self.unique_name = info.base.unique_name.clone();
                self.item_type = TradeItemType::Gear;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_skin(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.skin().get_by(name),
            ignored_combinations,
            |info: CacheSkin| {
                self.unique_name = info.base.unique_name.clone();
                self.sub_type = info.base.sub_type.clone();
                self.item_type = TradeItemType::Skin;
                info.base.unique_name
            },
            cache,
        )
    }
    pub fn is_sentinel(
        &mut self,
        line: &str,
        prev_line: &str,
        _: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
        cache: &CacheState,
    ) -> Result<DetectionStatus, Error> {
        lookup_item(
            line,
            prev_line,
            |name| cache.sentinel().get_by(name),
            ignored_combinations,
            |info: CacheSentinel| {
                self.unique_name = info.base.unique_name.clone();
                self.sub_type = info.base.sub_type.clone();
                self.item_type = TradeItemType::Sentinel;
                info.base.unique_name
            },
            cache,
        )
    }

    /* -------------------------------------------------------------
        Validation and Info retrieval
    ------------------------------------------------------------- */
    pub fn validate(
        &mut self,
        prev_line: &str,
        detection: &TradeDetection,
        ignored_combinations: &[DetectionStatus],
    ) -> Result<DetectionStatus, Error> {
        let cache = states::cache_client().expect("Cache not found");
        for check in [
            Self::is_platinum_or_credits,
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
            let status = check(
                self,
                &self.raw.clone(),
                prev_line,
                detection,
                ignored_combinations,
                &cache,
            )?;
            if status.is_found() {
                return Ok(status);
            }
        }
        Ok(DetectionStatus::None)
    }

    /* -------------------------------------------------------------
        Trade Item Info Retrieval
    ------------------------------------------------------------- */
    pub fn get_trade_item_info(&self) -> Result<CacheTradableItem, Error> {
        let cache = states::cache_client().expect("Cache not found");
        let info = cache.tradable_item().get_by(&self.unique_name)?;
        Ok(info)
    }
    pub fn item_name(&self) -> String {
        match self.get_trade_item_info() {
            Ok(info) => info.name.clone(),
            Err(_) => self.raw.clone(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.item_type != TradeItemType::Unknown && self.error.is_none()
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
        let mut message: Vec<String> = vec![];
        if !self.raw.is_empty() {
            message.push(format!("Raw: '{}'", self.raw));
        }
        if !self.unique_name.is_empty() {
            message.push(format!("Unique Name: '{}'", self.unique_name));
        }
        message.push(format!("Quantity: {}", self.quantity));
        message.push(format!("Item Type: {:?}", self.item_type));

        if let Some(sub_type) = &self.sub_type {
            message.push(format!("Sub Type: {}", sub_type.display()));
        }
        if let Some((error, _)) = &self.error {
            message.push(format!("Error: {}", error));
        }
        write!(f, "{}", message.join(" | "))
    }
}

fn lookup_item<T>(
    line: &str,
    prev_line: &str,
    get_by: impl Fn(&str) -> Result<T, Error>,
    ignored_combinations: &[DetectionStatus],
    mut apply: impl FnMut(T) -> String,
    cache: &CacheState,
) -> Result<DetectionStatus, Error> {
    if let Ok(info) = get_by(line) {
        let trade_item = cache.tradable_item().get_by(apply(info)).ok();
        if trade_item.is_some() {
            return Ok(DetectionStatus::Line);
        }
    }

    if !is_ignored(ignored_combinations, DetectionStatus::PreviousLine) {
        if let Ok(info) = get_by(prev_line) {
            let trade_item = cache.tradable_item().get_by(apply(info)).ok();
            if trade_item.is_some() {
                return Ok(DetectionStatus::PreviousLine);
            }
        }
    }

    if !is_ignored(ignored_combinations, DetectionStatus::LineThenPreviousLine) {
        let line_then_previous = format!("{line}{prev_line}");
        if let Ok(info) = get_by(&line_then_previous) {
            let trade_item = cache.tradable_item().get_by(apply(info)).ok();
            if trade_item.is_some() {
                return Ok(DetectionStatus::LineThenPreviousLine);
            }
        }
    }

    if !is_ignored(ignored_combinations, DetectionStatus::PreviousLineThenLine) {
        let previous_then_line = format!("{prev_line}{line}");
        if let Ok(info) = get_by(&previous_then_line) {
            let trade_item = cache.tradable_item().get_by(apply(info)).ok();
            if trade_item.is_some() {
                return Ok(DetectionStatus::PreviousLineThenLine);
            }
        }
    }

    Ok(DetectionStatus::None)
}
