use ::entity::stock::item::dto::*;
use ::entity::stock::item::stock_item_wat;
use ::entity::stock::item::{stock_item, stock_item::Entity as StockItem};
use ::entity::stock::item::{
    stock_item_old, stock_item_old::Entity as StockItemOld, stock_item_wat::Entity as StockItemWat,
};

use ::entity::sub_type::SubType;
use sea_orm::sea_query::Func;
use sea_orm::{sea_query::Expr, *};

pub struct StockItemQuery;

impl StockItemQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find().all(db).await
    }

    pub async fn get_all_v2(
        db: &DbConn,
        query: StockItemPaginationQueryDto,
    ) -> Result<::entity::dto::pagination::PaginatedDto<stock_item::Model>, DbErr> {
        let mut stmt = StockItem::find();

        // Filtering by query (search)
        if let Some(ref q) = query.query {
            // Case-sensitive search in WfmUrl and ItemName columns
            stmt = stmt.filter(
                Condition::any()
                    .add(
                        Expr::expr(Func::lower(Expr::col(stock_item::Column::WfmUrl)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col(stock_item::Column::ItemName)))
                            .like(&format!("%{}%", q.to_lowercase())),
                    ),
            );
        }
        // Filtering by status
        if let Some(ref status) = query.status {
            stmt = stmt.filter(stock_item::Column::Status.eq(status));
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
                "item_name" => stmt = stmt.order_by(stock_item::Column::ItemName, order),
                "bought" => stmt = stmt.order_by(stock_item::Column::Bought, order),
                "status" => stmt = stmt.order_by(stock_item::Column::Status, order),
                "minimum_price" => stmt = stmt.order_by(stock_item::Column::MinimumPrice, order),
                "list_price" => stmt = stmt.order_by(stock_item::Column::ListPrice, order),
                "owned" => stmt = stmt.order_by(stock_item::Column::Owned, order),
                _ => {}
            }
        }

        // Print the generated SQL for debugging
        println!(
            "Generating SQL for StockItemQuery::get_all_v2: {:?}",
            stmt.clone().build(db.get_database_backend()).to_string()
        );

        // Pagination
        let page = query.pagination.page.max(1);
        let limit = query.pagination.limit.max(1);
        let paginator = stmt.paginate(db, limit as u64);
        let total = paginator.num_items().await? as i64;
        let results = if query.pagination.limit == -1 {
            StockItem::find().all(db).await?
        } else {
            paginator.fetch_page((page - 1) as u64).await?
        };
        Ok(::entity::dto::pagination::PaginatedDto::new(
            total, limit, page, results,
        ))
    }
    pub async fn get_all(db: &DbConn) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find().all(db).await
    }

    pub async fn get_all_stock_items(
        db: &DbConn,
        minimum_owned: i32,
    ) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find()
            .filter(Expr::col(stock_item::Column::Owned).gt(minimum_owned))
            .all(db)
            .await
    }
    pub async fn get_by_id(db: &DbConn, id: i64) -> Result<Option<stock_item::Model>, DbErr> {
        StockItem::find_by_id(id).one(db).await
    }
    pub async fn find_by_url_name(
        db: &DbConn,
        url_name: &str,
    ) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find()
            .filter(stock_item::Column::WfmUrl.contains(url_name))
            .all(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_item::Model>, DbErr> {
        StockItem::find_by_id(id).one(db).await
    }
    pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find()
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
    pub async fn get_old_stock_items(db: &DbConn) -> Result<Vec<stock_item_old::Model>, DbErr> {
        StockItemOld::find().all(db).await
    }
    pub async fn get_wat_stock_items(db: &DbConn) -> Result<Vec<stock_item_wat::Model>, DbErr> {
        StockItemWat::find().all(db).await
    }
}
