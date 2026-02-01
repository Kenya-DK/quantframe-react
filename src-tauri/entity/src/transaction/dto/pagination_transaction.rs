use crate::{dto::*, enums::*, transaction::*};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::Func;
use sea_orm::*;
use sea_query::Expr;
use serde::{Deserialize, Serialize};
use utils::sorting::SortDirection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPaginationQueryDto {
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
    pub transaction_type: FieldChange<TransactionType>,
    #[serde(default)]
    pub item_type: FieldChange<TransactionItemType>,
    #[serde(default)]
    pub from_date: FieldChange<DateTimeUtc>,
    #[serde(default)]
    pub to_date: FieldChange<DateTimeUtc>,
    #[serde(default)]
    pub wfm_id: FieldChange<String>,
    #[serde(default)]
    pub wfm_url: FieldChange<String>,
    #[serde(default)]
    pub unique_name: FieldChange<String>,
    #[serde(default)]
    pub sub_type: FieldChange<SubType>,
    #[serde(default)]
    pub tags: FieldChange<Vec<String>>,
}
impl TransactionPaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            transaction_type: FieldChange::Ignore,
            item_type: FieldChange::Ignore,
            from_date: FieldChange::Ignore,
            to_date: FieldChange::Ignore,
            wfm_id: FieldChange::Ignore,
            wfm_url: FieldChange::Ignore,
            unique_name: FieldChange::Ignore,
            sub_type: FieldChange::Ignore,
            tags: FieldChange::Ignore,
        }
    }
    pub fn get_query(&self) -> Select<Entity> {
        use FieldChange::*;
        let mut stmt = Entity::find();
        match &self.query {
            Value(q) => {
                stmt = stmt.filter(
                    Condition::any()
                        .add(
                            Expr::expr(Func::lower(Expr::col(transaction::Column::WfmUrl)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col(transaction::Column::ItemName)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col(transaction::Column::UserName)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        ),
                )
            }
            _ => {}
        }
        match &self.transaction_type {
            Value(q) => stmt = stmt.filter(transaction::Column::TransactionType.eq(q.to_string())),
            _ => {}
        }
        match &self.item_type {
            Value(q) => stmt = stmt.filter(transaction::Column::ItemType.eq(q.to_string())),
            _ => {}
        }
        match &self.from_date {
            Value(from_date) => stmt = stmt.filter(transaction::Column::CreatedAt.gte(*from_date)),
            _ => {}
        }
        match &self.to_date {
            Value(to_date) => stmt = stmt.filter(transaction::Column::CreatedAt.lte(*to_date)),
            _ => {}
        }
        match &self.wfm_id {
            Value(wfm_id) => stmt = stmt.filter(transaction::Column::WfmId.eq(wfm_id)),
            _ => {}
        }
        match &self.wfm_url {
            Value(wfm_url) => stmt = stmt.filter(transaction::Column::WfmUrl.eq(wfm_url)),
            _ => {}
        }
        match &self.unique_name {
            Value(unique_name) => {
                stmt = stmt.filter(transaction::Column::ItemUniqueName.eq(unique_name))
            }
            _ => {}
        }
        match &self.sub_type {
            Value(sub_type) => {
                stmt = stmt.filter(transaction::Column::SubType.eq(sub_type.clone()))
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
                                Expr::col(transaction::Column::Tags)
                                    .like(format!("%{}%", tag.trim())),
                            );
                        }
                    }
                    stmt = stmt.filter(tag_condition);
                }
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
                    "wfm_url" => stmt = stmt.order_by(transaction::Column::WfmUrl, order),
                    "price" => stmt = stmt.order_by(transaction::Column::Price, order),
                    "transaction_type" => {
                        stmt = stmt.order_by(transaction::Column::TransactionType, order)
                    }
                    "item_type" => stmt = stmt.order_by(transaction::Column::ItemType, order),
                    "created_at" => stmt = stmt.order_by(transaction::Column::CreatedAt, order),
                    "item_name" => stmt = stmt.order_by(transaction::Column::ItemName, order),
                    "user_name" => stmt = stmt.order_by(transaction::Column::UserName, order),
                    "profit" => stmt = stmt.order_by(transaction::Column::Profit, order),
                    "credits" => stmt = stmt.order_by(transaction::Column::Credits, order),
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
    pub fn set_transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = FieldChange::Value(transaction_type);
        self
    }
    pub fn set_item_type(mut self, item_type: TransactionItemType) -> Self {
        self.item_type = FieldChange::Value(item_type);
        self
    }
    pub fn set_from_date(mut self, from_date: DateTimeUtc) -> Self {
        self.from_date = FieldChange::Value(from_date);
        self
    }
    pub fn set_to_date(mut self, to_date: DateTimeUtc) -> Self {
        self.to_date = FieldChange::Value(to_date);
        self
    }
    pub fn set_wfm_id(&mut self, wfm_id: impl Into<String>) -> Self {
        self.wfm_id = FieldChange::Value(wfm_id.into());
        self.clone()
    }
    pub fn set_wfm_url(&mut self, wfm_url: impl Into<String>) -> Self {
        self.wfm_url = FieldChange::Value(wfm_url.into());
        self.clone()
    }
    pub fn set_unique_name(mut self, unique_name: impl Into<String>) -> Self {
        self.unique_name = FieldChange::Value(unique_name.into());
        self
    }
    pub fn set_sub_type(mut self, sub_type: Option<SubType>) -> Self {
        self.sub_type = match sub_type {
            Some(v) => FieldChange::Value(v),
            None => FieldChange::Null,
        };
        self
    }
    pub fn set_tag(mut self, tags: Vec<String>) -> Self {
        self.tags = FieldChange::Value(tags);
        self
    }
}

impl Default for TransactionPaginationQueryDto {
    fn default() -> Self {
        Self {
            pagination: PaginationQueryDto::default(),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            transaction_type: FieldChange::Ignore,
            item_type: FieldChange::Ignore,
            from_date: FieldChange::Ignore,
            to_date: FieldChange::Ignore,
            wfm_id: FieldChange::Ignore,
            wfm_url: FieldChange::Ignore,
            unique_name: FieldChange::Ignore,
            sub_type: FieldChange::Ignore,
            tags: FieldChange::Ignore,
        }
    }
}
