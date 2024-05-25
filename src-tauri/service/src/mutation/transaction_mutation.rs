use ::entity::transaction::{transaction, transaction::Entity as Transaction};
use sea_orm::*;

pub struct TransactionMutation;

impl TransactionMutation {
    pub async fn create_from_old(
        db: &DbConn,
        form_data: transaction::Model,
    ) -> Result<transaction::ActiveModel, DbErr> {
        transaction::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_type: Set(form_data.item_type.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            tags: Set(form_data.tags.to_owned()),
            transaction_type: Set(form_data.transaction_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            user_name: Set(form_data.user_name.to_owned()),
            price: Set(form_data.price.to_owned()),
            properties: Set(form_data.properties.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
    pub async fn create(
        db: &DbConn,
        form_data: transaction::Model,
    ) -> Result<transaction::Model, DbErr> {
        transaction::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_type: Set(form_data.item_type.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            tags: Set(form_data.tags.to_owned()),
            transaction_type: Set(form_data.transaction_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            user_name: Set(form_data.user_name.to_owned()),
            price: Set(form_data.price.to_owned()),
            properties: Set(form_data.properties.to_owned()),
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
        form_data: transaction::Model,
    ) -> Result<transaction::Model, DbErr> {
        let post: transaction::ActiveModel = Transaction::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        transaction::ActiveModel {
            id: post.id,
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_type: Set(form_data.item_type.to_owned()),
            item_unique_name: post.item_unique_name.clone(),
            sub_type: Set(form_data.sub_type.to_owned()),
            tags: Set(form_data.tags.to_owned()),
            transaction_type: Set(form_data.transaction_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            user_name: post.user_name.clone(),
            price: Set(form_data.price.to_owned()),
            properties: Set(form_data.properties.to_owned()),
            created_at: post.created_at.clone(),
            updated_at: Set(chrono::Utc::now()),
        }
        .update(db)
        .await
    }

    
    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: transaction::ActiveModel = Transaction::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Transaction::delete_many().exec(db).await
    }
}
