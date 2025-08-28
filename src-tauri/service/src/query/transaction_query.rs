use ::entity::transaction::dto::TransactionPaginationQueryDto;
use ::entity::transaction::{transaction, transaction::Entity as Transaction};
use sea_orm::*;

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_all(
        db: &DbConn,
        query: TransactionPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<transaction::Model>, DbErr> {
        let stmt = query.get_query();

        // // Print the generated SQL for debugging
        // println!(
        //     "Generating SQL for StockItemQuery::get_all_v2: {}",
        //     stmt.clone().build(db.get_database_backend()).to_string()
        // );

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
        Ok(::entity::dto::pagination::PaginatedResult::new(
            total, limit, page, results,
        ))
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<transaction::Model>, DbErr> {
        Transaction::find_by_id(id).one(db).await
    }
}
