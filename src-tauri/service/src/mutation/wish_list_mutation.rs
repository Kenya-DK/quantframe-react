use ::entity::wish_list::{wish_list, wish_list::Entity as WishList};
use sea_orm::*;

pub struct WishListMutation;

impl WishListMutation {
    pub async fn create(
        db: &DbConn,
        form_data: &wish_list::Model,
    ) -> Result<wish_list::Model, DbErr> {
        wish_list::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            maximum_price: Set(form_data.maximum_price.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i64,
        form_data: wish_list::Model,
    ) -> Result<wish_list::Model, DbErr> {
        let post: wish_list::ActiveModel = WishList::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        wish_list::ActiveModel {
            id: post.id,
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: post.item_unique_name.clone(),
            sub_type: Set(form_data.sub_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            maximum_price: Set(form_data.maximum_price.to_owned()),
            price_history: post.price_history.clone(),
            status: post.status.clone(),
            list_price: post.list_price.clone(),
            created_at: post.created_at.clone(),
            updated_at: Set(chrono::Utc::now()),
        }
        .update(db)
        .await
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: wish_list::ActiveModel = WishList::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        WishList::delete_many().exec(db).await
    }
}
