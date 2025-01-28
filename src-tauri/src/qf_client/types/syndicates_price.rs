use entity::sub_type::SubType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyndicatesPrice {
    #[serde(rename = "syndicate_id")]
    pub syndicate_id: String,
    #[serde(rename = "syndicate")]
    pub syndicate: String,
    #[serde(rename = "standing")]
    pub standing: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,
    #[serde(rename = "wfm_url_name")]
    pub wfm_url_name: String,
    #[serde(rename = "min_price")]
    pub min_price: i64,
    #[serde(rename = "max_price")]
    pub max_price: i64,
    #[serde(rename = "volume")]
    pub volume: i64,
    #[serde(rename = "avg_price")]
    pub avg_price: f64,
    #[serde(rename = "datetime")]
    pub datetime: String,
    #[serde(rename = "sub_type")]
    pub sub_type: Option<SubType>,
}
