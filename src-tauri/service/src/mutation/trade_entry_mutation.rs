use crate::ErrorFromExt;
use ::entity::trade_entry::*;
use sea_orm::*;
use utils::*;
pub struct TradeEntryMutation;

static COMPONENT: &str = "TradeEntryMutation";
impl TradeEntryMutation {
    pub async fn create(
        db: &DbConn,
        form_data: &trade_entry::Model,
    ) -> Result<trade_entry::Model, Error> {
        trade_entry::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            name: Set(form_data.name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            price: Set(form_data.price),
            group: Set(form_data.group.to_owned()),
            properties: Set(form_data.properties.to_owned()),
            tags: Set(form_data.tags.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| {
            Error::from_db(
                format!("{}:Create", COMPONENT),
                "Failed to create Trade Entry",
                e,
                get_location!(),
            )
        })
    }

    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateTradeEntry,
    ) -> Result<trade_entry::Model, Error> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateById", COMPONENT),
                    "Failed to find Trade Entry by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:UpdateById", COMPONENT),
                "Trade Entry not found",
                get_location!(),
            ))?;

        let mut active: trade_entry::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateById", COMPONENT),
                "Failed to update Trade Entry",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, Error> {
        let post: trade_entry::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:DeleteById", COMPONENT),
                    "Failed to find Trade Entry by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:DeleteById", COMPONENT),
                "Trade Entry not found",
                get_location!(),
            ))?
            .into();

        post.delete(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteById", COMPONENT),
                "Failed to delete Trade Entry",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, Error> {
        Entity::delete_many().exec(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteAll", COMPONENT),
                "Failed to delete all Trade Entries",
                e,
                get_location!(),
            )
        })
    }
}
