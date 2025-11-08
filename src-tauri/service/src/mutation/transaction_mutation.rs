use crate::ErrorFromExt;
use ::entity::transaction::*;
use sea_orm::*;
use utils::*;
pub struct TransactionMutation;

static COMPONENT: &str = "TransactionMutation";
impl TransactionMutation {
    pub async fn create(
        db: &DbConn,
        form_data: &transaction::Model,
    ) -> Result<transaction::Model, Error> {
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
            profit: Set(form_data.profit.to_owned()),
            properties: Set(form_data.properties.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| {
            Error::from_db(
                format!("{}:Create", COMPONENT),
                "Failed to create Transaction",
                e,
                get_location!(),
            )
        })
    }

    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateTransaction,
    ) -> Result<transaction::Model, Error> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateById", COMPONENT),
                    "Failed to find Transaction by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:UpdateById", COMPONENT),
                "Transaction not found",
                get_location!(),
            ))?;

        let mut active: transaction::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateById", COMPONENT),
                "Failed to update Transaction",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, Error> {
        let post: transaction::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:DeleteById", COMPONENT),
                    "Failed to find Transaction by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:DeleteById", COMPONENT),
                "Transaction not found",
                get_location!(),
            ))?
            .into();

        post.delete(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteById", COMPONENT),
                "Failed to delete Transaction",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, Error> {
        Entity::delete_many().exec(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteAll", COMPONENT),
                "Failed to delete all Transactions",
                e,
                get_location!(),
            )
        })
    }
}
