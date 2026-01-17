use ::entity::setting::*;

use sea_orm::*;

use crate::ErrorFromExt;
use utils::*;

pub struct SettingQuery;

static COMPONENT: &str = "SettingQuery";
impl SettingQuery {
    pub async fn get(
        db: &DbConn,
        key: impl Into<String>,
        default: impl Into<String>,
    ) -> Result<String, Error> {
        let key = key.into();
        let default = default.into();
        let setting = Entity::find()
            .filter(setting::Column::Key.eq(key.clone()))
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:Get", COMPONENT),
                    "Failed to get Setting",
                    e,
                    get_location!(),
                )
            })?;
        if let Some(setting) = setting {
            Ok(setting.value)
        } else {
            Ok(default)
        }
    }
}
