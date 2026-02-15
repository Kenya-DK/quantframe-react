use std::fmt::Display;

use entity::enums::RivenAttributeGrade;
use serde::{Deserialize, Serialize};

static RIVEN_SCALE_GRADE: [&str; 11] =
    ["F", "C-", "C", "C+", "B-", "B", "B+", "A-", "A", "A+", "S"];
fn get_attr_letter_grade_from_random_percent(random_per_one: f64) -> &'static str {
    let num = random_per_one * 20.0 - 10.0;
    let mut num2 = -11.5;
    let mut result = "??";

    for grade in RIVEN_SCALE_GRADE.iter() {
        num2 += 2.0;

        if *grade == "B" {
            num2 += 1.0;
        }

        if num2 > num {
            result = grade;
            break;
        }
    }

    result
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RivenSingleAttribute {
    pub url_name: String,
    pub tag: String,
    pub value: f64,
    pub raw_value: f64,
    #[serde(rename = "minValue")]
    pub min_value: f64,
    #[serde(rename = "maxValue")]
    pub max_value: f64,
    #[serde(rename = "letterGrade")]
    pub letter_grade: String,
    #[serde(rename = "grade")]
    pub grade: RivenAttributeGrade,
    pub positive: bool,

    // Extra properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl RivenSingleAttribute {
    pub fn new(
        url_name: impl Into<String>,
        tag: impl Into<String>,
        value: f64,
        min_value: f64,
        max_value: f64,
        raw_value: f64,
        positive: bool,
        grade: RivenAttributeGrade,
    ) -> Self {
        Self {
            url_name: url_name.into(),
            tag: tag.into(),
            value,
            min_value,
            raw_value,
            max_value,
            letter_grade: get_attr_letter_grade_from_random_percent(raw_value).to_string(),
            positive,
            grade,
            properties: None,
        }
    }
    pub fn new_base(
        url_name: impl Into<String>,
        tag: impl Into<String>,
        value: f64,
        raw_value: f64,
        is_positive: bool,
    ) -> Self {
        Self {
            url_name: url_name.into(),
            tag: tag.into(),
            value,
            raw_value,
            min_value: 0.0,
            max_value: 0.0,
            letter_grade: "??".to_string(),
            positive: is_positive,
            grade: RivenAttributeGrade::Unknown,
            properties: None,
        }
    }
    pub fn apply_rank_multiplier(&mut self, disposition: f64, rank: f64) {
        let level_multiplier = disposition * ((rank + 1.0) / 9.0);
        self.value *= level_multiplier;
        self.min_value *= level_multiplier;
        self.max_value *= level_multiplier;
    }
    pub fn get_property_value<T>(&self, key: impl Into<String>, default: T) -> T
    where
        T: Default + serde::de::DeserializeOwned,
    {
        let key = key.into();
        if let Some(props) = &self.properties {
            if let Some(value) = props.get(&key) {
                return serde_json::from_value(value.clone()).unwrap();
            }
        }
        default
    }
    pub fn set_property_value<T>(&mut self, key: impl Into<String>, value: T)
    where
        T: serde::Serialize,
    {
        let key = key.into();
        let value = serde_json::to_value(value).unwrap();
        if let Some(props) = &mut self.properties {
            props.as_object_mut().unwrap().insert(key, value);
        } else {
            let mut map = serde_json::Map::new();
            map.insert(key, value);
            self.properties = Some(serde_json::Value::Object(map));
        }
    }
}

impl Display for RivenSingleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Current Value: {:.2}, Min Value: {:.2}, Max Value: {:.2}, Grade: {}, Positive: {}",
            self.url_name,
            self.value,
            self.min_value,
            self.max_value,
            self.letter_grade,
            self.positive
        )
    }
}
