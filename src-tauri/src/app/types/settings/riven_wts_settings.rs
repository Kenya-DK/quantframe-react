use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RivenWtsSettings {
    pub min_profit: i64,
    pub threshold_percentage: f64,
    pub max_results: i64,
    /// Use the riven price prediction model instead of the live auction average.
    pub use_predict: bool,
}

impl Default for RivenWtsSettings {
    fn default() -> Self {
        Self {
            min_profit: 25,
            threshold_percentage: 15.0,
            max_results: 5,
            use_predict: false,
        }
    }
}
