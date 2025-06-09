use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    cache::types::cache_tradable_item::CacheTradableItem,
    log_parser::enums::trade_item_type::TradeItemType,
    utils::modules::{
        error::AppError,
        states,
        trading_helper::{
            combine_and_detect_match, combine_and_detect_multiple_matches, contains_at_least,
            contains_unicode, parse_quantity, tags_to_type,
        },
    },
};

use super::trade_detection::{DetectionStatus, TradeDetection};

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
            DetectionStatus::Combined
        } else if last_item_status.is_found() || is_platinum_status.is_found() {
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
        if item.item_type == TradeItemType::Mod && item.sub_type.is_none() {
            item.error = Some(("Mod Rank not found".to_string(), Value::Null));
            item.unique_name = "".to_string();
            item.item_type = TradeItemType::Unknown;
        }
        (status, item)
    }

    // Helper function to extract logic used by both matching paths
    fn apply_item_info(&mut self, found: &CacheTradableItem) {
        let tags: Vec<&str> = found.tags.iter().map(|s| s.as_str()).collect();
        self.unique_name = found.unique_name.clone();
        self.item_type = tags_to_type(tags.clone());

        if tags.contains(&"relic") {
            self.sub_type = Some(SubType::variant("intact"));
        }
        if tags.contains(&"arcane_enhancement") {
            if let Some(max_rank) = found.sub_type.as_ref().and_then(|st| st.max_rank) {
                self.sub_type = Some(SubType::rank(max_rank));
            }
        }
    }
    fn detect_variant_or_rank(
        &mut self,
        line: &str,
        next_line: &str,
        open: &str,
        close: &str,
    ) -> Option<(String, DetectionStatus)> {
        let start = contains_at_least(&line, open, 1, true);
        let end = contains_at_least(&line, close, 1, true);
        if start && end {
            return Some((line.to_string(), DetectionStatus::Line));
        }
        let (combined, status) =
            combine_and_detect_multiple_matches(line, next_line, &[open, close], false, false);

        if status.is_found()
            && contains_at_least(&combined, open, 1, true)
            && contains_at_least(&combined, close, 1, true)
        {
            Some((combined, status))
        } else {
            None
        }
    }
    fn split_name_and_enclosed(line: &str, open: char, close: char) -> (String, String) {
        let index = line.find(open).unwrap_or(0);
        let rank_str = line[index..].replace(&[open, close][..], "");
        let name_part = line[..index].trim_end();
        (name_part.to_string(), rank_str)
    }
    pub fn is_variant_item(
        &mut self,
        line: &str,
        next_line: &str,
    ) -> Result<DetectionStatus, AppError> {
        // Check if the item is a mod eg. "Serration (RIVEN RANK 0)"
        if let Some((combine, status)) = self.detect_variant_or_rank(line, next_line, "(", ")") {
            let (name_part, rank_str) = Self::split_name_and_enclosed(&combine, '(', ')');
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
                println!("Detected Legendary Fusion Core: {}", combine);
                self.item_type = TradeItemType::FusionCore;
                self.sub_type = None; // Legendary Fusion Core is a special case
            } else if combine.contains("(RIVEN RANK ") {
                if combine.contains(" Riven Mod")
                    && self
                        .is_trade_item(&format!("{name_part} (Veiled)"), next_line)?
                        .is_found()
                {
                    self.item_type = TradeItemType::RivenVeiled;
                } else if let Some(pos) = name_part.rfind(' ') {
                    let (weapon, att) = name_part.split_at(pos);
                    self.item_type = TradeItemType::RivenUnVeiled;
                    self.unique_name =
                        format!("/WF_Special/Other/Riven/{}/{}", weapon.trim(), att.trim());
                }
            } else {
                self.is_trade_item(&name_part, next_line)?;
            }

            return Ok(status);
        }

        if let Some((combine, status)) = self.detect_variant_or_rank(line, next_line, "[", "]") {
            let (name_part, type_str) = Self::split_name_and_enclosed(&combine, '[', ']');
            if self.is_trade_item(&name_part, next_line)?.is_found() {
                self.sub_type = Some(SubType::variant(&type_str));
                return Ok(status);
            }
        }

        Ok(DetectionStatus::None)
    }

    pub fn is_arcane(&mut self, line: &str, next_line: &str) -> Result<DetectionStatus, AppError> {
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

    pub fn validate(&mut self, next_line: &str) -> Result<DetectionStatus, AppError> {
        for check in [Self::is_trade_item, Self::is_variant_item, Self::is_arcane] {
            let status = check(self, &self.raw.clone(), next_line)?;
            if status.is_found() {
                return Ok(status);
            }
        }
        Ok(DetectionStatus::None)
    }
    pub fn is_valid(&self) -> bool {
        !self.raw.is_empty()
    }

    pub fn display(&self) -> String {
        let mut name = format!("Raw: {} | Quantity: {}", self.raw, self.quantity);
        if !self.unique_name.is_empty() {
            name.push_str(&format!(" | Unique Name: {}", self.unique_name));
        }
        if let Some(sub_type) = &self.sub_type {
            name.push_str(&format!(" | Sub Type: {}", sub_type.display()));
        }
        if let Some((error, _)) = &self.error {
            name.push_str(&format!(" | Error: {}", error));
        }
        name
    }

    pub fn item_name(&self) -> String {
        let ch = states::cache().expect("Cache not found");
        let name = match ch
            .tradable_items()
            .get_by(&self.unique_name, "--item_by unique_name")
        {
            Ok(mach) => {
                if mach.is_some() {
                    mach.unwrap().name.clone()
                } else {
                    self.raw.clone()
                }
            }
            Err(_) => self.raw.clone(),
        };
        name
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
