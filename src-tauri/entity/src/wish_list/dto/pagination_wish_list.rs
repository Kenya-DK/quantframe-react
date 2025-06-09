use serde::{Deserialize, Serialize};

use crate::dto::{pagination::PaginationQueryDto, sort::SortDirection};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishListPaginationQueryDto {
    pub pagination: PaginationQueryDto,
    // Add any stock item specific filters or fields here
    pub query: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<SortDirection>,
    pub status: Option<String>,
    // You can add more fields as needed for filtering
}
impl WishListPaginationQueryDto {
    pub fn new(
        pagination: PaginationQueryDto,
        query: Option<String>,
        sort_by: Option<String>,
        sort_direction: Option<SortDirection>,
        status: Option<String>,
    ) -> Self {
        Self {
            pagination,
            query,
            sort_by,
            sort_direction,
            status,
        }
    }
}
