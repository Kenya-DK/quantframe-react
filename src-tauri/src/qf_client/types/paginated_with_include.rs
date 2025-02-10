use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedWithInclude<T, K> {
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "limit")]
    pub limit: i64,

    #[serde(rename = "page")]
    pub page: i64,

    #[serde(rename = "results")]
    pub results: Vec<T>,

    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<K>,
}
