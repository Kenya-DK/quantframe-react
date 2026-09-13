use crate::{
    handlers::*,
    utils::{modules::states, CreateStockRivenExt},
    DATABASE,
};
use entity::{dto::*, enums::*, stock_riven::*};
use service::{sea_orm::DatabaseConnection, StockRivenMutation, StockRivenQuery};
use utils::SubType;
use utils::{get_location, info, warning, Error, OperationSet};
use wf_market::enums::OrderType;

// --------------------------------------------------
// Helper functions.
// --------------------------------------------------
fn log(
    component: &str,
    model: &Model,
    updated_model: &Option<Model>,
    status: &str,
    flags: &OperationSet,
    operations: &OperationSet,
) {
    let log_opts = utils::LoggerOptions::default();
    let sub_component = if operations.contains("StockRiven_Deleted") {
        "DeletedByUuid"
    } else if operations.contains("StockRiven_Create") {
        "CreatedByEntity"
    } else {
        "StockRivenOperation"
    };
    match (status, updated_model) {
        ("NotFound", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Stock riven not found for UUID: {} | Operations: {:?} | Flags: {:?}",
                model.uuid, operations.operations, flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableNotFoundLog")),
        ),

        (_, Some(updated)) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Sold stock riven {} {} | Bought: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                updated.weapon_name,
                updated.mod_name,
                updated.bought,
                status,
                operations.operations,
                flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Deleted", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Deleted stock riven {} {} | Status: {} | Operations: {:?} | Flags: {:?}",
                model.weapon_name,
                model.mod_name,
                status,
                operations.operations,
                flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableDeletedLog")),
        ),

        ("Updated", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Updated stock riven: {} {} | Status: {} | Operations: {:?} | Flags: {:?}",
                model.weapon_name,
                model.mod_name,
                status,
                operations.operations,
                flags.operations
            ),
            &log_opts.set_enable(!flags.has("DisableUpdatedLog")),
        ),

        ("Created", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Created stock riven: {} {} | Bought: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                model.weapon_name,
                model.mod_name,
                model.bought,
                status,
                operations.operations,
                flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCreatedLog")),
        ),

        ("Complete", _) => info(
            format!("{component}:{sub_component}"),
            &format!(
                "Completed stock riven: {} {} | Bought: {} | Status: {} | Operations: {:?} | Flags: {:?}",
                model.weapon_name,
                model.mod_name,
                model.bought,
                status,
                operations.operations,
                flags.operations
            ),
            &log_opts.set_enable(!flags.contains("DisableCompleteLog")),
        ),
        _ => {
            warning(
                format!("{component}:{sub_component}"),
                &format!(
                    "Unhandled status: {} for stock riven: {} {} | Operations: {:?} | Flags: {:?}",
                    status,
                    model.weapon_name,
                    model.mod_name,
                    operations.operations,
                    flags.operations
                ),
                &log_opts,
            );
        }
    }
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

async fn handle_stock_riven_delete(
    conn: &DatabaseConnection,
    model: &Model,
    operations: &mut OperationSet,
    component: &str,
) -> Result<String, Error> {
    match StockRivenMutation::delete_uuid(conn, &model.uuid).await {
        Ok(_) => {
            operations.add("StockRiven_Deleted");
            Ok("Deleted".to_string())
        }
        Err(e) => {
            if e.to_string().contains("NotFound") {
                operations.add("StockRiven_NotFound");
                Ok("NotFound".to_string())
            } else {
                Err(Error::new(
                    component,
                    format!("Failed to delete StockRiven: {}", e),
                    get_location!(),
                ))
            }
        }
    }
}
async fn handle_stock_riven_create(
    conn: &DatabaseConnection,
    model: Model,
    operations: &mut OperationSet,
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

    operations.add(format!("StockRiven_{op}"));

    Ok(created)
}

pub async fn handle_riven_by_model(
    mut model: Model,
    user_name: impl Into<String>,
    operation: OrderType,
    flags: &OperationSet,
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
            let status = handle_stock_riven_delete(conn, &model, &mut operations, component).await?;
            log(component, &model, &None, &status, flags, &operations);
        }

        OrderType::Buy => {
            model = handle_stock_riven_create(conn, model, &mut operations, component, file).await?;
            log(component, &model, &None, "Created", flags, &operations);
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
    if let Some(suffix) = flags.get_value_after("ReturnOn") {
        if operations.ends_with(suffix) {
            return Ok((operations, model));
        }
    }

    // --------------------------------------------------
    // Transaction creation
    // --------------------------------------------------
    if model.bought <= 0 {
        operations.add("StockRivenNotBought");
        log(component, &model, &None, "Complete", flags, &operations);
        return Ok((operations, model));
    }

    let mut tx = model.to_transaction(user_name, model.bought, TransactionType::Purchase);

    if operation == OrderType::Sell {
        tx.transaction_type = TransactionType::Sale;
    }

    handle_transaction(tx, &flags)
        .await
        .map_err(|e| e.with_location(get_location!()).log(file))?;
    log(component, &model, &None, "Complete", flags, &operations);

    Ok((operations, model))
}
pub async fn handle_riven_by_entity(
    mut item: CreateStockRiven,
    user_name: impl Into<String>,
    operation: OrderType,
    flags: &OperationSet,
) -> Result<(OperationSet, Model), Error> {
    let file = "handle_riven.log";
    item.validate().map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!()).log(file);
        e
    })?;
    handle_riven_by_model(item.to_model(), user_name, operation, flags)
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
    flags: &OperationSet,
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
    match handle_riven_by_model(model, user_name, operation, flags).await {
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
    flags: &OperationSet,
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
        flags,
    )
    .await
    .map_err(|e| e.with_location(get_location!()))
}