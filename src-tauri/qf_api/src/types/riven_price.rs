use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RivenPrice {
    #[serde(default)]
    pub wfm_id: String,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub wfm_url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub min_price: f64,
    #[serde(default)]
    pub max_price: f64,
    #[serde(default)]
    pub avg_price: f64,
    #[serde(default)]
    pub median_price: f64,
    #[serde(default)]
    pub datetime: String,
}
