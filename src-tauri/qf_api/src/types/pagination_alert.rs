use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::enums::*;
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,
    #[serde(default)]
    pub query: FieldChange<String>,
    #[serde(default)]
    pub sort_by: FieldChange<String>,
    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
    #[serde(default)]
    pub enabled: FieldChange<bool>,
}
impl AlertPaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            enabled: FieldChange::Ignore,
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
        match &self.enabled {
            Value(e) => query.push(format!("enabled={}", e)),
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
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = FieldChange::Value(enabled);
        self
    }
}

impl Display for AlertPaginationQueryDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query_str = match &self.query {
            FieldChange::Value(q) => format!("Some(\"{}\")", q),
            FieldChange::Ignore => "Ignore".to_string(),
            FieldChange::Null => "Null".to_string(),
        };
        let sort_by_str = match &self.sort_by {
            FieldChange::Value(s) => format!("Some(\"{}\")", s),
            FieldChange::Ignore => "Ignore".to_string(),
            FieldChange::Null => "Null".to_string(),
        };
        let sort_direction_str = match &self.sort_direction {
            FieldChange::Value(d) => format!("Some({:?})", d),
            FieldChange::Ignore => "Ignore".to_string(),
            FieldChange::Null => "Null".to_string(),
        };
        let enabled_str = match &self.enabled {
            FieldChange::Value(e) => format!("Some({})", e),
            FieldChange::Ignore => "Ignore".to_string(),
            FieldChange::Null => "Null".to_string(),
        };
        write!(
            f,
            "Page: {}, Limit: {}, Query: {}, Sort By: {}, Sort Direction: {}, Enabled: {}",
            self.pagination.page,
            self.pagination.limit,
            query_str,
            sort_by_str,
            sort_direction_str,
            enabled_str
        )
    }
}