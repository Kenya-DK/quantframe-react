use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct PriceHistoryVec(pub Vec<PriceHistory>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceHistory {

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "price")]
    pub price: i64,
}
impl PriceHistory {
    pub fn new(created_at: String, price: i64) -> Self {
        Self {
            created_at,
            price,
        }
    }
}