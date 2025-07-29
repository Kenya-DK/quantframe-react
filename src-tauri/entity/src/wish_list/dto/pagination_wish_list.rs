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
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: None,
            sort_by: None,
            sort_direction: Some(SortDirection::Asc),
            status: None,
        }
    }
    pub fn default() -> Self {
        Self {
            pagination: PaginationQueryDto::default(),
            query: None,
            sort_by: None,
            sort_direction: Some(SortDirection::Asc),
            status: None,
        }
    }
}
