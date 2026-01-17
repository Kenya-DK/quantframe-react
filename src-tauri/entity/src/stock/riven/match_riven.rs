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

impl MatchRivenStruct {
    pub fn get_auction_search_query(&self) -> (Option<Vec<String>>, Option<String>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<String>) {
        if self.enabled.is_some() && !self.enabled.unwrap() {
            return (None, None, None, None, None, None, None);
        }
            // Find Positive stats
            let positive_stats = self
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| a.positive && a.is_required)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Find Negative stats
            let mut negative_stats = self
                .attributes
                .clone()
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|a| !a.positive && a.is_required)
                .map(|a| a.url_name.clone())
                .collect::<Vec<_>>();

            // Match Required Negative stats
            if self.required_negative.unwrap_or(false) {
                negative_stats.push("has".to_string());
            }

            // Match Rerolls
            let mut min_rerolls: Option<i64> = None;
            let mut max_rerolls: Option<i64> = None;
            if self.re_rolls.is_some() {
                let re_rolls = self.re_rolls.as_ref().unwrap();
                if re_rolls.min != 0 {
                    min_rerolls = Some(re_rolls.min);
                }
                max_rerolls = re_rolls.max;
            }

            // Match Mastery Rank
            let mut min_mastery_rank: Option<i64> = None;
            let mut max_mastery_rank: Option<i64> = None;
            if self.mastery_rank.is_some() {
                let mastery_rank = &self.mastery_rank.as_ref().unwrap();
                if mastery_rank.min != 0 {
                    min_mastery_rank = Some(mastery_rank.min);
                }
                max_mastery_rank = mastery_rank.max;
            }

            // Match Polarity
            let mut polarity: Option<String> = None;
            if self.polarity.is_some() {
                if self.polarity.clone().unwrap() == "" {
                    polarity = Some("any".to_string());
                } else {
                    polarity = Some(self.polarity.clone().unwrap());
                }
            }
            return (Some(positive_stats), negative_stats.get(0).cloned(), min_rerolls, max_rerolls, min_mastery_rank, max_mastery_rank, polarity);
    }   

}