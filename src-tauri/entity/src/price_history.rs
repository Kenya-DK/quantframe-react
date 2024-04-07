use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceHistory {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "price")]
    pub price: i64,
}