use std::collections::HashMap;

use crate::{ErrorFromExt, WishListQuery};
use ::entity::{dto::*, wish_list::*};
use sea_orm::*;
use utils::*;

pub struct WishListMutation;

static COMPONENT: &str = "WishListMutation";

impl WishListMutation {
    pub async fn create(
        db: &DbConn,
        form_data: &wish_list::Model,
    ) -> Result<wish_list::Model, Error> {
        wish_list::ActiveModel {
            wfm_id: Set(form_data.wfm_id.to_owned()),
            wfm_url: Set(form_data.wfm_url.to_owned()),
            item_name: Set(form_data.item_name.to_owned()),
            item_unique_name: Set(form_data.item_unique_name.to_owned()),
            is_hidden: Set(form_data.is_hidden.to_owned()),
            sub_type: Set(form_data.sub_type.to_owned()),
            quantity: Set(form_data.quantity.to_owned()),
            maximum_price: Set(form_data.maximum_price.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| {
            Error::from_db(
                format!("{}:Create", COMPONENT),
                "Failed to create Wish List item",
                e,
                get_location!(),
            )
        })
    }
    pub async fn add_item(
        db: &DbConn,
        item: wish_list::Model,
    ) -> Result<(String, wish_list::Model), Error> {
        // Find the item by id
        let found_item =
            WishListQuery::find_by_url_name_and_sub_type(db, &item.wfm_url, item.sub_type.clone())
                .await
                .map_err(|e| e.with_location(get_location!()))?;
        if found_item.is_none() {
            match WishListMutation::create(db, &item.clone()).await {
                Ok(insert) => {
                    return Ok(("Created".to_string(), insert));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        let found_item = found_item.unwrap();
        let total = found_item.quantity + item.quantity;
        match WishListMutation::update_by_id(
            db,
            UpdateWishList::new(found_item.id).with_quantity(total),
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

    pub async fn bought_by_url_and_sub_type(
        db: &DbConn,
        url: &str,
        sub_type: Option<SubType>,
        quantity: i64,
    ) -> Result<(String, Option<wish_list::Model>), Error> {
        let item = WishListQuery::find_by_url_name_and_sub_type(db, url, sub_type)
            .await
            .map_err(|e| e.with_location(get_location!()))?;
        let id = match item {
            Some(i) => i.id,
            None => -1,
        };
        return WishListMutation::bought_by_id(db, id, quantity).await;
    }

    pub async fn bought_by_id(
        db: &DbConn,
        id: i64,
        mut quantity: i64,
    ) -> Result<(String, Option<wish_list::Model>), Error> {
        // Find the item by id
        let item = Entity::find_by_id(id).one(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:BoughtById", COMPONENT),
                "Failed to find Wish List item by ID",
                e,
                get_location!(),
            )
        })?;
        if item.is_none() {
            return Ok(("NotFound".to_string(), None));
        }

        // If quantity is 0, set it to 1
        if quantity == 0 {
            quantity = 1;
        }

        // Update the item
        let mut item = item.unwrap();
        item.quantity = item.quantity - quantity;
        if item.quantity <= 0 {
            match WishListMutation::delete_by_id(db, id).await {
                Ok(_) => {
                    return Ok(("Deleted".to_string(), Some(item)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            match WishListMutation::update_by_id(
                db,
                UpdateWishList::new(id).with_quantity(item.quantity),
            )
            .await
            {
                Ok(_) => {
                    return Ok(("Updated".to_string(), Some(item)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub async fn update_by_id(
        db: &DbConn,
        input: UpdateWishList,
    ) -> Result<wish_list::Model, Error> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateById", COMPONENT),
                    "Failed to find Wish List item by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:UpdateById", COMPONENT),
                "Wish List item not found",
                get_location!(),
            ))?;

        let mut active: wish_list::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateById", COMPONENT),
                "Failed to update Wish List item",
                e,
                get_location!(),
            )
        })
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, Error> {
        let post: wish_list::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                Error::from_db(
                    format!("{}:DeleteById", COMPONENT),
                    "Failed to find Wish List item by ID",
                    e,
                    get_location!(),
                )
            })?
            .ok_or(Error::new(
                format!("{}:DeleteById", COMPONENT),
                "Wish List item not found",
                get_location!(),
            ))?
            .into();

        post.delete(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:DeleteById", COMPONENT),
                "Failed to delete Wish List item",
                e,
                get_location!(),
            )
        })
    }
    pub async fn update_names(db: &DbConn, mapper: &HashMap<String, String>) -> Result<(), Error> {
        let items = Entity::find().all(db).await.map_err(|e| {
            Error::from_db(
                format!("{}:UpdateNames", COMPONENT),
                "Failed to retrieve all Wish List items",
                e,
                get_location!(),
            )
        })?;
        for item in items {
            let updated_name = match mapper.get(&item.item_unique_name) {
                Some(name) => name.to_string(),
                None => continue,
            };
            let mut active: wish_list::ActiveModel = item.into();
            active.item_name = Set(updated_name);
            active.updated_at = Set(chrono::Utc::now());
            active.update(db).await.map_err(|e| {
                Error::from_db(
                    format!("{}:UpdateNames", COMPONENT),
                    "Failed to update Wish List item name",
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
                "Failed to delete all Wish List items",
                e,
                get_location!(),
            )
        })
    }
}
