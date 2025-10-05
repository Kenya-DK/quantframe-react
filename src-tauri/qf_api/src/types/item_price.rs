use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::SubType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemPrice {
    #[serde(default)]
    pub wfm_url: String,

    #[serde(default)]
    pub wfm_id: String,

    pub sub_type: Option<SubType>,

    #[serde(default)]
    pub avg_price: f64,

    pub supply: f64,
    #[serde(default)]
    pub demand: f64,
    #[serde(default)]
    pub closed_price: f64,
    pub datetime: DateTime<Utc>,
    #[serde(default)]
    pub donch_bot: f64,
    #[serde(default)]
    pub donch_top: f64,
    #[serde(default)]
    pub moving_avg: f64,
    #[serde(default)]
    pub open_price: f64,
    #[serde(default)]
    pub max_price: f64,
    #[serde(default)]
    pub median: f64,
    #[serde(default)]
    pub min_price: f64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub trading_tax: i32,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub wa_price: f64,
}
