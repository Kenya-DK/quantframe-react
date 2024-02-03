use std::{
    clone,
    sync::{Arc, Mutex},
};
// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("command_stock.log".to_string()));

use crate::{
    database::{
        client::DBClient,
        modules::{stock_item, stock_riven::MatchRivenStruct},
    },
    enums::{LogLevel, OrderType},
    error::{self, AppError},
    logger,
    structs::{Order, RivenAttribute},
    wfm_client::client::WFMClient,
};
use eyre::eyre;
use once_cell::sync::Lazy;
use serde_json::json;

// Item Stock Commands
#[tauri::command]
pub async fn create_item_stock(
    url_name: String,
    quantity: i32,
    price: f64,
    rank: i32,
    sub_type: Option<&str>,
    minium_price: Option<i32>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    settings: tauri::State<'_, Arc<Mutex<crate::settings::SettingsState>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let settings = settings.lock()?.clone();

    // Create Item in Stock DB
    let stockitem = match db
        .stock_item()
        .create(&url_name, quantity, price, minium_price, rank, sub_type)
        .await
    {
        Ok(stockitem) => stockitem,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // Create transaction if price is greater than 0
    if price <= 0.0 {
        return Ok(json!(stockitem));
    }
    match db
        .transaction()
        .create(&url_name, "item", "buy", quantity, price as i32, rank, None)
        .await
    {
        Ok(_) => {
            // Send Close Event to Warframe Market API if enabled
            if !settings.live_scraper.stock_item.report_to_wfm {
                return Ok(serde_json::to_value(stockitem).unwrap());
            }
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };
    match wfm.orders().close(&url_name, OrderType::Buy).await {
        Ok(_) => {
            return Ok(json!(stockitem));
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn update_item_stock(
    id: i64,
    owned: Option<i32>,
    minium_price: Option<i32>,
    hidden: Option<bool>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    // Find Riven in Stock
    let stock = db.stock_item().get_by_id(id).await?;
    if stock.is_none() {
        return Err(AppError::new("Command", eyre!("Item not found")));
    }

    // Update Riven in Stock
    match db
        .stock_item()
        .update_by_id(id, owned, None, minium_price, None,None, hidden)
        .await
    {
        Ok(stock) => {
            return Ok(json!(stock.clone()));
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn delete_item_stock(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    // Delete Item from Stock
    let stockitem = match db.stock_item().delete(id).await {
        Ok(stockitem) => {
            // Send Delete Event to Frontend
            db.stock_item().emit("DELETE", json!(stockitem.clone()));
            stockitem
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // Get all sell orders from Warframe Market API and find the order for the item
    let ordres: Vec<Order> = wfm.orders().get_my_orders().await?.sell_orders;
    let order = ordres
        .iter()
        .find(|order| order.item.as_ref().unwrap().url_name == stockitem.url)
        .clone();
    // Check if order is found
    if order.is_none() {
        return Ok(json!(stockitem.clone()));
    }
    // Delete the order from Warframe Market API
    match wfm
        .orders()
        .delete(
            &order.unwrap().id
        )
        .await
    {
        Ok(_) => {
            return Ok(json!(stockitem.clone()));
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}

#[tauri::command]
pub async fn sell_item_stock(
    id: i64,
    quantity: i32,
    price: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    settings: tauri::State<'_, Arc<Mutex<crate::settings::SettingsState>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let settings = settings.lock()?.clone();

    // Sell Item in Stock DB
    let invantory = match db.stock_item().sell_item(id, quantity).await {
        Ok(invantory) => invantory,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // Send Stock Item to Frontend
    if invantory.owned == 0 {
        db.stock_item().emit("DELETE", json!(invantory.clone()));
    } else {
        db.stock_item()
            .emit("CREATE_OR_UPDATE", json!(invantory.clone()));
    }

    // Create Transaction in DB
    match db
        .transaction()
        .create(
            &invantory.url,
            "item",
            "sell",
            quantity,
            price,
            invantory.rank,
            None,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }

    if settings.live_scraper.stock_item.report_to_wfm {
        // Send Close Event to Warframe Market API
        match wfm.orders().close(&invantory.url, OrderType::Sell).await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e);
            }
        }
        return Ok(json!(invantory.clone()));
    }
    let ordres: Vec<Order> = wfm.orders().get_my_orders().await?.sell_orders;
    let order = ordres
        .iter()
        .find(|order| order.item.as_ref().unwrap().url_name == invantory.url)
        .clone();

    // Check if order is found
    if order.is_none() {
        return Ok(json!(invantory.clone()));
    }

    // Delete the order from Warframe Market API OR Update the order Warframe Market API
    if invantory.owned <= 0 {
        // Delete the order from Warframe Market API
        match wfm
            .orders()
            .delete(
                &order.unwrap().id
            )
            .await
        {
            Ok(_) => {
                return Ok(json!(invantory.clone()));
            }
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                if e.log_level() != LogLevel::Error {
                    return Err(e);
                } else {
                    return Ok(json!(invantory.clone()));
                }
            }
        }
    } else {
        // Update the order from Warframe Market API
        match wfm
            .orders()
            .update(
                &order.unwrap().id,
                order.unwrap().platinum as i32,
                invantory.owned,
                order.unwrap().visible
            )
            .await
        {
            Ok(_) => {
                return Ok(json!(invantory.clone()));
            }
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                return Err(e);
            }
        }
    }
}
#[tauri::command]
pub async fn sell_item_stock_by_url(
    name: String,
    quantity: i32,
    price: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
    settings: tauri::State<'_, Arc<Mutex<crate::settings::SettingsState>>>,
) -> Result<serde_json::Value, AppError> {
    let db_state = db.lock()?.clone();

    // Find Item in Stock DB by url_name
    let stock_item = match db_state
        .stock_item()
        .get_item_by_url_name(name.as_str())
        .await
    {
        Ok(strock_item) => {
            if strock_item.is_none() {
                return Err(AppError::new("Command", eyre!("Item not found: {}", name)));
            }
            strock_item.unwrap()
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    match sell_item_stock(stock_item.id.clone(), quantity, price, db, wfm, settings).await {
        Ok(invantory) => {
            return Ok(invantory);
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}

// Riven Stock Commands
#[tauri::command]
pub async fn create_riven_stock(
    id: String,
    price: f64,
    rank: i32,
    attributes: Vec<RivenAttribute>,
    match_riven: Option<MatchRivenStruct>,
    mastery_rank: i32,
    re_rolls: i32,
    polarity: &str,
    mod_name: &str,
    minium_price: Option<i32>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();

    // Create Riven in Stock DB
    let riven_item = match db
        .stock_riven()
        .create(
            None,
            &id,
            mod_name,
            price.clone(),
            rank,
            attributes,
            match_riven,
            mastery_rank,
            re_rolls,
            polarity,
            minium_price,
        )
        .await
    {
        Ok(riven_item) => riven_item,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    // If price is less than 0, return
    if price <= 0.0 {
        return Ok(json!(riven_item.clone()));
    }

    // Create Transaction
    match db
        .transaction()
        .create(
            &riven_item.weapon_url.clone(),
            "riven",
            "buy",
            1,
            price as i32,
            riven_item.rank.clone(),
            Some(json!(riven_item.clone())),
        )
        .await
    {
        Ok(_) => {
            return Ok(json!(riven_item.clone()));
        }
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }
}
#[tauri::command]
pub async fn import_auction(
    id: String,
    price: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();
    let auctions = wfm.auction().get_my_auctions().await?;

    // Find Auction in Warframe Market API
    let auction = auctions.iter().find(|auction| auction.id == id).clone();
    if auction.is_none() {
        return Err(AppError::new(
            "Command",
            eyre!("Auction {} not found", id.clone()),
        ));
    }
    let auction = auction.unwrap().clone();

    // Import Auction into Stock DB
    let riven_item = match db
        .stock_riven()
        .import_auction(auction.clone(), price)
        .await
    {
        Ok(riven_item) => riven_item,
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    };

    if riven_item.price <= 0.0 {
        return Ok(json!(riven_item.clone()));
    }
    // Create Transaction in DB
    match db
        .transaction()
        .create(
            &riven_item.weapon_url,
            "riven",
            "buy",
            1,
            price as i32,
            riven_item.rank,
            Some(json!(riven_item.clone())),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e);
        }
    }

    Ok(json!(riven_item.clone()))
}

#[tauri::command]
pub async fn delete_riven_stock(
    id: i64,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    // Find Riven in Stock
    let stock = db.stock_riven().get_by_id(id).await?;
    if stock.is_none() {
        return Err(AppError::new(
            "Command",
            eyre!("Riven {} not found", id.clone()),
        ));
    }

    let stock = stock.unwrap().clone();
    // Delete Riven from Stock
    db.stock_riven().delete(id).await?;

    let json_stock = serde_json::to_value(&stock).unwrap();

    // Delete Riven from Warframe Market
    if stock.order_id.is_some() {
        let order_id = stock.order_id.unwrap();
        match wfm.auction().delete(order_id.as_str()).await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                logger::info_con(
                    "CommandStock",
                    format!("Error deleting Riven from Warframe Market: {:?}", order_id).as_str(),
                );
            }
        };
    }

    Ok(json_stock)
}

#[tauri::command]
pub async fn update_riven_stock(
    id: i64,
    attributes: Option<Vec<RivenAttribute>>,
    match_riven: Option<MatchRivenStruct>,
    minium_price: Option<i32>,
    private: Option<bool>,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    // Find Riven in Stock
    let stock = db.stock_riven().get_by_id(id).await?;
    if stock.is_none() {
        return Err(AppError::new("Riven not found", eyre!("Riven not found")));
    }

    // Update Riven in Stock
    let stock = db
        .stock_riven()
        .update_by_id(
            id,
            None,
            None,
            None,
            None,
            attributes,
            match_riven,
            minium_price,
            None,
            private,
        )
        .await?;
    Ok(json!(stock.clone()))
}
#[tauri::command]
pub async fn sell_riven_stock(
    id: i64,
    price: i32,
    db: tauri::State<'_, Arc<Mutex<DBClient>>>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<serde_json::Value, AppError> {
    let db = db.lock()?.clone();
    let wfm = wfm.lock()?.clone();

    // Find Riven in Stock
    let stock = db.stock_riven().get_by_id(id).await?;
    if stock.is_none() {
        return Err(AppError::new("Riven not found", eyre!("Riven not found")));
    }
    let stock = stock.unwrap().clone();

    // Delete Riven from Stock
    db.stock_riven().delete(id).await?;

    let json_stock = serde_json::to_value(&stock).unwrap();

    // Delete Riven from Warframe Market
    if stock.order_id.is_some() {
        let order_id = stock.order_id.unwrap();
        match wfm.auction().delete(order_id.as_str()).await {
            Ok(_) => {}
            Err(e) => {
                error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
                logger::info_con(
                    "CommandStock",
                    format!("Error deleting Riven from Warframe Market: {:?}", order_id).as_str(),
                );
            }
        };
    }

    // Create Transaction
    db.transaction()
        .create(
            &stock.weapon_url,
            "riven",
            "sell",
            1,
            price as i32,
            stock.rank,
            Some(json!({
                "type": "riven",
                "weapon_url_name": stock.weapon_url,
                "re_rolls": stock.re_rolls,
                "polarity": stock.polarity,
                "name": stock.mod_name,
                "mod_rank": stock.rank,
                "mastery_level": stock.mastery_rank,
                "attributes": stock.attributes,
            })),
        )
        .await
        .map_err(|e| {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            e
        })?;
    Ok(json_stock)
}

// -----------------------------------------------------------------------------------------------
