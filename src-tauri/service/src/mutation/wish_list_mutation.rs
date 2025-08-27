use ::entity::{dto::*, wish_list::*};
use sea_orm::*;

use crate::WishListQuery;

pub struct WishListMutation;

impl WishListMutation {
    pub async fn create(
        db: &DbConn,
        form_data: &wish_list::Model,
    ) -> Result<wish_list::Model, DbErr> {
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
    }
    pub async fn add_item(
        db: &DbConn,
        item: wish_list::Model,
    ) -> Result<(String, wish_list::Model), DbErr> {
        // Find the item by id
        let found_item =
            WishListQuery::find_by_url_name_and_sub_type(db, &item.wfm_url, item.sub_type.clone())
                .await?;
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
    ) -> Result<(String, Option<wish_list::Model>), DbErr> {
        let item = WishListQuery::find_by_url_name_and_sub_type(db, url, sub_type).await?;
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
    ) -> Result<(String, Option<wish_list::Model>), DbErr> {
        // Find the item by id
        let item = Entity::find_by_id(id).one(db).await?;
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
    ) -> Result<wish_list::Model, DbErr> {
        let item = Entity::find_by_id(input.id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find wish list item.".to_owned()))?;

        let mut active: wish_list::ActiveModel = input.apply_to(item.into());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await
    }

    pub async fn delete_by_id(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: wish_list::ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Entity::delete_many().exec(db).await
    }
}
