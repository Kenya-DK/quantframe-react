use entity::dto::SubType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ItemPriceInfo {
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(rename = "order_type")]
    pub order_type: String,

    #[serde(rename = "volume")]
    pub volume: f64,

    #[serde(rename = "max_price")]
    pub max_price: f64,

    #[serde(rename = "min_price")]
    pub min_price: f64,

    #[serde(rename = "avg_price")]
    pub avg_price: f64,

    #[serde(rename = "moving_avg")]
    pub moving_avg: Option<f64>,

    #[serde(rename = "median")]
    pub median: f64,

    #[serde(rename = "profit")]
    pub profit: f64,

    #[serde(rename = "profit_margin", default)]
    pub profit_margin: f64,

    #[serde(rename = "trading_tax")]
    pub trading_tax: i64,

    #[serde(rename = "week_price_shift")]
    pub week_price_shift: f64,

    #[serde(rename = "sub_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
}
// Add default values for ItemPriceInfo
impl Default for ItemPriceInfo {
    fn default() -> Self {
        ItemPriceInfo {
            wfm_url: "".to_string(),
            wfm_id: "".to_string(),
            order_type: "".to_string(),
            volume: 0.0,
            max_price: 0.0,
            min_price: 0.0,
            avg_price: 0.0,
            moving_avg: None,
            median: 0.0,
            profit: 0.0,
            profit_margin: 0.0,
            trading_tax: 0,
            week_price_shift: 0.0,
            sub_type: None,
        }
    }
}
