use std::collections::HashMap;

use ::entity::stock_riven::*;
use sea_orm::*;
use utils::*;

use crate::{ErrorFromExt, StockRivenQuery};

pub struct StockRivenMutation;

static COMPONENT: &str = "StockRivenMutation";
impl StockRivenMutation {
    pub async fn create(
        db: &DbConn,
        form_data: stock_riven::Model,
    ) -> Result<(String, stock_riven::Model), Error> {
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
        .await
        .map_err(|e| {
            Error::from_db(
                format!("{}:Create", COMPONENT),
                "Failed to create Stock Riven",
                e,
                get_location!(),
            )
        })?;

        Ok(("Create".to_string(), model))
    }
    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateStockRiven,
    ) -> Result<stock_riven::Model, Error> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateById", COMPONENT),
                    "Failed to find Stock Riven by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:UpdateById", COMPONENT),
                "Stock Riven not found",
                get_location!(),
            ))?;

        let mut active: stock_riven::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateById", COMPONENT),
                "Failed to update Stock Riven",
                e,
                get_location!(),
            )
        })
    }
    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<stock_riven::Model>, Error> {
        Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:FindById", COMPONENT),
                "Failed to find Stock Riven by ID",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete(db: &DbConn, id: i64) -> Result<DeleteResult, Error> {
        let post: stock_riven::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:DeleteById", COMPONENT),
                    "Failed to find Stock Riven by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:DeleteById", COMPONENT),
                "Stock Riven not found",
                get_location!(),
            ))?
            .into();

        post.delete(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteById", COMPONENT),
                "Failed to delete Stock Riven",
                e,
                get_location!(),
            )
        })
    }
    pub async fn delete_uuid(db: &DbConn, uuid: impl Into<String>) -> Result<DeleteResult, Error> {
        let entry = StockRivenQuery::get_by_uuid(db, uuid)
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        if let Some(entry) = entry {
            StockRivenMutation::delete(db, entry.id).await
        } else {
            Err(Error::new(
                format!("{}:DeleteByUUID", COMPONENT),
                "Stock Riven not found for given UUID",
                get_location!(),
            ))
        }
    }
    pub async fn update_names(db: &DbConn, mapper: &HashMap<String, String>) -> Result<(), Error> {
        let items = Entity::find().all(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateNames", COMPONENT),
                "Failed to retrieve all Stock Rivens",
                e,
                get_location!(),
            )
        })?;
        for item in items {
            let updated_name = match mapper.get(&item.weapon_unique_name) {
                Some(name) => name.to_string(),
                None => continue,
            };
            let mut active: stock_riven::ActiveModel = item.into();
            active.weapon_name = Set(updated_name);
            active.updated_at = Set(chrono::Utc::now());
            active.update(db).await.map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateNames", COMPONENT),
                    "Failed to update Stock Riven name",
                    e,
                    get_location!(),
                )
            })?;
        }

        Ok(())
    }
    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, Error> {
        Entity::delete_many().exec(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteAll", COMPONENT),
                "Failed to delete all Stock Rivens",
                e,
                get_location!(),
            )
        })
    }
}
