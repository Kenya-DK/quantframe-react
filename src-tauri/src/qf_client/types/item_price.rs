use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPrice {
    #[serde(rename = "datetime")]
    pub datetime: String,
    #[serde(rename = "volume")]
    pub volume: f64,
    #[serde(rename = "min_price")]
    pub min_price: f64,
    #[serde(rename = "max_price")]
    pub max_price: f64,
    #[serde(rename = "avg_price")]
    pub avg_price: f64,
    #[serde(rename = "wa_price")]
    pub wa_price: f64,
    #[serde(rename = "median")]
    pub median: f64,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "moving_avg")]
    pub moving_avg: f64,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "trading_tax")]
    pub trading_tax: i32,
    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i32>,
}
