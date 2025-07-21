use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paginated<T> {
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "limit")]
    pub limit: i64,

    #[serde(rename = "page")]
    pub page: i64,

    #[serde(rename = "results")]
    pub results: Vec<T>,
}
