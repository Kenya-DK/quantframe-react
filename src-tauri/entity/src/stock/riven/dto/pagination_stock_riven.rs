use serde::{Deserialize, Serialize};

use crate::dto::{pagination::PaginationQueryDto, sort::SortDirection};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRivenPaginationQueryDto {
    pub pagination: PaginationQueryDto,
    // Add any stock riven specific filters or fields here
    pub query: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<SortDirection>,
    pub status: Option<String>,
    // You can add more fields as needed for filtering
}
