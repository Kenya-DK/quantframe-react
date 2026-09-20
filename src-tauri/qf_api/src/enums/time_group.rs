use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimeGroup {
    Day,
    Hour,
    Month,
}

impl ToString for TimeGroup {
    fn to_string(&self) -> String {
        match self {
            TimeGroup::Day => "day".to_string(),
            TimeGroup::Hour => "hour".to_string(),
            TimeGroup::Month => "month".to_string(),
        }
    }
}