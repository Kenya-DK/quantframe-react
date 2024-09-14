use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct MatchRivenStruct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<MinMaxStruct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastery_rank: Option<MinMaxStruct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re_rolls: Option<MinMaxStruct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polarity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_negative: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<MatchRivenAttributeStruct>>,
}

impl MatchRivenStruct {
    pub fn new() -> Self {
        Self {
            enabled: Some(false),
            rank: None,
            mastery_rank: None,
            re_rolls: None,
            polarity: None,
            similarity: None,
            required_negative: None,
            attributes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchRivenAttributeStruct {
    pub url_name: String,
    pub positive: bool,
    pub is_required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinMaxStruct {
    pub min: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
}