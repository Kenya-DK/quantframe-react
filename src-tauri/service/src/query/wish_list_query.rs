use ::entity::wish_list::*;

use ::entity::dto::SubType;
use sea_orm::sea_query::Func;
use sea_orm::{sea_query::Expr, *};

pub struct WishListQuery;

impl WishListQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<wish_list::Model>, DbErr> {
        Entity::find().all(db).await
    }
    pub async fn get_all(
        db: &DbConn,
        query: WishListPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedDto<wish_list::Model>, DbErr> {
        let stmt = query.get_query();

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
