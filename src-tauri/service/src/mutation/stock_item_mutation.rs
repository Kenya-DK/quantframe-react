use ::entity::{
    enums::stock_status::StockStatus,
    stock::item::stock_item::{self, Entity as StockItem},
    sub_type::SubType,
};
use sea_orm::*;

pub struct StockItemMutation;

impl StockItemMutation {
    pub async fn create_from_old(
        db: &DbConn,
        form_data: stock_item::Model,
    ) -> Result<stock_item::ActiveModel, DbErr> {
        stock_item::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            owned: Set(form_data.owned.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
    pub async fn create(
        db: &DbConn,
        form_data: stock_item::Model,
    ) -> Result<stock_item::Model, DbErr> {
        stock_item::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            owned: Set(form_data.owned.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
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

    pub async fn find_by_url_name_and_sub_type(
        db: &DbConn,
        url_name: &str,
        sub_type: Option<SubType>,
    ) -> Result<Option<stock_item::Model>, DbErr> {
        let items = StockItemMutation::find_by_url_name(db, url_name).await?;
        for item in items {
            if item.sub_type == sub_type {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i64,
        form_data: stock_item::Model,
    ) -> Result<stock_item::Model, DbErr> {
        let post: stock_item::ActiveModel = StockItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        stock_item::ActiveModel {
            id: post.id,
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            owned: Set(form_data.owned.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
            created_at: post.created_at.clone(),
            updated_at: Set(chrono::Utc::now()),
        }
        .update(db)
        .await
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: stock_item::ActiveModel = StockItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find Item.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn update_all(
        db: &DbConn,
        status: StockStatus,
        list_price: Option<i64>,
    ) -> Result<Vec<stock_item::Model>, DbErr> {
        StockItem::update_many()
            .col_expr(stock_item::Column::Status, status.into())
            .col_expr(stock_item::Column::ListPrice, list_price.into())
            .exec(db)
            .await?;

        StockItem::find().all(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        StockItem::delete_many().exec(db).await
    }
}
