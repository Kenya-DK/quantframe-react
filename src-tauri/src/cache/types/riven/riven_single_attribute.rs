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
    pub value: f64,
    #[serde(rename = "minValue")]
    pub min_value: f64,
    #[serde(rename = "maxValue")]
    pub max_value: f64,
    #[serde(rename = "letterGrade")]
    pub letter_grade: String,
    #[serde(rename = "grade")]
    pub grade: RivenAttributeGrade,
    pub positive: bool,
}

impl RivenSingleAttribute {
    pub fn new(
        url_name: impl Into<String>,
        value: f64,
        min_value: f64,
        max_value: f64,
        raw_random_value: f64,
        positive: bool,
        grade: RivenAttributeGrade,
    ) -> Self {
        Self {
            url_name: url_name.into(),
            value,
            min_value,
            max_value,
            letter_grade: get_attr_letter_grade_from_random_percent(raw_random_value).to_string(),
            positive,
            grade,
        }
    }
    pub fn apply_rank_multiplier(&mut self, disposition: f64, rank: f64) {
        let level_multiplier = disposition * ((rank + 1.0) / 9.0);
        self.value *= level_multiplier;
        self.min_value *= level_multiplier;
        self.max_value *= level_multiplier;
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
