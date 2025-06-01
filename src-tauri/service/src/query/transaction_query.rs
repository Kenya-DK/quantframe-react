use ::entity::transaction::dto::TransactionPaginationQueryDto;
use ::entity::transaction::{transaction, transaction::Entity as Transaction};

use sea_orm::sea_query::Func;
use sea_orm::*;
use sea_query::Expr;

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_all_v2(
        db: &DbConn,
        query: TransactionPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedDto<transaction::Model>, DbErr> {
        let mut stmt = Transaction::find();

        // Filtering by query (search)
        if let Some(ref q) = query.query {
            // Case-sensitive search in WfmUrl and ItemName columns
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
            );
        }
        // Filtering by transaction type
        if let Some(ref transaction_type) = query.transaction_type {
            stmt =
                stmt.filter(transaction::Column::TransactionType.eq(transaction_type.to_string()));
        }
        // Filtering by item type
        if let Some(ref item_type) = query.item_type {
            stmt = stmt.filter(transaction::Column::ItemType.eq(item_type.to_string()));
        }
        // Filtering by date range
        if let Some(ref from_date) = query.from_date {
            stmt = stmt.filter(
                transaction::Column::CreatedAt
                    .gte(from_date.parse::<chrono::NaiveDateTime>().unwrap()),
            );
        }
        if let Some(ref to_date) = query.to_date {
            stmt = stmt.filter(
                transaction::Column::CreatedAt
                    .lte(to_date.parse::<chrono::NaiveDateTime>().unwrap()),
            );
        }
        // Sorting
        if let Some(ref sort_by) = query.sort_by {
            let dir = query
                .sort_direction
                .as_ref()
                .unwrap_or(&::entity::dto::sort::SortDirection::Asc);
            let order = match dir {
                ::entity::dto::sort::SortDirection::Asc => Order::Asc,
                ::entity::dto::sort::SortDirection::Desc => Order::Desc,
            };
            // Only allow sorting by known columns for safety
            match sort_by.as_str() {
                "wfm_url" => stmt = stmt.order_by(transaction::Column::WfmUrl, order),
                "price" => stmt = stmt.order_by(transaction::Column::Price, order),
                "transaction_type" => {
                    stmt = stmt.order_by(transaction::Column::TransactionType, order)
                }
                "item_type" => stmt = stmt.order_by(transaction::Column::ItemType, order),
                _ => {}
            }
        }

        // Pagination
        let page = query.pagination.page.max(1);
        let limit = query.pagination.limit.max(1);
        let total;
        let results = if query.pagination.limit == -1 {
            total = stmt.clone().count(db).await? as i64;
            stmt.all(db).await?
        } else {
            let paginator = stmt.paginate(db, limit as u64);
            total = paginator.num_items().await? as i64;
            paginator.fetch_page((page - 1) as u64).await?
        };
        Ok(::entity::dto::pagination::PaginatedDto::new(
            total, limit, page, results,
        ))
    }
    pub async fn get_all(db: &DbConn) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find().all(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<transaction::Model>, DbErr> {
        Transaction::find_by_id(id).one(db).await
    }
}
