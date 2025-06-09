use ::entity::enums::stock_status::StockStatus;
use ::entity::stock::riven::dto::StockRivenPaginationQueryDto;
use ::entity::stock::riven::{stock_riven, stock_riven::Entity as StockRiven};

use ::entity::sub_type::SubType;
use sea_orm::sea_query::Func;
use sea_orm::*;
use sea_query::Expr;

pub struct StockRivenQuery;

impl StockRivenQuery {
    pub async fn get_all_v2(
        db: &DbConn,
        query: StockRivenPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedDto<stock_riven::Model>, DbErr> {
        let mut stmt = StockRiven::find();

        // Filtering by query (search)
        if let Some(ref q) = query.query {
            // Case-sensitive search in WfmUrl and ItemName columns
            stmt = stmt.filter(
                Condition::any()
                    .add(
                        Expr::expr(Func::lower(Expr::col(stock_riven::Column::WeaponName)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col(stock_riven::Column::WfmWeaponUrl)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col(stock_riven::Column::ModName)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    ),
            );
        }
        // Filtering by status
        if let Some(ref status) = query.status {
            stmt = stmt.filter(stock_riven::Column::Status.eq(status));
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
                "item_name" => stmt = stmt.order_by(stock_riven::Column::WeaponName, order),
                "bought" => stmt = stmt.order_by(stock_riven::Column::Bought, order),
                "status" => stmt = stmt.order_by(stock_riven::Column::Status, order),
                "minimum_price" => stmt = stmt.order_by(stock_riven::Column::MinimumPrice, order),
                "list_price" => stmt = stmt.order_by(stock_riven::Column::ListPrice, order),
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
    pub async fn get_all(db: &DbConn) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::find().all(db).await
    }

    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i64>, DbErr> {
        let items = StockRivenQuery::get_all(db).await?;
        let res = items.iter().map(|x| x.id).collect();
        Ok(res)
    }

    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<stock_riven::Model>, DbErr> {
        StockRiven::find_by_id(id).one(db).await
    }
    pub async fn get_by_order_id(
        db: &DbConn,
        order_id: &str,
    ) -> Result<Option<stock_riven::Model>, DbErr> {
        StockRiven::find()
            .filter(stock_riven::Column::WfmOrderId.eq(order_id))
            .one(db)
            .await
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::find()
            .filter(Expr::col(stock_riven::Column::Id).is_in(ids))
            .all(db)
            .await
    }
    pub async fn clear_all_order_id(db: &DbConn) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::update_many()
            .col_expr(
                stock_riven::Column::WfmOrderId,
                Expr::value(Option::<String>::None),
            )
            .col_expr(
                stock_riven::Column::Status,
                Expr::value(StockStatus::Pending),
            )
            .col_expr(
                stock_riven::Column::ListPrice,
                Expr::value(Option::<i64>::None),
            )
            .exec(db)
            .await?;
        StockRivenQuery::get_all(db).await
    }

    pub async fn get_by_riven_name(
        db: &DbConn,
        weapon_url: &str,
        mod_name: &str,
        sub_type: SubType,
    ) -> Result<Option<stock_riven::Model>, DbErr> {
        StockRiven::find()
            .filter(stock_riven::Column::WfmWeaponUrl.eq(weapon_url))
            .filter(stock_riven::Column::ModName.eq(mod_name))
            .filter(stock_riven::Column::SubType.eq(sub_type))
            .one(db)
            .await
    }

    pub async fn update_bulk(
        db: &DbConn,
        ids: Vec<i64>,
        minimum_price: Option<i64>,
        is_hidden: Option<bool>,
    ) -> Result<Vec<stock_riven::Model>, DbErr> {
        let mut query = StockRiven::update_many();

        if let Some(minimum_price) = minimum_price {
            query = query.col_expr(stock_riven::Column::MinimumPrice, minimum_price.into());
        }
        if let Some(is_hidden) = is_hidden {
            query = query.col_expr(stock_riven::Column::IsHidden, is_hidden.into());
        }
        query = query.filter(Expr::col(stock_riven::Column::Id).is_in(ids));

        query.exec(db).await?;
        StockRiven::find().all(db).await
    }
}
