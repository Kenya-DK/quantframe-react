use ::entity::wish_list::dto::WishListPaginationQueryDto;
use ::entity::wish_list::{wish_list, wish_list::Entity as WishList};

use ::entity::sub_type::SubType;
use sea_orm::sea_query::Func;
use sea_orm::{sea_query::Expr, *};

pub struct WishListQuery;

impl WishListQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<wish_list::Model>, DbErr> {
        WishList::find().all(db).await
    }
    pub async fn get_all_v2(
        db: &DbConn,
        query: WishListPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedDto<wish_list::Model>, DbErr> {
        let mut stmt = WishList::find();

        // Filtering by query (search)
        if let Some(ref q) = query.query {
            // Case-sensitive search in WfmUrl and ItemName columns
            stmt = stmt.filter(
                Condition::any()
                    .add(
                        Expr::expr(Func::lower(Expr::col(wish_list::Column::WfmUrl)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col(wish_list::Column::ItemName)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    ),
            );
        }
        // Filtering by status
        if let Some(ref status) = query.status {
            stmt = stmt.filter(wish_list::Column::Status.eq(status));
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
                "item_name" => stmt = stmt.order_by(wish_list::Column::ItemName, order),
                "status" => stmt = stmt.order_by(wish_list::Column::Status, order),
                "maximum_price" => stmt = stmt.order_by(wish_list::Column::MaximumPrice, order),
                "list_price" => stmt = stmt.order_by(wish_list::Column::ListPrice, order),
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
    pub async fn get_all(db: &DbConn) -> Result<Vec<wish_list::Model>, DbErr> {
        WishList::find().all(db).await
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, DbErr> {
        WishList::find_by_id(id).one(db).await
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<wish_list::Model>, DbErr> {
        WishList::find()
            .filter(wish_list::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<wish_list::Model>, DbErr> {
        WishList::find_by_id(id).one(db).await
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<wish_list::Model>, DbErr> {
        WishList::find()
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
