use ::entity::{enums::*, syndicate_item::*};
use sea_orm::*;
use utils::*;

use crate::{ErrorFromExt, SyndicateItemQuery};

pub struct SyndicateItemMutation;

static COMPONENT: &str = "SyndicateItemMutation";

impl SyndicateItemMutation {
    pub async fn create(
        db: &DbConn,
        form_data: syndicate_item::Model,
    ) -> Result<syndicate_item::Model, Error> {
        syndicate_item::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            list_price: Set(form_data.list_price.to_owned()),
            syndicate_name: Set(form_data.syndicate_name.to_owned()),
            syndicate_unique_name: Set(form_data.syndicate_unique_name.to_owned()),
            standing_cost: Set(form_data.standing_cost.to_owned()),
            status: Set(form_data.status.to_owned()),
            price_history: Set(form_data.price_history.to_owned()),
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
                "Failed to create Syndicate Item",
                e,
                get_location!(),
            )
        })
    }

    pub async fn add_item(
        db: &DbConn,
        stock: syndicate_item::Model,
    ) -> Result<(String, syndicate_item::Model), Error> {
        // Find the item by id
        let item = SyndicateItemQuery::find_by_url_name_and_sub_type(
            db,
            &stock.wfm_url,
            stock.sub_type.clone(),
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;
        if item.is_none() {
            match SyndicateItemMutation::create(db, stock.clone()).await {
                Ok(insert) => {
                    return Ok(("Created".to_string(), insert));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        // Update the item
        let item = item.unwrap();
        match SyndicateItemMutation::update_by_id(
            db,
            UpdateSyndicateItem::new(item.id)
                .with_syndicate_name(stock.syndicate_name)
                .with_syndicate_unique_name(stock.syndicate_unique_name)
                .with_standing_cost(stock.standing_cost),
        )
        .await
        {
            Ok(up_item) => {
                return Ok(("Updated".to_string(), up_item));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateSyndicateItem,
    ) -> Result<syndicate_item::Model, Error> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateById", COMPONENT),
                    "Failed to find Syndicate Item by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:UpdateById", COMPONENT),
                "Syndicate Item not found",
                get_location!(),
            ))?;

        let mut active: syndicate_item::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateById", COMPONENT),
                "Failed to update Syndicate Item",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, Error> {
        let post: syndicate_item::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:DeleteById", COMPONENT),
                    "Failed to find Syndicate Item by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:DeleteById", COMPONENT),
                "Syndicate Item not found",
                get_location!(),
            ))?
            .into();

        post.delete(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteById", COMPONENT),
                "Failed to delete Syndicate Item",
                e,
                get_location!(),
            )
        })
    }

    pub async fn update_all(
        db: &DbConn,
        status: StockStatus,
        list_price: Option<i64>,
    ) -> Result<Vec<syndicate_item::Model>, Error> {
        Entity::update_many()
            .col_expr(syndicate_item::Column::Status, status.into())
            .col_expr(syndicate_item::Column::ListPrice, list_price.into())
            .exec(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateAll", COMPONENT),
                    "Failed to update all Syndicate Items",
                    e,
                    get_location!(),
                )
            })?;

        Entity::find().all(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateAll", COMPONENT),
                "Failed to retrieve all Syndicate Items after update",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, Error> {
        Entity::delete_many().exec(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteAll", COMPONENT),
                "Failed to delete all Syndicate Items",
                e,
                get_location!(),
            )
        })
    }
}
