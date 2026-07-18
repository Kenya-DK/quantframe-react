use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWtsSettings {
    pub min_sma: i64,
    pub min_profit: i64,
    pub max_price_drop: i64,
    pub min_listings_below: i64,
}

impl Default for ItemWtsSettings {
    fn default() -> Self {
        Self {
            min_sma: 3,
            min_profit: 10,
            max_price_drop: -1,
            min_listings_below: -1,
        }
    }
}
