use ::entity::stock::item::stock_item_wat;
use ::entity::stock::item::{stock_item, stock_item::Entity as StockItem};
use ::entity::stock::item::{
    stock_item_old, stock_item_old::Entity as StockItemOld, stock_item_wat::Entity as StockItemWat,
};

use ::entity::sub_type::SubType;
use sea_orm::{sea_query::Expr, *};

pub struct StockItemQuery;

impl StockItemQuery {
    pub async fn find_all_transactions(db: &DbConn) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::find().all(db).await
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
