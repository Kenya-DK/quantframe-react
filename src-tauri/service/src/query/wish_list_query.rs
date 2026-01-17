use ::entity::wish_list::*;

use crate::{paginate_query, ErrorFromExt};
use ::entity::dto::SubType;
use sea_orm::{sea_query::Expr, *};
use utils::*;
pub struct WishListQuery;

static COMPONENT: &str = "WishListQuery";
impl WishListQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<wish_list::Model>, Error> {
        Entity::find().all(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindAllTransactions", COMPONENT),
                "Failed to find all transactions",
                e,
                get_location!(),
            )
        })
    }
    pub async fn get_all(
        db: &DbConn,
        query: WishListPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<wish_list::Model>, Error> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit)
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        Ok(paginated_result)
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:GetById", COMPONENT),
                "Failed to get Wish List item by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<wish_list::Model>, Error> {
        Entity::find()
            .filter(wish_list::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByUrlName", COMPONENT),
                    "Failed to find Wish List items by URL name",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindById", COMPONENT),
                "Failed to find Wish List item by ID",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<wish_list::Model>, Error> {
        Entity::find()
            .filter(Expr::col(wish_list::Column::Id).is_in(ids))
            .all(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:FindByIds", COMPONENT),
                    "Failed to find Wish List items by IDs",
                    e,
                    get_location!(),
                )
            })
    }

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<wish_list::Model>, Error> {
        let items = WishListQuery::find_by_url_name(db, url_name)
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        for item in items {
            if item.sub_type == sub_type {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}
