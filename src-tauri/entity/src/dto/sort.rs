// Pagination DTOs (Rust equivalent of the provided TypeScript DTOs)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortQueryDto {
    /// The field to sort by
    pub sort_by: String,
    /// The direction of the sort
    pub sort_direction: SortDirection,
}
