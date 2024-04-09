use ::entity::{stock_riven, stock_riven::Entity as StockRiven};
use ::entity::{stock_riven_old, stock_riven_old::Entity as StockRivenOld};

use sea_orm::{sea_query::Expr, *};

pub struct StockRivenQuery;

impl StockRivenQuery {

    pub async fn get_all(db: &DbConn) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::find().all(db).await
    }
    pub async fn get_old_stock_riven(db: &DbConn) -> Result<Vec<stock_riven_old::Model>, DbErr> {
        StockRivenOld::find().all(db).await
    }
}
