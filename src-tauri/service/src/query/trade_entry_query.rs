use ::entity::trade_entry::*;

use sea_orm::{sea_query::Expr, *};

use crate::{paginate_query, ErrorFromExt};
use utils::*;

pub struct TradeEntryQuery;

static COMPONENT: &str = "TradeEntryQuery";
impl TradeEntryQuery {
    pub async fn get_all(
        db: &DbConn,
        query: TradeEntryPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<Model>, Error> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit)
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        Ok(paginated_result)
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<trade_entry::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindById", COMPONENT),
                "Failed to find Trade Entry by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<trade_entry::Model>, Error> {
        Entity::find()
            .filter(Expr::col(trade_entry::Column::Id).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByIds", COMPONENT),
                    "Failed to find Trade Entries by IDs",
                    e,
                    get_location!(),
                )
            })
    }
}
