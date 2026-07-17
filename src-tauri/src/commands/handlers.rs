use crate::{commands::item, handlers::*};
use utils::{get_location, Error, OperationSet};

#[tauri::command]
pub async fn handles_handle_items(items: Vec<ItemEntity>) -> Result<i32, Error> {
    let mut total = 0;
    let mut processed_items = Vec::new();
    // WishList
    let mut iter = items.into_iter();
    while let Some(item) = iter.next() {
        let item = item.clone();
        if item.operations.has("WishList") {
            let (o, updated_item) = handle_wish_list_by_entity(
                item.clone().into(),
                item.user_name.clone(),
                item.order_type.clone(),
                &item.operations,
            )
            .await
            .map_err(|e| e.with_location(get_location!()))?;

            total += 1;
            processed_items.push((o, updated_item.item_name));
        } else {
            let (o, updated_item) = handle_item_by_entity(
                item.clone().into(),
                item.user_name.clone(),
                item.order_type.clone(),
                &item.operations,
            )
            .await
            .map_err(|e| e.with_location(get_location!()))?;

            total += 1;
            processed_items.push((o, updated_item.item_name));
        }

        // match handle_item(
        //     item.wfm_url,
        //     item.sub_type,
        //     item.quantity,
        //     item.price,
        //     item.user_name,
        //     item.order_type,
        //     item.operations,
        // )
        // .await
        // {
        //     Ok((o, updated_item)) => {
        //         total += 1;
        //         processed_items.push((o, updated_item.item_name));
        //     }
        //     Err(e) => {
        //         return Err(e.with_location(get_location!()));
        //     }
        // }
    }
    Ok(total)
}
