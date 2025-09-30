use ::entity::wish_list::*;

use crate::paginate_query;
use ::entity::dto::SubType;
use sea_orm::{sea_query::Expr, *};
pub struct WishListQuery;

impl WishListQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<wish_list::Model>, DbErr> {
        Entity::find().all(db).await
    }
    pub async fn get_all(
        db: &DbConn,
        query: WishListPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<wish_list::Model>, DbErr> {
        let stmt = query.get_query();

        // Pagination
        let paginated_result =
            paginate_query(stmt, db, query.pagination.page, query.pagination.limit).await?;
        Ok(paginated_result)
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<wish_list::Model>, DbErr> {
        Entity::find()
            .filter(wish_list::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<wish_list::Model>, DbErr> {
        Entity::find()
            .filter(Expr::col(wish_list::Column::Id).is_in(ids))
            .all(db)
            .await
    }

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<wish_list::Model>, DbErr> {
        let items = WishListQuery::find_by_url_name(db, url_name).await?;
        for item in items {
            if item.sub_type == sub_type {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}
