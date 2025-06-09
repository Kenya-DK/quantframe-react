use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{pagination::PaginationQueryDto, sort::SortDirection},
    transaction::transaction::{TransactionItemType, TransactionType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPaginationQueryDto {
    pub pagination: PaginationQueryDto,
    // Add any stock item specific filters or fields here
    pub query: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<SortDirection>,
    pub transaction_type: Option<TransactionType>, // e.g., "sale" or "purchase"
    pub item_type: Option<TransactionItemType>,    // e.g., "item" or "riven"
    pub from_date: Option<DateTimeUtc>,            // Optional field for filtering by date range
    pub to_date: Option<DateTimeUtc>,              // Optional field for filtering by date range
                                                   // You can add more fields as needed for filtering
}
impl TransactionPaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: None,
            sort_by: None,
            sort_direction: Some(SortDirection::Asc),
            transaction_type: None,
            item_type: None,
            from_date: None,
            to_date: None,
        }
    }
    pub fn default() -> Self {
        Self {
            pagination: PaginationQueryDto::default(),
            query: None,
            sort_by: None,
            sort_direction: Some(SortDirection::Asc),
            transaction_type: None,
            item_type: None,
            from_date: None,
            to_date: None,
        }
    }
    pub fn set_pagination(&mut self, pagination: PaginationQueryDto) -> Self {
        self.pagination = pagination;
        self.clone()
    }

    pub fn set_query(&mut self, query: Option<String>) -> Self {
        self.query = query;
        self.clone()
    }

    pub fn set_sort_by(&mut self, sort_by: Option<String>) -> Self {
        self.sort_by = sort_by;
        self.clone()
    }
    pub fn set_sort_direction(&mut self, sort_direction: Option<SortDirection>) -> Self {
        self.sort_direction = sort_direction;
        self.clone()
    }
    pub fn set_transaction_type(&mut self, transaction_type: Option<TransactionType>) -> Self {
        self.transaction_type = transaction_type;
        self.clone()
    }
    pub fn set_item_type(&mut self, item_type: Option<TransactionItemType>) -> Self {
        self.item_type = item_type;
        self.clone()
    }
    pub fn set_from_date(&mut self, from_date: Option<DateTimeUtc>) -> Self {
        self.from_date = from_date;
        self.clone()
    }
    pub fn set_to_date(&mut self, to_date: Option<DateTimeUtc>) -> Self {
        self.to_date = to_date;
        self.clone()
    }
}
