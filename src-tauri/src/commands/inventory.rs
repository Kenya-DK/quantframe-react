use std::sync::{Arc, Mutex};

use crate::{
    database::client::DBClient,
    database::modules::inventory::InventoryStruct,
    error::{self, AppError},
    structs::{Invantory, Order, RivenAttribute},
    wfm_client::client::WFMClient,
};

#[tauri::command]
pub async fn create_invantory_entry(
    id: String,
    report: bool,
    quantity: i32,
    item_type: String,
    price: f64,
    rank: i32,
    sub_type: Option<&str>,
    attributes: Option<Vec<RivenAttribute>>,
    mastery_rank: Option<i32>,
    re_rolls: Option<i32>,
    polarity: Option<&str>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<InventoryStruct, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    match db
        .inventory()
        .create(
            &id,
            &item_type,
            quantity,
            price,
            rank,
            sub_type,
            attributes.clone(),
            mastery_rank,
            re_rolls,
            polarity,
        )
        .await
    {
        Ok(invantory) => {
            // Create transaction
            db.transaction()
                .create(
                    &id,
                    "item",
                    "buy",
                    quantity,
                    price as i32,
                    rank,
                    sub_type,
                    attributes,
                    mastery_rank,
                    re_rolls,
                    polarity
                )
                .await?;
            // Send Close Event to Warframe Market API
            if report {
                wfm.orders().close(&id, "buy").await?;
            }
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}

#[tauri::command]
pub async fn delete_invantory_entry(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<InventoryStruct, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    match db.inventory().delete(id).await {
        Ok(invantory) => {
            db.inventory()
                .emit("DELETE", serde_json::to_value(invantory.clone()).unwrap());
            let ordres: Vec<Order> = wfm.orders().get_my_orders().await?.sell_orders;
            let order = ordres
                .iter()
                .find(|order| order.item.as_ref().unwrap().url_name == invantory.item_url)
                .clone();
            if order.is_some() {
                wfm.orders()
                    .delete(
                        &order.unwrap().id,
                        &invantory.item_name,
                        &invantory.item_id,
                        "sell",
                    )
                    .await?;
            }
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}
#[tauri::command]
pub async fn sell_invantory_entry(
    id: i64,
    item_type: String,
    report: bool,
    price: i32,
    quantity: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<InventoryStruct, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    match db
        .inventory()
        .sell_item(id, &item_type, price, quantity)
        .await
    {
        Ok(invantory) => {
            if invantory.owned == 0 {
                db.inventory()
                    .emit("DELETE", serde_json::to_value(invantory.clone()).unwrap());
            } else {
                db.inventory().emit(
                    "CREATE_OR_UPDATE",
                    serde_json::to_value(invantory.clone()).unwrap(),
                );
            }
            db.transaction()
                .create(
                    &invantory.item_url,
                    &invantory.item_type,
                    "sell",
                    quantity,
                    price,
                    invantory.rank,
                    invantory.sub_type.as_deref(),
                    Some(invantory.clone().attributes.0),
                    invantory.mastery_rank,
                    invantory.re_rolls,
                    invantory.polarity.as_deref(),
                )
                .await?;

            // Send Close Event to Warframe Market API
            if report {
                wfm.orders().close(&invantory.item_url, "sell").await?;
            } else {
                let ordres: Vec<Order> = wfm.orders().get_my_orders().await?.sell_orders;
                let order = ordres
                    .iter()
                    .find(|order| order.item.as_ref().unwrap().url_name == invantory.item_url)
                    .clone();
                if order.is_some() {
                    if invantory.owned <= 0 {
                        wfm.orders()
                            .delete(
                                &order.unwrap().id,
                                &invantory.item_name,
                                &invantory.item_id,
                                "sell",
                            )
                            .await?;
                    } else {
                        wfm.orders()
                            .update(
                                &order.unwrap().id,
                                order.unwrap().platinum as i32,
                                invantory.owned,
                                order.unwrap().visible,
                                &invantory.item_name,
                                &invantory.item_id,
                                "sell",
                            )
                            .await?;
                    }
                }
            }
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(db.log_file.clone(), &e);
            return Err(e);
        }
    };
}
