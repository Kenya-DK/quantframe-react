use std::fmt::Display;

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
    unique_id: String,
    localization: String,
    current_value: f64,
    worst_case: f64,
    best_case: f64,
    raw_random_value: f64,
    letter_grade: String,
}

impl RivenSingleAttribute {
    pub fn new(
        unique_id: impl Into<String>,
        localization: impl Into<String>,
        current_value: f64,
        worst_case: f64,
        best_case: f64,
        raw_random_value: f64,
    ) -> Self {
        Self {
            unique_id: unique_id.into(),
            localization: localization.into(),
            current_value,
            worst_case,
            best_case,
            raw_random_value,
            letter_grade: get_attr_letter_grade_from_random_percent(raw_random_value).to_string(),
        }
    }
    pub fn apply_rank_multiplier(&mut self, disposition: f64, rank: f64) {
        let level_multiplier = disposition * ((rank + 1.0) / 9.0);
        self.current_value *= level_multiplier;
        self.worst_case *= level_multiplier;
        self.best_case *= level_multiplier;
    }
}

impl Display for RivenSingleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): Current Value: {:.2}, Worst Case: {:.2}, Best Case: {:.2}, Grade: {}",
            self.localization,
            self.unique_id,
            self.current_value,
            self.worst_case,
            self.best_case,
            self.letter_grade
        )
    }
}