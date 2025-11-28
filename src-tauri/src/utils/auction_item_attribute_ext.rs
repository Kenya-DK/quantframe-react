use std::fmt::Display;

use entity::stock_riven::{CreateStockRiven, RivenAttribute};
use serde::{Deserialize, Serialize};
use utils::{get_location, warning, Error, LoggerOptions};
use wf_market::{
    enums::AuctionType,
    types::{Auction, AuctionWithOwner},
};

use crate::{
    cache::{client::CacheState, types::CacheRivenWeapon},
    enums::FindBy,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemAttributeDetails {
    #[serde(default)]
    pub grade: String,
    #[serde(default)]
    pub grade_value: i32,
    #[serde(default)]
    pub min: f64,
    #[serde(default)]
    pub max: f64,
    #[serde(default)]
    pub score: i32,
}
impl ItemAttributeDetails {
    pub fn set_grade(mut self, grade: impl Into<String>) -> Self {
        self.grade = grade.into();
        self
    }
    pub fn set_grade_value(mut self, grade_value: i32) -> Self {
        self.grade_value = grade_value;
        self
    }
    pub fn set_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }
    pub fn set_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
    pub fn set_score(mut self, score: i32) -> Self {
        self.score = score;
        self
    }
}
// Default implementation for ItemAttributeDetails
impl Default for ItemAttributeDetails {
    fn default() -> Self {
        ItemAttributeDetails {
            grade: "N/A".to_string(),
            grade_value: 0,
            min: 0,
            max: 0,
            score: -1,
        }
    }
}

// Extension trait for item attribute
pub trait ItemAttributeExt {
    fn get_details(&self) -> ItemAttributeDetails;
    fn update_details(&mut self, details: ItemAttributeDetails) -> Self;
}

impl ItemAttributeExt for ItemAttribute {
    fn get_details(&self) -> ItemAttributeDetails {
        if let Some(properties) = &self.properties {
            serde_json::from_value(properties.clone()).unwrap_or_else(|_| ItemAttributeDetails::default())
        } else {
            ItemAttributeDetails::default()
        }
    }

    fn update_details(&mut self, details: ItemAttributeDetails) -> Self {
        self.properties = Some(serde_json::to_value(details).unwrap());
        self.clone()
    }
}
