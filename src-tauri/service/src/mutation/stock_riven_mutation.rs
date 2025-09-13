use ::entity::{enums, stock_riven::*};
use prelude::Expr;
use sea_orm::*;

use crate::StockRivenQuery;

pub struct StockRivenMutation;

impl StockRivenMutation {
    pub async fn create(
        db: &DbConn,
        form_data: stock_riven::Model,
    ) -> Result<(String, stock_riven::Model), DbErr> {
        let model = stock_riven::ActiveModel {
            wfm_weapon_id: Set(form_data.wfm_weapon_id.to_owned()),
            wfm_weapon_url: Set(form_data.wfm_weapon_url.to_owned()),
            uuid: Set(form_data.uuid().to_string()),
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
        .await?;

        Ok(("Create".to_string(), model))
    }
    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateStockRiven,
    ) -> Result<stock_riven::Model, DbErr> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("NotFound".to_owned()))?;

        let mut active: stock_riven::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await
    }
    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_riven::Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn delete(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: stock_riven::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("NotFound".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }
    pub async fn delete_uuid(db: &DbConn, uuid: impl Into<String>) -> Result<DeleteResult, DbErr> {
        let entry = StockRivenQuery::get_by_uuid(db, uuid).await?;
        if let Some(entry) = entry {
            StockRivenMutation::delete(db, entry.id).await
        } else {
            Err(DbErr::Custom("NotFound".to_string()))
        }
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Entity::delete_many().exec(db).await
    }
}
