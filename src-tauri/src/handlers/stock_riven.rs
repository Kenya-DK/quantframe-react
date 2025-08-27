use entity::{dto::*, enums::*, stock_riven::*};
use service::{StockItemMutation, StockRivenMutation};
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{
    enums::*,
    handlers::*,
    utils::{modules::states, CreateStockRivenExt},
    DATABASE,
};

pub async fn handle_riven_by_entity(
    mut item: CreateStockRiven,
    user_name: impl Into<String>,
    operation: OrderType,
) -> Result<(Vec<String>, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleRiven";
    let file = "handle_riven.log";
    let mut operations: Vec<String> = vec![];
    item.validate(FindByType::Url).map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(Some(file));
        e
    })?;
    let mut model = item.to_model();

    // Handle StockItem creation, deletion, or update
    if operation == OrderType::Sell {
        match StockRivenMutation::delete_uuid(conn, &model.uuid).await {
            Ok(_) => {
                operations.push("StockRiven_Deleted".to_string());
            }
            Err(e) => {
                return Err(Error::new(
                    component,
                    format!("Failed to delete StockRiven: {}", e),
                    get_location!(),
                ))
            }
        }
    } else if operation == OrderType::Buy {
        match StockRivenMutation::create(conn, model).await {
            Ok((c_operation, created_item)) => {
                operations.push(c_operation);
                model = created_item;
                info(
                    format!("{}:Create", component),
                    &format!("Created stock riven: {}", model.weapon_name),
                    &utils::LoggerOptions::default(),
                );
            }
            Err(e) => {
                return Err(Error::new(
                    component,
                    format!("Failed to create StockRiven: {}", e),
                    get_location!(),
                )
                .log(Some(file)));
            }
        }
    }

    // If the operation is a sale, we need to check if there's an existing order
    if operation == OrderType::Sell {
        let app = states::app_state()?;
        if let Some(auction) = app
            .wfm_client
            .auction()
            .cache_auctions()
            .get_by_uuid(&model.uuid)
        {
            match app.wfm_client.auction().delete(&auction.id).await {
                Ok(_) => {
                    operations.push("Auction_Deleted".to_string());
                }
                Err(e) => {
                    return Err(Error::new(
                        component,
                        format!("Failed to delete Auction: {}", e),
                        get_location!(),
                    ))
                }
            }
        }
    }

    // Create a transaction from the item
    if !item.is_validated {
        return Err(Error::new(
            component,
            "Stock riven item is not validated yet",
            get_location!(),
        )
        .log(Some(file)));
    }
    if item.bought.unwrap_or(0) <= 0 {
        return Ok((operations, model));
    }

    let mut transaction = model.to_transaction(
        user_name,
        item.bought.unwrap_or(0),
        TransactionType::Purchase,
    );
    if operation == OrderType::Sell {
        transaction.transaction_type = TransactionType::Sale;
    }
    handle_transaction(transaction)
        .await
        .map_err(|e| e.with_location(get_location!()).log(Some(file)))?;

    Ok((operations, model))
}

/// Handles stock riven operations (buy/sell) with WFM integration
pub async fn handle_riven(
    wfm_url: String,
    mod_name: String,
    mastery_rank: i64,
    rank: i64,
    re_rolls: i64,
    polarity: String,
    attributes: Vec<RivenAttribute>,
    bought: i64,
    user_name: impl Into<String>,
    operation: OrderType,
) -> Result<(Vec<String>, Model), Error> {
    handle_riven_by_entity(
        CreateStockRiven::new(
            wfm_url,
            mod_name,
            mastery_rank,
            re_rolls,
            polarity,
            attributes,
            rank,
        )
        .set_bought(bought),
        user_name,
        operation,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
