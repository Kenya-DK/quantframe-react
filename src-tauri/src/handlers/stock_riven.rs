use entity::{dto::*, enums::*, stock_riven::*};
use service::{sea_orm::DatabaseConnection, StockRivenMutation, StockRivenQuery};
use utils::{get_location, info, Error};
use wf_market::enums::OrderType;

use crate::{
    handlers::*,
    types::OperationSet,
    utils::{modules::states, CreateStockRivenExt},
    DATABASE,
};

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------

async fn handle_stock_riven_delete(
    conn: &DatabaseConnection,
    model: &Model,
    operations: &mut OperationSet,
    component: &str,
) -> Result<(), Error> {
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
    Ok(())
}
async fn handle_stock_riven_create(
    conn: &DatabaseConnection,
    model: Model,
    operations: &mut OperationSet,
    flags: &[&str],
    component: &str,
    file: &str,
) -> Result<Model, Error> {
    let (op, created) = StockRivenMutation::create(conn, model).await.map_err(|e| {
        Error::new(
            component,
            format!("Failed to create StockRiven: {e}"),
            get_location!(),
        )
        .log(file)
    })?;

    operations.add(op);

    info(
        format!("{component}:Create"),
        &format!("Created stock riven: {}", created.weapon_name),
        &utils::LoggerOptions::default().set_enable(!flags.contains(&"DisableCreateLog")),
    );

    Ok(created)
}
async fn delete_existing_auction(
    model: &Model,
    operations: &mut OperationSet,
    component: &str,
) -> Result<(), Error> {
    let app = states::app_state()?;

    if let Some(auction) = app
        .wfm_client
        .auction()
        .cache_auctions()
        .get_by_uuid(&model.uuid)
    {
        app.wfm_client
            .auction()
            .delete(&auction.id)
            .await
            .map_err(|e| {
                Error::new(
                    component,
                    format!("Failed to delete Auction: {e}"),
                    get_location!(),
                )
            })?;

        operations.add("Auction_Deleted");
    }

    Ok(())
}

pub async fn handle_riven_by_model(
    mut model: Model,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let conn = DATABASE.get().unwrap();
    let component = "HandleRiven";
    let file = "handle_riven.log";

    let mut operations = OperationSet::new();

    // --------------------------------------------------
    // Stock mutation (buy / sell)
    // --------------------------------------------------
    match operation {
        OrderType::Sell => {
            handle_stock_riven_delete(conn, &model, &mut operations, component).await?;
        }

        OrderType::Buy => {
            model = handle_stock_riven_create(
                conn,
                model,
                &mut operations,
                operation_flags,
                component,
                file,
            )
            .await?;
        }
    }

    // --------------------------------------------------
    // Auction cleanup (sell only)
    // --------------------------------------------------
    if operation == OrderType::Sell {
        delete_existing_auction(&model, &mut operations, component).await?;
    }

    // --------------------------------------------------
    // Early return condition from flags
    // --------------------------------------------------
    if operation_flags
        .iter()
        .find_map(|f| f.strip_prefix("ReturnOn:"))
        .map(|suffix| operations.ends_with(suffix))
        .unwrap_or(false)
    {
        return Ok((operations, model));
    }

    // --------------------------------------------------
    // Transaction creation
    // --------------------------------------------------
    if model.bought <= 0 {
        return Ok((operations, model));
    }

    let mut tx = model.to_transaction(user_name, model.bought, TransactionType::Purchase);

    if operation == OrderType::Sell {
        tx.transaction_type = TransactionType::Sale;
    }

    handle_transaction(tx, true)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;

    Ok((operations, model))
}
pub async fn handle_riven_by_entity(
    mut item: CreateStockRiven,
    user_name: impl Into<String>,
    operation: OrderType,
    operation_flags: &[&str],
) -> Result<(OperationSet, Model), Error> {
    let file = "handle_riven.log";
    item.validate().map_err(|e| {
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
    let file = "handle_riven_by_name.log";
    let mut operations: OperationSet = OperationSet::new();
    let weapon_url = weapon_url.into();
    let mod_name = mod_name.into();
    let model =
        match StockRivenQuery::get_by_riven_name(conn, &weapon_url, &mod_name, sub_type).await {
            Ok(model_opt) => model_opt,
            Err(e) => return Err(e.with_location(get_location!()).log(file)),
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
        operation_flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}
