// Pagination DTOs (Rust equivalent of the provided TypeScript DTOs)
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
            limit: if limit < 1 { default_limit() } else { limit },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedDto<T> {
    /// The total number of items in the database
    pub total: i64,
    /// The number of items returned in this request
    pub limit: i64,
    /// The current page
    pub page: i64,
    /// The results for this page
    pub results: Vec<T>,
}

impl<T> PaginatedDto<T> {
    pub fn new(total: i64, limit: i64, page: i64, results: Vec<T>) -> Self {
        Self {
            total,
            limit,
            page,
            results,
        }
    }
}
