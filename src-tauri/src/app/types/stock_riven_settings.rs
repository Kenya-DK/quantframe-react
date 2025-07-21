use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockRivenSettings {
    pub min_profit: i64,
    pub threshold_percentage: f64,
    pub limit_to: i64,
    pub update_interval: i64, // in seconds
}

impl Default for StockRivenSettings {
    fn default() -> Self {
        StockRivenSettings {
            min_profit: 25,
            threshold_percentage: 15.0,
            limit_to: 5,
            update_interval: 120, // in seconds
        }
    }
}
