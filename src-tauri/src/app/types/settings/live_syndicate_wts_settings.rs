use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveSyndicateWtsSettings {
    pub syndicates: Vec<String>,
    pub max_rank_for_type: Vec<String>,
    pub volume_threshold: i64,
    pub max_standing_cost: i64,
    pub min_price: i64,
    pub max_price_drop: i64,
    pub min_listings_below: i64,
}

impl Default for LiveSyndicateWtsSettings {
    fn default() -> Self {
        Self {
            syndicates: Vec::new(),
            max_rank_for_type: vec!["mod".to_string()],
            volume_threshold: 10,
            max_standing_cost: 10000,
            min_price: 10,
            max_price_drop: -1,
            min_listings_below: -1,
        }
    }
}
