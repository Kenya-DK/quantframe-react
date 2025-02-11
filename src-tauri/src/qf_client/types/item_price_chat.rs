use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPriceChat {
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "volume_chart")]
    pub volume_chart: Vec<f64>,
    #[serde(rename = "min_price_chart")]
    pub min_price_chart: Vec<f64>,
    #[serde(rename = "max_price_chart")]
    pub max_price_chart: Vec<f64>,
    #[serde(rename = "avg_price")]
    pub avg_price: Vec<f64>,
    #[serde(rename = "median_price_chart")]
    pub median_price_chart: Vec<f64>,
    #[serde(rename = "moving_avg_chart")]
    pub moving_avg_chart: Vec<f64>,
}
