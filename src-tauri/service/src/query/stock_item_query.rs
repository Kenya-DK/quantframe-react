use ::entity::stock_item::*;

use ::entity::dto::SubType;
use sea_orm::{sea_query::Expr, *};

pub struct StockItemQuery;

impl StockItemQuery {
    pub async fn get_all(
        db: &DbConn,
        query: StockItemPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedResult<Model>, DbErr> {
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

        Ok(::entity::dto::pagination::PaginatedResult::new(
            total, limit, page, results,
        ))
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<stock_item::Model>, DbErr> {
        Entity::find()
            .filter(stock_item::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_item::Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<stock_item::Model>, DbErr> {
        Entity::find()
            .filter(Expr::col(stock_item::Column::Id).is_in(ids))
            .all(db)
            .await
    }

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<stock_item::Model>, DbErr> {
        let items = StockItemQuery::find_by_url_name(db, url_name).await?;
        for item in items {
            if item.sub_type == sub_type {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}
