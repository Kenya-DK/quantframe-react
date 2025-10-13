use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationQueryDto {
    /// For pagination. Defines which page the results are fetched from.
    #[serde(default = "default_page")]
    pub page: i64,
    /// For pagination. Defines how many entries are returned per page.
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    25
}

impl Default for PaginationQueryDto {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }
}

impl PaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            page: if page < 1 { default_page() } else { page },
            limit: if limit < -1 { default_limit() } else { limit },
        }
    }
}
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

    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}
