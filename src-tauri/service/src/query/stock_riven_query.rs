use ::entity::enums::stock_status::StockStatus;
use ::entity::stock::riven::{stock_riven, stock_riven::Entity as StockRiven};
use ::entity::stock::riven::{stock_riven_old, stock_riven_old::Entity as StockRivenOld};

use sea_orm::*;
use sea_query::Expr;

pub struct StockRivenQuery;

impl StockRivenQuery {
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

    pub async fn clear_all_order_id(db: &DbConn) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::update_many()
            .col_expr(stock_riven::Column::WfmOrderId, Expr::value(Option::<String>::None))
            .col_expr(stock_riven::Column::Status, Expr::value(StockStatus::Pending))
            .exec(db)
            .await?;
        StockRivenQuery::get_all(db).await
    }

    pub async fn get_old_stock_riven(db: &DbConn) -> Result<Vec<stock_riven_old::Model>, DbErr> {
        StockRivenOld::find().all(db).await
    }
}
