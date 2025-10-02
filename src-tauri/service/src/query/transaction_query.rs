use crate::paginate_query;
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

        // Print the generated SQL for debugging
        // println!(
        //     "Generating SQL for StockItemQuery::get_all_v2: {}",
        //     stmt.clone().build(db.get_database_backend()).to_string()
        // );

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit).await?;
        Ok(paginated_result)
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<transaction::Model>, DbErr> {
        Transaction::find_by_id(id).one(db).await
    }
}
