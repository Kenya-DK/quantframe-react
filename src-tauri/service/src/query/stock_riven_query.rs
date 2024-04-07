use ::entity::{stock_riven, stock_riven::Entity as StockRiven};

use sea_orm::{sea_query::Expr, *};

pub struct StockRivenQuery;

impl StockRivenQuery {

    pub async fn get_all(db: &DbConn) -> Result<Vec<stock_riven::Model>, DbErr> {
        StockRiven::find().all(db).await
    }
}
