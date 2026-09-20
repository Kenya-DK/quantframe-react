use std::{collections::HashMap, os::raw, sync::Mutex};

use entity::{dto::*, syndicate_item::*};
use qf_api::enums::app_events::ApplicationEvent as EventType;
use qf_api::{
    enums::FieldChange,
    types::{SyndicateItemPrice, SyndicateItemPricePaginationQueryDto},
};
use service::{SyndicateItemMutation, SyndicateItemQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, Error, LoggerOptions, OperationSet, SubType};
use wf_market::enums::OrderType;

use crate::{
    app::AppState,
    cache::CacheState,
    handlers::{handle_syndicate_item, handle_syndicate_item_by_entity, handle_wfm_item},
    helper::{self},
    live_scraper::is_disabled,
    track_event,
    types::PermissionsFlags,
    utils::ErrorFromExt,
    APP, DATABASE,
};

#[tauri::command]
pub async fn syndicate_item_import_items(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<i64, Error> {
    let app = app.lock()?.clone();
    if !app
        .user
        .has_permission(PermissionsFlags::SyndicatePricesSearch)?
    {
        let err = Error::new(
            "Command::SyndicateItemImportItems",
            "User does not have permission to search syndicate prices",
            get_location!(),
        );
        track_event!(
            EventType::SyndicateItemImport,
            [
                ("success", "false".to_string()),
                ("error_type", "permission_denied".to_string()),
            ]
        );
        return Err(err);
    }
    let settings = &app.settings.live_scraper.syndicate.wts;
    let mut query = SyndicateItemPricePaginationQueryDto::new(1, -1);
    query.syndicates = FieldChange::Value(settings.get_syndicate_ids());
    if !is_disabled(settings.volume_threshold) {
        query.volume_gt = FieldChange::Value(settings.volume_threshold);
    }
    if !is_disabled(settings.min_price) {
        query.min_price_gt = FieldChange::Value(settings.min_price);
    }

    let items = match app.qf_client.syndicate().get_prices(query).await {
        Ok(items) => items.results,
        Err(e) => {
            let error_type = e.error_type().to_string();
            let err = Error::from_qf(
                "SyndicateModule:InterestingItems",
                "Failed to get syndicate items",
                e,
                get_location!(),
            );
            track_event!(
                EventType::SyndicateItemImport,
                [
                    ("success", "false".to_string()),
                    ("error_type", error_type),
                ]
            );
            return Err(err);
        }
    };
    let types = |item: &SyndicateItemPrice| {
        if item.sub_type.is_none() || item.sub_type.clone().unwrap().rank.is_none() {
            return true;
        }
        let sub_type = item.sub_type.as_ref().unwrap();
        sub_type.rank.unwrap_or(0) <= 0
    };

    let combined_filter = |item: &SyndicateItemPrice| types(item);

    // Filter items based on settings
    let filtered_items: Vec<SyndicateItemPrice> = items
        .into_iter()
        .filter(|item| combined_filter(item))
        .collect();
    for item in filtered_items.iter() {
        let syndicate_item = CreateSyndicateItem::new(
            &item.wfm_id,
            item.sub_type.clone(),
            &item.syndicate_unique_name,
        );
        handle_syndicate_item_by_entity(
            syndicate_item,
            0,
            "",
            OrderType::Buy,
            &OperationSet::new(),
        )
        .await
        .map_err(|e| {
            track_event!(
                EventType::SyndicateItemImport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "import_failed".to_string()),
                ]
            );
            e.with_location(get_location!())
        })?;
    }
    info(
        "Command::SyndicateItemImportItems",
        format!(
            "Finished importing {} syndicate items.",
            filtered_items.len()
        ),
        &LoggerOptions::default(),
    );
    track_event!(
        EventType::SyndicateItemImport,
        [
            ("success", "true".to_string()),
            ("count", filtered_items.len().to_string()),
        ]
    );
    Ok(filtered_items.len() as i64)
}

#[tauri::command]
pub async fn get_syndicate_item_pagination(
    query: SyndicateItemPaginationQueryDto,
) -> Result<PaginatedResult<syndicate_item::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match SyndicateItemQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn get_syndicate_item_financial_report(
    query: SyndicateItemPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_syndicate_item_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn get_syndicate_item_status_counts(
    query: SyndicateItemPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_syndicate_item_pagination(query).await?;
    Ok(group_by(&items.results, |item| item.status.to_string())
        .iter()
        .map(|(status, items)| (status.clone(), items.len()))
        .collect::<HashMap<_, _>>())
}
#[tauri::command]
pub async fn get_syndicate_item_syndicate_counts(
    query: SyndicateItemPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_syndicate_item_pagination(query).await?;
    Ok(
        group_by(&items.results, |item| item.syndicate_name.to_string())
            .iter()
            .map(|(status, items)| (status.clone(), items.len()))
            .collect::<HashMap<_, _>>(),
    )
}

#[tauri::command]
pub async fn syndicate_item_create(
    input: CreateSyndicateItem,
) -> Result<syndicate_item::Model, Error> {
    match handle_syndicate_item_by_entity(input, 0, "", OrderType::Buy, &OperationSet::new()).await
    {
        Ok((_, updated_item)) => {
            track_event!(
                EventType::SyndicateItemCreate,
                [("success", "true".to_string())]
            );
            return Ok(updated_item);
        }
        Err(e) => {
            track_event!(
                EventType::SyndicateItemCreate,
                [
                    ("success", "false".to_string()),
                    ("error_type", "create_failed".to_string()),
                ]
            );
            return Err(e
                .with_location(get_location!())
                .log("syndicate_item_create.log"));
        }
    }
}

#[tauri::command]
pub async fn syndicate_item_sell(
    wfm_url: String,
    sub_type: Option<SubType>,
    raw_syndicate: String,
    price: i64,
) -> Result<syndicate_item::Model, Error> {
    match handle_syndicate_item(
        wfm_url,
        sub_type,
        raw_syndicate,
        price,
        "",
        OrderType::Sell,
        &OperationSet::new(),
    )
    .await
    {
        Ok((_, updated_item)) => {
            track_event!(
                EventType::SyndicateItemSell,
                [("success", "true".to_string())]
            );
            return Ok(updated_item);
        }
        Err(e) => {
            track_event!(
                EventType::SyndicateItemSell,
                [
                    ("success", "false".to_string()),
                    ("error_type", "sell_failed".to_string()),
                ]
            );
            return Err(e
                .with_location(get_location!())
                .log("syndicate_item_sell.log"));
        }
    }
}

#[tauri::command]
pub async fn syndicate_item_delete(id: i64) -> Result<syndicate_item::Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = SyndicateItemQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            track_event!(
                EventType::SyndicateItemDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", "query_failed".to_string()),
                ]
            );
            e.with_location(get_location!())
        })?;
    if item.is_none() {
        let err = Error::new(
            "Command::SyndicateItemDelete",
            format!("Syndicate item with ID {} not found", id),
            get_location!(),
        );
        track_event!(
            EventType::SyndicateItemDelete,
            [
                ("success", "false".to_string()),
                ("error_type", "item_not_found".to_string()),
            ]
        );
        return Err(err);
    }
    let item = item.unwrap();

    handle_wfm_item(
        &item.wfm_id,
        &item.sub_type,
        1,
        OrderType::Sell,
        OperationSet::from(vec!["ShouldDelete"]),
    )
    .await
    .map_err(|e| {
        track_event!(
            EventType::SyndicateItemDelete,
            [
                ("success", "false".to_string()),
                ("error_type", "delete_failed".to_string()),
            ]
        );
        e.with_location(get_location!())
            .log("syndicate_item_delete.log")
    })?;
    match SyndicateItemMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => {
            track_event!(
                EventType::SyndicateItemDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", "delete_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()));
        }
    }

    track_event!(
        EventType::SyndicateItemDelete,
        [("success", "true".to_string())]
    );
    Ok(item)
}

#[tauri::command]
pub async fn syndicate_item_delete_multiple(ids: Vec<i64>) -> Result<i64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut deleted_count = 0;

    for id in ids {
        match SyndicateItemMutation::delete_by_id(conn, id).await {
            Ok(_) => deleted_count += 1,
            Err(e) => {
                track_event!(
                    EventType::SyndicateItemDelete,
                    [
                        ("success", "false".to_string()),
                        ("error_type", "delete_failed".to_string()),
                    ]
                );
                return Err(e.with_location(get_location!()));
            }
        }
    }
    track_event!(
        EventType::SyndicateItemDelete,
        [
            ("success", "true".to_string()),
            ("count", deleted_count.to_string()),
        ]
    );
    Ok(deleted_count)
}

#[tauri::command]
pub async fn syndicate_item_update(
    input: UpdateSyndicateItem,
) -> Result<syndicate_item::Model, Error> {
    let conn = DATABASE.get().unwrap();
    match SyndicateItemMutation::update_by_id(conn, input).await {
        Ok(syndicate_item) => {
            track_event!(
                EventType::SyndicateItemUpdate,
                [("success", "true".to_string())]
            );
            Ok(syndicate_item)
        }
        Err(e) => {
            track_event!(
                EventType::SyndicateItemUpdate,
                [
                    ("success", "false".to_string()),
                    ("error_type", "update_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()));
        }
    }
}

#[tauri::command]
pub async fn syndicate_item_update_multiple(
    ids: Vec<i64>,
    input: UpdateSyndicateItem,
) -> Result<Vec<syndicate_item::Model>, Error> {
    let conn = DATABASE.get().unwrap();
    let mut updated_items = Vec::new();

    for id in ids {
        let mut update_input = input.clone();
        update_input.id = id;
        match SyndicateItemMutation::update_by_id(conn, update_input).await {
            Ok(syndicate_item) => updated_items.push(syndicate_item),
            Err(e) => {
                track_event!(
                    EventType::SyndicateItemUpdate,
                    [
                        ("success", "false".to_string()),
                        ("error_type", "update_failed".to_string()),
                    ]
                );
                return Err(e.with_location(get_location!()));
            }
        }
    }
    track_event!(
        EventType::SyndicateItemUpdate,
        [
            ("success", "true".to_string()),
            ("count", updated_items.len().to_string()),
        ]
    );
    Ok(updated_items)
}

#[tauri::command]
pub async fn syndicate_item_get_by_id(
    id: i64,
    operations: Option<Vec<String>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<syndicate_item::Model, Error> {
    let cache = cache.lock()?.clone();
    let app = app.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let mut item = match SyndicateItemQuery::find_by_id(conn, id).await {
        Ok(syndicate_item) => {
            if let Some(item) = syndicate_item {
                item
            } else {
                return Err(Error::new(
                    "Command::SyndicateItemGetById",
                    "Syndicate item not found",
                    get_location!(),
                ));
            }
        }
        Err(e) => return Err(e.with_location(get_location!())),
    };

    helper::populate_item_market_properties(
        &mut item.properties,
        &item.wfm_url,
        item.sub_type.clone(),
        item.standing_cost,
        item.list_price,
        OperationSet::from(
            operations.unwrap_or(
                vec!["MarketInfo", "TransactionInfo", "ProfitabilityInfo"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ),
        OrderType::Sell,
        &cache,
        &app.wfm_client,
    )
    .await?;

    Ok(item)
}

#[tauri::command]
pub async fn export_syndicate_item_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: SyndicateItemPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        track_event!(
            EventType::SyndicateItemExport,
            [
                ("success", "false".to_string()),
                ("error_type", "permission_denied".to_string()),
            ]
        );
        e.log("export_syndicate_item_json.log");
        return Err(e);
    }

    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match SyndicateItemQuery::get_all(conn, query).await {
        Ok(syndicate_items) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_Syndicate_Item", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&syndicate_items.results).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportSyndicateItemJson",
                        format!("Failed to serialize syndicate item to JSON: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::SyndicateItemExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "serialization_error".to_string()),
                        ]
                    );
                    err
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportSyndicateItemJson",
                        format!("Failed to write syndicate item to file: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::SyndicateItemExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "file_write_error".to_string()),
                        ]
                    );
                    err
                })?;
                info(
                    "Command::ExportSyndicateItemJson",
                    format!("Exported syndicate item to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                track_event!(
                    EventType::SyndicateItemExport,
                    [
                        ("success", "true".to_string()),
                        ("count", syndicate_items.results.len().to_string()),
                    ]
                );
                return Ok(file_path.to_string());
            }
            track_event!(
                EventType::SyndicateItemExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "cancelled".to_string()),
                ]
            );
            return Ok("".to_string());
        }
        Err(e) => {
            track_event!(
                EventType::SyndicateItemExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "query_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()));
        }
    }
}
