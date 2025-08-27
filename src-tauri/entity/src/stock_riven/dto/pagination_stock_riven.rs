use crate::{dto::*, enums::*, stock_riven::*};
use sea_orm::sea_query::Func;
use sea_orm::*;
use sea_query::Expr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRivenPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,
    // Add any stock riven specific filters or fields here
    #[serde(default)]
    pub query: FieldChange<String>,
    #[serde(default)]
    pub sort_by: FieldChange<String>,
    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,
    #[serde(default)]
    pub status: FieldChange<String>,
    // You can add more fields as needed for filtering
}
impl StockRivenPaginationQueryDto {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            pagination: PaginationQueryDto::new(page, limit),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            status: FieldChange::Ignore,
        }
    }
    pub fn get_query(&self) -> Select<stock_riven::Entity> {
        use FieldChange::*;
        let mut stmt = stock_riven::Entity::find();
        match &self.query {
            Value(q) => {
                stmt = stmt.filter(
                    Condition::any()
                        .add(
                            Expr::expr(Func::lower(Expr::col(stock_riven::Column::WeaponName)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col(stock_riven::Column::WfmWeaponUrl)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col(stock_riven::Column::ModName)))
                                .like(&format!("%{}%", q.to_lowercase())),
                        ),
                )
            }
            _ => {}
        }
        match self.status {
            Value(ref q) => stmt = stmt.filter(stock_riven::Column::Status.eq(q)),
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
                    "item_name" => stmt = stmt.order_by(stock_riven::Column::WeaponName, order),
                    "bought" => stmt = stmt.order_by(stock_riven::Column::Bought, order),
                    "status" => stmt = stmt.order_by(stock_riven::Column::Status, order),
                    "minimum_price" => {
                        stmt = stmt.order_by(stock_riven::Column::MinimumPrice, order)
                    }
                    "list_price" => stmt = stmt.order_by(stock_riven::Column::ListPrice, order),
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

    pub fn set_status(mut self, status: impl Into<String>) -> Self {
        self.status = FieldChange::Value(status.into());
        self
    }
}

impl Default for StockRivenPaginationQueryDto {
    fn default() -> Self {
        Self {
            pagination: PaginationQueryDto::default(),
            query: FieldChange::Ignore,
            sort_by: FieldChange::Ignore,
            sort_direction: FieldChange::Value(SortDirection::Asc),
            status: FieldChange::Ignore,
        }
    }
}
