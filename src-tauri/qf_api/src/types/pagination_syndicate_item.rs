use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::enums::*;
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyndicateItemPricePaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,
    // Add any stock item specific filters or fields here
    #[serde(default)]
    pub query: FieldChange<String>,
    #[serde(default)]
    pub sort_by: FieldChange<String>,
    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,

    #[serde(default)]
    pub volume_gt: FieldChange<i64>,
    #[serde(default)]
    pub volume_lt: FieldChange<i64>,
    #[serde(default)]
    pub standing_cost_gt: FieldChange<i64>,
    #[serde(default)]
    pub standing_cost_lt: FieldChange<i64>,
}
impl SyndicateItemPricePaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            volume_gt: FieldChange::Ignore,
            volume_lt: FieldChange::Ignore,
            standing_cost_gt: FieldChange::Ignore,
            standing_cost_lt: FieldChange::Ignore,
        }
    }
    pub fn get_query(&self) -> String {
        use FieldChange::*;
        let mut query: Vec<String> = Vec::new();
        query.push(format!("page={}", self.pagination.page));
        query.push(format!("limit={}", self.pagination.limit));
        match &self.query {
            Value(q) => query.push(format!("query={}", q)),
            _ => {}
        }
        match &self.sort_by {
            Value(s) => query.push(format!("sort_by={}", s)),
            _ => {}
        }
        match &self.sort_direction {
            Value(d) => query.push(format!("sort_direction={}", d.to_string())),
            _ => {}
        }
        match &self.volume_gt {
            Value(v) => query.push(format!("volumeGt={}", v)),
            _ => {}
        }
        match &self.volume_lt {
            Value(v) => query.push(format!("volumeLt={}", v)),
            _ => {}
        }
        match &self.standing_cost_gt {
            Value(v) => query.push(format!("standingCostGt={}", v)),
            _ => {}
        }
        match &self.standing_cost_lt {
            Value(v) => query.push(format!("standingCostLt={}", v)),
            _ => {}
        }
        query.join("&")
    }
    pub fn set_pagination(mut self, pagination: PaginationQueryDto) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn set_query(mut self, query: impl Into<String>) -> Self {
        self.query = FieldChange::Value(query.into());
        self
    }

    pub fn set_sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = FieldChange::Value(sort_by.into());
        self
    }

    pub fn set_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = FieldChange::Value(sort_direction);
        self
    }
}
