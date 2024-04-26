use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct PriceHistoryVec(pub Vec<PriceHistory>);

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
impl PriceHistory {
    pub fn new(user_id: String, name: String, created_at: String, price: i64) -> Self {
        Self {
            user_id,
            name,
            created_at,
            price,
        }
    }
}