use entity::sub_type::SubType;
use serde::{de, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    cache::types::{cache_item_base::CacheItemBase, cache_tradable_item::CacheTradableItem},
    commands::item,
    log_parser::enums::trade_item_type::TradeItemType,
    utils::modules::{
        error::AppError,
        states,
        trading_helper::{
            combine_and_detect_match, combine_and_detect_multiple_matches, contains_unicode,
        },
    },
};

use super::{
    create_stock_entity::CreateStockEntity,
    trade_detection::{DetectionStatus, TradeDetection},
};

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
}
impl TradeItem {
    pub fn from_string(
        line: &str,
        next_line: &str,
        detection: &TradeDetection,
    ) -> (DetectionStatus, TradeItem) {
        let mut line = line.to_owned();
        let mut next_line = next_line.to_owned();
        let mut raw = line.to_owned();

        // Check if the item is platinum
        let (is_platinum_combined, is_platinum_status) =
            detection.is_platinum(&line, &next_line, false, false);
        if is_platinum_status.is_combined() {
            line = is_platinum_combined.clone();
            raw = line.clone();
            next_line = "".to_string();
        }

        // Check if the item is the last item
        let (mut last_item_combined, last_item_status) =
            detection.is_last_item(&line, &next_line, false, false);

        if last_item_status == DetectionStatus::NextLine {
            next_line.truncate(next_line.find(", leftItem=/").unwrap());
        } else if last_item_status == DetectionStatus::Line {
            line.truncate(line.find(", leftItem=/").unwrap());
            raw = line.clone();
        } else if last_item_status.is_combined() {
            last_item_combined.truncate(last_item_combined.find(", leftItem=/").unwrap());
            line = last_item_combined.clone();
            raw = line.clone();
        }

        let status = if last_item_status.is_combined() || is_platinum_status.is_combined() {
            DetectionStatus::CombinedWithSpace
        } else if last_item_status.is_found() || is_platinum_status.is_found() {
            DetectionStatus::Line
        } else {
            DetectionStatus::None
        };

        // Get the quantity of the item
        let mut quantity = 1;
        if raw.contains(" x ") {
            let cloned = raw.clone();
            let parts: Vec<&str> = cloned.split(" x ").collect();
            raw = parts[0].to_string();
            quantity = parts[1].parse().unwrap_or(1);
        }
        raw = raw.trim().to_string();

        let mut item = TradeItem {
            raw,
            quantity,
            unique_name: "".to_string(),
            item_type: if is_platinum_status.is_found() {
                TradeItemType::Platinum
            } else {
                TradeItemType::Unknown
            },
            sub_type: None,
            error: None,
        };
        if item.item_type == TradeItemType::Platinum {
            return (status, item);
        }
        // Validate the item
        let status = item.validate(&next_line).unwrap_or(DetectionStatus::None);
        if !status.is_found() {
            item.error = Some(("Item not found".to_string(), Value::Null));
        }

        (status, item)
    }

    fn tags_to_type(&self, tags: Vec<&str>) -> TradeItemType {
        if tags.contains(&"relic") {
            return TradeItemType::Relic;
        }
        if !tags.contains(&"component") && tags.contains(&"blueprint") {
            return TradeItemType::MainBlueprint;
        }
        if tags.contains(&"component") {
            return TradeItemType::Component;
        }
        if tags.contains(&"lens") {
            return TradeItemType::Lens;
        }
        if tags.contains(&"arcane_enhancement") {
            return TradeItemType::Arcane;
        }
        if tags.contains(&"mod") {
            return TradeItemType::Mod;
        }
        TradeItemType::Unknown
    }

    fn contains_at_least(
        &self,
        haystack: &str,
        needles: &str,
        count: usize,
        is_exact_match: bool,
    ) -> bool {
        let found = haystack.chars().filter(|&c| needles.contains(c)).count();

        if is_exact_match {
            found == count // Requires exact match
        } else {
            found >= count // Allows "at least" match
        }
    }
    // Helper function to extract logic used by both matching paths
    fn apply_item_info(&mut self, found: &CacheTradableItem) {
        let tags: Vec<&str> = found.tags.iter().map(|s| s.as_str()).collect();
        self.unique_name = found.unique_name.clone();
        self.item_type = self.tags_to_type(tags.clone());

        if tags.contains(&"relic") {
            self.sub_type = Some(SubType::variant("intact"));
        }
        if tags.contains(&"arcane_enhancement") {
            if let Some(max_rank) = found.sub_type.as_ref().and_then(|st| st.max_rank) {
                self.sub_type = Some(SubType::rank(max_rank));
            }
        }
    }
    // Find the item in the cache
    pub fn is_trade_item(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<DetectionStatus, AppError> {
        let ch = states::cache().expect("Cache not found");
        let machs = ch.tradable_items().get_item_dict("--item_by name")?;

        if machs.iter().any(|mach| line == mach.0) {
            let found = machs.get(line).unwrap();
            self.apply_item_info(found);
            return Ok(DetectionStatus::Line);
        }

        for mach in machs.iter() {
            let (combine, status) = combine_and_detect_match(line, next_line, mach.0, false, true);
            if status.is_found() {
                let found = machs.get(combine.as_str()).unwrap();
                self.apply_item_info(found);
                return Ok(status);
            }
        }
        Ok(DetectionStatus::None)
    }

    pub fn has_rank(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<(String, DetectionStatus), AppError> {
        let start = self.contains_at_least(&line, "(", 1, true);
        let end = self.contains_at_least(&line, ")", 1, true);
        if start && end {
            return Ok((line.to_string(), DetectionStatus::Line));
        }
        let (mut combine, status) =
            combine_and_detect_multiple_matches(&line, next_line, &["(", ")"], false, false);

        if status.is_found() {
            if !combine.contains(" RANK") {
                combine = combine.replace("RANK", " RANK")
            }
            let start = self.contains_at_least(&combine, "(", 1, true);
            let end = self.contains_at_least(&combine, ")", 1, true);
            if start && end {
                return Ok((combine, status));
            }
        }
        Ok((line.to_string(), DetectionStatus::None))
    }
    pub fn has_type(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<(String, DetectionStatus), AppError> {
        let start = self.contains_at_least(&line, "[", 1, true);
        let end = self.contains_at_least(&line, "]", 1, true);
        if start && end {
            return Ok((line.to_string(), DetectionStatus::Line));
        }
        let (combine, status) =
            combine_and_detect_multiple_matches(&line, next_line, &["[", "]"], false, false);

        if status.is_found() {
            let start = self.contains_at_least(&combine, "[", 1, true);
            let end = self.contains_at_least(&combine, "]", 1, true);
            if start && end {
                return Ok((combine, status));
            }
        }
        Ok((line.to_string(), DetectionStatus::None))
    }

    pub fn is_variant_item(&mut self, next_line: &str) -> Result<DetectionStatus, AppError> {
        let line = self.raw.clone();
        // Check if the item is a variant item Rank
        let (combine, status) = self.has_rank(&line, next_line)?;
        if status.is_found() {
            let index = combine.find("(").unwrap() as usize;
            let rank_part = &combine[index..];
            let name_part = &combine[..index - 1];
            // Check if the item is a mod/fish true if mod else it is a fish
            if rank_part.len() > 3 {
                // Set the item rank.
                let rank_part = rank_part.replace("(", "").replace(")", "");
                // Get The Rank of the mod
                for s in rank_part.split(' ') {
                    if let Ok(result) = s.parse::<i64>() {
                        self.sub_type = Some(SubType::rank(result));
                        break;
                    }
                }
                if combine.contains("(RIVEN RANK ") {
                    // Check if the item is a veiled riven
                    if combine.contains(" Riven Mod") {
                        if self
                            .is_trade_item(&format!("{} (Veiled)", name_part), next_line)?
                            .is_found()
                        {
                            self.item_type = TradeItemType::RivenVeiled;
                            return Ok(status);
                        }
                    } else {
                        let last_space_index = name_part.rfind(" ").unwrap() as usize;
                        let weapon = &name_part[..last_space_index];
                        let att = &name_part[last_space_index + 1..];
                        self.item_type = TradeItemType::RivenUnVeiled;
                        self.unique_name = format!("/WF_Special/Other/Riven/{}/{}", weapon, att);
                    }
                } else {
                    if self.is_trade_item(name_part, next_line)?.is_found() {
                        return Ok(status);
                    }
                }
            } else {
                let size = rank_part.replace("(", "").replace(")", "");
                if size.len() == 1 {
                    if let Some(c) = size.chars().next() {
                        self.sub_type = Some(SubType::rank(c as i64));
                    }
                }
                if self.is_trade_item(name_part, next_line)?.is_found() {
                    return Ok(status);
                }
            }
            return Ok(status);
        }
        // Check if the item is a variant item Relic
        let (combine, status) = self.has_type(&line, next_line)?;
        if status.is_found() {
            let index = combine.find("[").unwrap() as usize;
            let type_part = &combine[index..];
            let name_part = &combine[..index - 1];
            println!("Type Part: {}", type_part);
            if self.is_trade_item(name_part, next_line)?.is_found() {
                let type_part = type_part.replace("[", "").replace("]", "").to_lowercase();
                self.sub_type = Some(SubType::variant(&type_part));
                return Ok(status);
            }
            return Ok(status);
        }
        Ok(DetectionStatus::None)
    }

    fn has_arcane(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<(String, DetectionStatus), AppError> {
        if line.len() != line.chars().count() {
            return Ok((line.to_string(), DetectionStatus::Line));
        }
        let (combine, status) = contains_unicode(&line, next_line, false);
        return Ok((combine, status));
    }

    pub fn is_arcane(&mut self, next_line: &str) -> Result<DetectionStatus, AppError> {
        let line = self.raw.clone();
        // Check if the item is a variant item
        let (combine, status) = self.has_arcane(&line, next_line)?;
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

    pub fn validate(&mut self, next_line: &str) -> Result<DetectionStatus, AppError> {
        // Is Trade Item
        let status = self.is_trade_item(self.raw.clone().as_str(), next_line)?;
        if status.is_found() {
            return Ok(status);
        }

        // Is Variant Item
        let status = self.is_variant_item(next_line)?;
        if status.is_found() {
            return Ok(status);
        }
        // Is Arcane
        let status = self.is_arcane(next_line)?;
        if status.is_found() {
            return Ok(status);
        }
        return Ok(DetectionStatus::None);
    }

    pub fn is_valid(&self) -> bool {
        self.raw.len() > 0
    }

    pub fn display(&self) -> String {
        format!(
            "Name: {} | Quantity: {}, Unique Name: {}",
            self.raw, self.quantity, self.unique_name
        )
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
        }
    }
}
