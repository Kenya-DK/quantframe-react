use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::enums::*;
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivenPricePaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,
    // Add any stock item specific filters or fields here
    #[serde(default)]
    pub query: FieldChange<String>,
    #[serde(default)]
    pub sort_by: FieldChange<String>,
    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,

    from_date: String,
    to_date: String,

    #[serde(default)]
    pub volume_gt: FieldChange<i64>,
    #[serde(default)]
    pub volume_lt: FieldChange<i64>,

    #[serde(default)]
    pub avg_price_gt: FieldChange<i64>,
    #[serde(default)]
    pub avg_price_lt: FieldChange<i64>,

    #[serde(default)]
    pub min_price_gt: FieldChange<i64>,
    #[serde(default)]
    pub min_price_lt: FieldChange<i64>,

    #[serde(default)]
    pub max_price_gt: FieldChange<i64>,
    #[serde(default)]
    pub max_price_lt: FieldChange<i64>,
}
impl RivenPricePaginationQueryDto {
    pub fn new(page: i64, limit: i64, from_date: String, to_date: String) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            from_date,
            to_date,
            volume_gt: FieldChange::Ignore,
            volume_lt: FieldChange::Ignore,
            avg_price_gt: FieldChange::Ignore,
            avg_price_lt: FieldChange::Ignore,
            min_price_gt: FieldChange::Ignore,
            min_price_lt: FieldChange::Ignore,
            max_price_gt: FieldChange::Ignore,
            max_price_lt: FieldChange::Ignore,
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
        match &self.avg_price_gt {
            Value(v) => query.push(format!("avgPriceGt={}", v)),
            _ => {}
        }
        match &self.avg_price_lt {
            Value(v) => query.push(format!("avgPriceLt={}", v)),
            _ => {}
        }
        match &self.min_price_gt {
            Value(v) => query.push(format!("minPriceGt={}", v)),
            _ => {}
        }
        match &self.min_price_lt {
            Value(v) => query.push(format!("minPriceLt={}", v)),
            _ => {}
        }
        match &self.max_price_gt {
            Value(v) => query.push(format!("maxPriceGt={}", v)),
            _ => {}
        }
        match &self.max_price_lt {
            Value(v) => query.push(format!("maxPriceLt={}", v)),
            _ => {}
        }
        query.push(format!("from_date={}", self.from_date));
        query.push(format!("to_date={}", self.to_date));

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

impl Display for RivenPricePaginationQueryDto {
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
        let from_date_str = self.from_date.clone();
        let to_date_str = self.to_date.clone();
        write!(
            f,
            "Page: {}, Limit: {}, Query: {}, Sort By: {}, Sort Direction: {}, From Date: {}, To Date: {}",
            self.pagination.page,
            self.pagination.limit,
            query_str,
            sort_by_str,
            sort_direction_str,
            from_date_str,
            to_date_str,
        )
    }
}
