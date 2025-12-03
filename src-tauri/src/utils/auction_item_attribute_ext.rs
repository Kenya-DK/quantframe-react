use entity::enums::RivenAttributeGrade;
use serde::{Deserialize, Serialize};
use wf_market::types::ItemAttribute;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemAttributeDetails {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub grade: String,
    #[serde(default)]
    pub role_grade: RivenAttributeGrade,
    #[serde(default)]
    pub min: f32,
    #[serde(default)]
    pub max: f32,
    #[serde(default)]
    pub score: i32,
}
impl ItemAttributeDetails {
    pub fn new(
        tag: impl Into<String>,
        grade: impl Into<String>,
        role_grade: RivenAttributeGrade,
        min: f32,
        max: f32,
        score: i32,
    ) -> Self {
        Self {
            tag: tag.into(),
            grade: grade.into(),
            role_grade,
            min,
            max,
            score,
        }
    }
    pub fn set_grade(mut self, grade: impl Into<String>) -> Self {
        self.grade = grade.into();
        self
    }
    pub fn set_min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }
    pub fn set_max(mut self, max: f32) -> Self {
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
            tag: "N/A".to_string(),
            grade: "N/A".to_string(),
            role_grade: RivenAttributeGrade::Unknown,
            min: 0.0,
            max: 0.0,
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
            serde_json::from_value(properties.clone())
                .unwrap_or_else(|_| ItemAttributeDetails::default())
        } else {
            ItemAttributeDetails::default()
        }
    }

    fn update_details(&mut self, details: ItemAttributeDetails) -> Self {
        self.properties = Some(serde_json::to_value(details).unwrap());
        self.clone()
    }
}
