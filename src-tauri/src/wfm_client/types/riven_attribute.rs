use serde::{Deserialize, Serialize};
#[derive(sqlx::Decode, Serialize, Deserialize, Clone, Debug)]
pub struct RivenAttribute {
    #[serde(rename = "positive")]
    pub positive: bool,

    #[serde(rename = "value")]
    pub value: f64,

    #[serde(rename = "url_name")]
    pub url_name: String,
}
