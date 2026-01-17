use sea_orm::sea_query::Func;
use sea_orm::*;
use sea_query::Expr;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{dto::*, enums::*, trade_entry::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEntryPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,
    // Add any stock item specific filters or fields here
    #[serde(default)]
    pub query: FieldChange<String>,
    #[serde(default)]
    pub group: FieldChange<String>,
    #[serde(default)]
    pub sort_by: FieldChange<String>,
    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
    #[serde(default)]
    pub tags: FieldChange<Vec<String>>,
    // You can add more fields as needed for filtering
}
impl TradeEntryPaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            tags: FieldChange::Ignore,
            group: FieldChange::Ignore,
        }
    }
    pub fn get_query(&self) -> Select<trade_entry::Entity> {
        use FieldChange::*;
        let mut stmt = trade_entry::Entity::find();
        match &self.query {
            Value(q) => {
                stmt = stmt.filter(
                    Condition::any()
                        .add(
                            Expr::expr(Func::lower(Expr::col(trade_entry::Column::Name)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col(trade_entry::Column::SubType)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        ),
                )
            }
            _ => {}
        }
        match &self.tags {
            Value(tags) => {
                if !tags.is_empty() {
                    // Create a condition that matches any of the provided tags
                    let mut tag_condition = Condition::any();
                    for tag in tags {
                        if !tag.trim().is_empty() {
                            tag_condition = tag_condition.add(
                                Expr::col(trade_entry::Column::Tags)
                                    .like(format!("%{}%", tag.trim())),
                            );
                        }
                    }
                    stmt = stmt.filter(tag_condition);
                }
            }
            _ => {}
        }
        match &self.group {
            Value(group) => {
                stmt = stmt.filter(trade_entry::Column::Group.eq(group.to_owned()));
            }
            _ => {}
        }
        match &self.sort_by {
            Value(sort_by) => {
                let dir = match &self.sort_direction {
                    Value(dir) => dir,
                    _ => &SortDirection::Asc,
                };
                let order = match dir {
                    SortDirection::Asc => Order::Asc,
                    SortDirection::Desc => Order::Desc,
                };
                // Only allow sorting by known columns for safety
                match sort_by.as_str() {
                    "name" => stmt = stmt.order_by(trade_entry::Column::Name, order),
                    "price" => stmt = stmt.order_by(trade_entry::Column::Price, order),
                    _ => {}
                }
            }
            _ => {}
        }
        stmt
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
    pub fn set_tags(mut self, tags: Vec<impl Into<String>>) -> Self {
        self.tags = FieldChange::Value(tags.into_iter().map(|s| s.into()).collect());
        self
    }
}

impl Display for TradeEntryPaginationQueryDto {
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
        let group_str = match &self.group {
            FieldChange::Value(g) => format!("Some(\"{}\")", g),
            FieldChange::Ignore => "Ignore".to_string(),
            FieldChange::Null => "Null".to_string(),
        };
        write!(
            f,
            "Page: {}, Limit: {}, Query: {}, Sort By: {}, Sort Direction: {}, Group: {}",
            self.pagination.page,
            self.pagination.limit,
            query_str,
            sort_by_str,
            sort_direction_str,
            group_str,
        )
    }
}
impl Default for TradeEntryPaginationQueryDto {
    fn default() -> Self {
        Self {
            pagination: PaginationQueryDto::default(),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            group: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            tags: FieldChange::Ignore,
        }
    }
}
