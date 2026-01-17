use ::entity::setting::*;
use sea_orm::*;
use utils::*;

use crate::ErrorFromExt;

pub struct SettingMutation;

static COMPONENT: &str = "SettingMutation";

impl SettingMutation {
    pub async fn update_create(
        db: &DbConn,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<setting::Model, Error> {
        let key = key.into();
        let value = value.into();
        let now = chrono::Utc::now();

        let existing = Entity::find()
            .filter(setting::Column::Key.eq(&key))
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateCreate", COMPONENT),
                    "Failed to query Setting",
                    e,
                    get_location!(),
                )
            })?;

        // Update
        if let Some(existing) = existing {
            let mut active: setting::ActiveModel = existing.into();
            active.value = Set(value);
            active.updated_at = Set(now);

            return active.update(db).await.map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateCreate", COMPONENT),
                    "Failed to update Setting",
                    e,
                    get_location!(),
                )
            });
        }

        // Create
        setting::ActiveModel {
            key: Set(key),
            value: Set(value),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| {
            Error::from_db(
                format!("{}:UpdateCreate", COMPONENT),
                "Failed to create Setting",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, Error> {
        Entity::delete_many().exec(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteAll", COMPONENT),
                "Failed to delete all Stock Items",
                e,
                get_location!(),
            )
        })
    }
}
