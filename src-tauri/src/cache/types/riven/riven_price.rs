use serde::{Deserialize, Serialize};

/// Input for a riven price prediction.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RivenPriceInput {
    /// Weapon slug (WFM riven url name) or display name.
    pub weapon: String,
    #[serde(default)]
    pub re_rolls: u32,
    /// Positive attribute slugs/display names (up to 3 are used).
    #[serde(default)]
    pub positives: Vec<String>,
    /// Negative attribute slug/display name.
    #[serde(default)]
    pub negative: Option<String>,
}

/// The predicted price for a riven.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RivenPriceEstimate {
    pub price: f32,
    pub log_price: f32,
    pub weapon_idx: i32,
    pub attr_indices: [i32; 4],
}
