use ::entity::stock::riven::{stock_riven, stock_riven::Entity as StockRiven};
use sea_orm::*;

pub struct StockRivenMutation;

impl StockRivenMutation {
    pub async fn create_from_old(
        db: &DbConn,
        form_data: stock_riven::Model,
    ) -> Result<stock_riven::ActiveModel, DbErr> {
        stock_riven::ActiveModel {
            wfm_weapon_id: Set(form_data.wfm_weapon_id.to_owned()),
            wfm_order_id: Set(form_data.wfm_order_id.to_owned()),
            wfm_weapon_url: Set(form_data.wfm_weapon_url.to_owned()),
            weapon_name: Set(form_data.weapon_name.to_owned()),
            weapon_type: Set(form_data.weapon_type.to_owned()),
            weapon_unique_name: Set(form_data.weapon_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            mod_name: Set(form_data.mod_name.to_owned()),
            attributes: Set(form_data.attributes.to_owned()),
            mastery_rank: Set(form_data.mastery_rank.to_owned()),
            re_rolls: Set(form_data.re_rolls.to_owned()),
            polarity: Set(form_data.polarity.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            filter: Set(form_data.filter.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            comment: Set(form_data.comment.to_owned()),
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
        form_data: stock_riven::Model,
    ) -> Result<stock_riven::Model, DbErr> {
        stock_riven::ActiveModel {
            wfm_weapon_id: Set(form_data.wfm_weapon_id.to_owned()),
            wfm_weapon_url: Set(form_data.wfm_weapon_url.to_owned()),
            weapon_name: Set(form_data.weapon_name.to_owned()),
            weapon_type: Set(form_data.weapon_type.to_owned()),
            weapon_unique_name: Set(form_data.weapon_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            mod_name: Set(form_data.mod_name.to_owned()),
            attributes: Set(form_data.attributes.to_owned()),
            mastery_rank: Set(form_data.mastery_rank.to_owned()),
            re_rolls: Set(form_data.re_rolls.to_owned()),
            polarity: Set(form_data.polarity.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            filter: Set(form_data.filter.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            comment: Set(form_data.comment.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
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
        form_data: stock_riven::Model,
    ) -> Result<stock_riven::Model, DbErr> {
        let post: stock_riven::ActiveModel = StockRiven::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        stock_riven::ActiveModel {
            id: post.id,
            wfm_weapon_id: Set(form_data.wfm_weapon_id.to_owned()),
            wfm_weapon_url: Set(form_data.wfm_weapon_url.to_owned()),
            wfm_order_id: Set(form_data.wfm_order_id.to_owned()),
            weapon_name: Set(form_data.weapon_name.to_owned()),
            weapon_type: Set(form_data.weapon_type.to_owned()),
            weapon_unique_name: Set(form_data.weapon_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            mod_name: Set(form_data.mod_name.to_owned()),
            attributes: Set(form_data.attributes.to_owned()),
            mastery_rank: Set(form_data.mastery_rank.to_owned()),
            re_rolls: Set(form_data.re_rolls.to_owned()),
            polarity: Set(form_data.polarity.to_owned()),
            bought: Set(form_data.bought.to_owned()),
            minimum_price: Set(form_data.minimum_price.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            filter: Set(form_data.filter.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            comment: Set(form_data.comment.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
            created_at: post.created_at,
            updated_at: Set(chrono::Utc::now()),
        }
        .update(db)
        .await
    }
    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_riven::Model>, DbErr> {
        StockRiven::find_by_id(id).one(db).await
    }

    pub async fn delete(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: stock_riven::ActiveModel = StockRiven::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        StockRiven::delete_many().exec(db).await
    }
}
