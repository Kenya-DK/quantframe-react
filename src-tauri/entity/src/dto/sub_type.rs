use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct SubType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rank")]
    pub rank: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<i64>, // Charges remaining (Requiem mods)

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variant")]
    pub variant: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amber_stars")]
    pub amber_stars: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cyan_stars")]
    pub cyan_stars: Option<i64>,
}
impl Default for SubType {
    fn default() -> Self {
        Self {
            rank: None,
            variant: None,
            charges: None,
            amber_stars: None,
            cyan_stars: None,
        }
    }
}

impl SubType {
    pub fn new(
        rank: Option<i64>,
        charges: Option<i64>,
        variant: Option<String>,
        amber_stars: Option<i64>,
        cyan_stars: Option<i64>,
    ) -> Self {
        Self {
            rank,
            variant,
            charges,
            amber_stars,
            cyan_stars,
        }
    }

    pub fn rank(rank: i64) -> Self {
        Self {
            rank: Some(rank),
            variant: None,
            amber_stars: None,
            cyan_stars: None,
            charges: None,
        }
    }
    pub fn variant(variant: &str) -> Self {
        Self {
            rank: None,
            variant: Some(variant.to_string()),
            amber_stars: None,
            cyan_stars: None,
            charges: None,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            rank: None,
            variant: None,
            amber_stars: None,
            cyan_stars: None,
            charges: None,
        }
    }
    pub fn display(&self) -> String {
        let mut display = String::new();
        if let Some(rank) = self.rank {
            display.push_str(&format!("Rank: {} ", rank));
        }
        if let Some(charges) = self.charges {
            display.push_str(&format!("Charges: {} ", charges));
        }
        if let Some(variant) = &self.variant {
            display.push_str(&format!("Variant: {} ", variant));
        }
        if let Some(amber_stars) = self.amber_stars {
            display.push_str(&format!("Amber Stars: {} ", amber_stars));
        }
        if let Some(cyan_stars) = self.cyan_stars {
            display.push_str(&format!("Cyan Stars: {} ", cyan_stars));
        }
        display
    }
    pub fn shot_display(&self) -> String {
        let mut display = String::new();
        if let Some(rank) = self.rank {
            display.push_str(&format!("R {} ", rank));
        }
        if let Some(charges) = self.charges {
            display.push_str(&format!("Ch {} ", charges));
        }
        if let Some(variant) = &self.variant {
            display.push_str(&format!("V {} ", variant));
        }
        if let Some(amber_stars) = self.amber_stars {
            display.push_str(&format!("A {} ", amber_stars));
        }
        if let Some(cyan_stars) = self.cyan_stars {
            display.push_str(&format!("C {} ", cyan_stars));
        }
        display.trim().to_string()
    }
    pub fn is_empty(&self) -> bool {
        self.rank.is_none()
            && self.variant.is_none()
            && self.charges.is_none()
            && self.amber_stars.is_none()
            && self.cyan_stars.is_none()
    }
}
impl Hash for SubType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank.hash(state);
        self.variant.hash(state);
        self.charges.hash(state);
        self.amber_stars.hash(state);
        self.cyan_stars.hash(state);
    }
}
