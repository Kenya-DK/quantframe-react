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

    pub fn set_query(&mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self.clone()
    }

    pub fn set_sort_by(&mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self.clone()
    }
    pub fn set_sort_direction(&mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self.clone()
    }
    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self.clone()
    }
    pub fn set_item_type(&mut self, item_type: TransactionItemType) -> Self {
        self.item_type = Some(item_type);
        self.clone()
    }
    pub fn set_from_date(&mut self, from_date: DateTimeUtc) -> Self {
        self.from_date = Some(from_date);
        self.clone()
    }
    pub fn set_to_date(&mut self, to_date: DateTimeUtc) -> Self {
        self.to_date = Some(to_date);
        self.clone()
    }
}
