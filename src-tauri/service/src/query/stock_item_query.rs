use ::entity::stock_item::*;

use ::entity::dto::SubType;
use sea_orm::{sea_query::Expr, *};

use crate::{paginate_query, ErrorFromExt};
use utils::*;

pub struct StockItemQuery;

static COMPONENT: &str = "StockItemQuery";
impl StockItemQuery {
    pub async fn get_all(
        db: &DbConn,
        query: StockItemPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<Model>, Error> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit)
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        Ok(paginated_result)
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<stock_item::Model>, Error> {
        Entity::find()
            .filter(stock_item::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByUrlName", COMPONENT),
                    "Failed to find Stock Items by URL name",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_item::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindById", COMPONENT),
                "Failed to find Stock Item by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<stock_item::Model>, Error> {
        Entity::find()
            .filter(Expr::col(stock_item::Column::Id).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByIds", COMPONENT),
                    "Failed to find Stock Items by IDs",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<stock_item::Model>, Error> {
        let items = StockItemQuery::find_by_url_name(db, url_name).await?;
        for item in items {
            if item.sub_type == sub_type {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}
