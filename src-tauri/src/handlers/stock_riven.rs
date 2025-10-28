
use entity::{dto::*, enums::*, stock_riven::*};
use service::{StockRivenMutation, StockRivenQuery};
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{
    enums::*,
    handlers::*,
    types::OperationSet,
    utils::{modules::states, CreateStockRivenExt, ErrorFromExt},
    DATABASE,
};

pub async fn handle_riven_by_model(
    mut model: Model,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleRiven";
    let file = "handle_riven.log";
    let mut operations: OperationSet = OperationSet::new();
    // Handle StockItem creation, deletion, or update
    if operation == OrderType::Sell {
        match StockRivenMutation::delete_uuid(conn, &model.uuid).await {
            Ok(_) => {
                operations.add("StockRiven_Deleted".to_string());
            }
            Err(e) => {
                if e.to_string().contains("NotFound") {
                    operations.add("StockRiven_NotFound".to_string());
                } else {
                    return Err(Error::new(
                        component,
                        format!("Failed to delete StockRiven: {}", e),
                        get_location!(),
                    ));
                }
            }
        }
    } else if operation == OrderType::Buy {
        match StockRivenMutation::create(conn, model).await {
            Ok((c_operation, created_item)) => {
                operations.add(c_operation);
                model = created_item;
                info(
                    format!("{}:Create", component),
                    &format!("Created stock riven: {}", model.weapon_name),
                    &utils::LoggerOptions::default()
                        .set_enable(!operation_flags.contains(&"DisableCreateLog")),
                );
            }
            Err(e) => {
                return Err(Error::new(
                    component,
                    format!("Failed to create StockRiven: {}", e),
                    get_location!(),
                )
                .log(file));
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
                    operations.add("Auction_Deleted".to_string());
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
    if operation_flags.iter().any(|op| op.starts_with("ReturnOn:")) {
        let return_on = operation_flags
            .iter()
            .filter(|op| op.starts_with("ReturnOn:"))
            .cloned()
            .collect::<Vec<_>>();
        if return_on.len() > 0 {
            let return_on = return_on[0].replace("ReturnOn:", "");
            if operations.ends_with(&return_on) {
                return Ok((operations, model));
            }
        }
    }

    if model.bought <= 0 {
        return Ok((operations, model));
    }

    let mut transaction = model.to_transaction(user_name, model.bought, TransactionType::Purchase);
    if operation == OrderType::Sell {
        transaction.transaction_type = TransactionType::Sale;
    }
    handle_transaction(transaction)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    Ok((operations, model))
}
pub async fn handle_riven_by_entity(
    mut item: CreateStockRiven,
    user_name: impl Into<String>,
    operation: OrderType,
    find_by: FindByType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let file = "handle_riven.log";
    item.validate(find_by).map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(file);
        e
    })?;
    handle_riven_by_model(item.to_model(), user_name, operation, operation_flags)
        .await
        .map_err(|e| e.with_location(get_location!()))
}

/// Handles stock riven operations (buy/sell) with WFM integration
pub async fn handle_riven_by_name(
    weapon_url: impl Into<String>,
    mod_name: impl Into<String>,
    sub_type: SubType,
    bought: i64,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Option<Model>), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleRivenByName";
    let file = "handle_riven_by_name.log";
    let mut operations: OperationSet = OperationSet::new();
    let weapon_url = weapon_url.into();
    let mod_name = mod_name.into();
    let model = match StockRivenQuery::get_by_riven_name(
        conn,
        &weapon_url,
        &mod_name,
        sub_type,
    )
    .await
    {
        Ok(model_opt) => model_opt,
        Err(e) => {
            return Err(Error::from_db(
                component,
                format!("Failed to query StockRiven by name: {}", weapon_url),
                e,
                get_location!(),
            )
            .log(file))
        }
    };
    if model.is_none() {
        operations.add("StockRiven_NotFound".to_string());
        return Ok((operations, None));
    }
    let mut model = model.unwrap();
    model.bought = bought;
    match handle_riven_by_model(model, user_name, operation, operation_flags).await {
        Ok((operations, model)) => {
            return Ok((operations, Some(model)));
        }
        Err(e) => return Err(e.with_location(get_location!()).log(file)),
    }
}
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
    find_by: FindByType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
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
        find_by,
        operation_flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
