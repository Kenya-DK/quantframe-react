use entity::{dto::*, wish_list::*};
use qf_api::enums::app_events::ApplicationEvent as EventType;
use service::{WishListMutation, WishListQuery};
use std::{collections::HashMap, sync::Mutex};
use tauri_plugin_dialog::DialogExt;
use utils::SubType;
use utils::{get_location, group_by, info, Error, LoggerOptions, OperationSet};
use wf_market::enums::OrderType;

use crate::{
    app::AppState,
    cache::CacheState,
    handlers::{handle_wfm_item, handle_wish_list, handle_wish_list_by_entity},
    helper, track_event,
    types::PermissionsFlags,
    APP, DATABASE,
};

#[tauri::command]
pub async fn get_wish_list_pagination(
    query: WishListPaginationQueryDto,
) -> Result<PaginatedResult<Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match WishListQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn get_wish_list_financial_report(
    query: WishListPaginationQueryDto,
) -> Result<FinancialReport, Error> {
    let items = get_wish_list_pagination(query).await?;
    Ok(FinancialReport::from(&items.results))
}

#[tauri::command]
pub async fn get_wish_list_status_counts(
    query: WishListPaginationQueryDto,
) -> Result<HashMap<String, usize>, Error> {
    let items = get_wish_list_pagination(query).await?;
    Ok(group_by(&items.results, |item| item.status.to_string())
        .iter()
        .map(|(status, items)| (status.clone(), items.len()))
        .collect::<HashMap<_, _>>())
}

#[tauri::command]
pub async fn wish_list_create(input: CreateWishListItem) -> Result<Model, Error> {
    match handle_wish_list_by_entity(input, "", OrderType::Sell, &OperationSet::new()).await {
        Ok((_, item)) => {
            track_event!(
                EventType::WishListCreate,
                [("success", "true".to_string())]
            );
            return Ok(item);
        }
        Err(e) => {
            track_event!(
                EventType::WishListCreate,
                [
                    ("success", "false".to_string()),
                    ("error_type", "create_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()).log("wish_list_buy.log"));
        }
    }
}

#[tauri::command]
pub async fn wish_list_bought(
    wfm_url: String,
    sub_type: Option<SubType>,
    quantity: i64,
    price: i64,
) -> Result<Model, Error> {
    match handle_wish_list(
        wfm_url,
        &sub_type,
        quantity,
        price,
        "",
        OrderType::Buy,
        &OperationSet::new(),
    )
    .await
    {
        Ok((_, updated_item)) => {
            track_event!(
                EventType::WishListBought,
                [("success", "true".to_string())]
            );
            return Ok(updated_item);
        }
        Err(e) => {
            track_event!(
                EventType::WishListBought,
                [
                    ("success", "false".to_string()),
                    ("error_type", "bought_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()).log("wish_list_buy.log"));
        }
    }
}

#[tauri::command]
pub async fn wish_list_delete(id: i64) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = WishListQuery::get_by_id(conn, id)
        .await
        .map_err(|e| {
            track_event!(
                EventType::WishListDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", "query_failed".to_string()),
                ]
            );
            e.with_location(get_location!())
        })?;
    if item.is_none() {
        let err = Error::new(
            "Command::WishListDelete",
            format!("Wish list item with ID {} not found", id),
            get_location!(),
        );
        track_event!(
            EventType::WishListDelete,
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
        OrderType::Buy,
        OperationSet::from(vec!["ShouldDelete"]),
    )
    .await
    .map_err(|e| {
        track_event!(
            EventType::WishListDelete,
            [
                ("success", "false".to_string()),
                ("error_type", "delete_failed".to_string()),
            ]
        );
        e.with_location(get_location!()).log("wish_list_delete.log")
    })?;
    match WishListMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => {
            track_event!(
                EventType::WishListDelete,
                [
                    ("success", "false".to_string()),
                    ("error_type", "delete_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()));
        }
    }

    track_event!(
        EventType::WishListDelete,
        [("success", "true".to_string())]
    );
    Ok(item)
}
#[tauri::command]
pub async fn wish_list_delete_multiple(ids: Vec<i64>) -> Result<i64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut deleted_count = 0;

    for id in ids {
        match WishListMutation::delete_by_id(conn, id).await {
            Ok(_) => deleted_count += 1,
            Err(e) => {
                track_event!(
                    EventType::WishListDelete,
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
        EventType::WishListDelete,
        [
            ("success", "true".to_string()),
            ("count", deleted_count.to_string()),
        ]
    );
    Ok(deleted_count)
}
#[tauri::command]
pub async fn wish_list_update(input: UpdateWishList) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    match WishListMutation::update_by_id(conn, input).await {
        Ok(item) => {
            track_event!(
                EventType::WishListUpdate,
                [("success", "true".to_string())]
            );
            Ok(item)
        }
        Err(e) => {
            track_event!(
                EventType::WishListUpdate,
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
pub async fn wish_list_update_multiple(
    ids: Vec<i64>,
    input: UpdateWishList,
) -> Result<Vec<Model>, Error> {
    let conn = DATABASE.get().unwrap();
    let mut updated_items = Vec::new();

    for id in ids {
        let mut update_input = input.clone();
        update_input.id = id;
        match WishListMutation::update_by_id(conn, update_input).await {
            Ok(wish_list) => updated_items.push(wish_list),
            Err(e) => {
                track_event!(
                    EventType::WishListUpdate,
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
        EventType::WishListUpdate,
        [
            ("success", "true".to_string()),
            ("count", updated_items.len().to_string()),
        ]
    );
    Ok(updated_items)
}
#[tauri::command]

pub async fn wish_list_get_by_id(
    id: i64,
    operations: Option<Vec<String>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<wish_list::Model, Error> {
    let cache = cache.lock()?.clone();
    let app = app.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let mut item = match WishListQuery::find_by_id(conn, id).await {
        Ok(wish_list_item) => {
            if let Some(item) = wish_list_item {
                item
            } else {
                return Err(Error::new(
                    "Command::WishListGetById",
                    "Wish list item not found",
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
        0,
        item.list_price,
        OperationSet::from(
            operations.unwrap_or(
                vec!["MarketInfo"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ),
        OrderType::Buy,
        &cache,
        &app.wfm_client,
    )
    .await?;

    Ok(item)
}
#[tauri::command]
pub async fn export_wish_list_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: WishListPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        track_event!(
            EventType::WishListExport,
            [
                ("success", "false".to_string()),
                ("error_type", "permission_denied".to_string()),
            ]
        );
        e.log("export_wish_list_json.log");
        return Err(e);
    }
    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match WishListQuery::get_all(conn, query).await {
        Ok(wish_list) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_Wish_List", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&wish_list.results).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportWishListJson",
                        format!("Failed to serialize wish list to JSON: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::WishListExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "serialization_error".to_string()),
                        ]
                    );
                    err
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportWishListJson",
                        format!("Failed to write wish list to file: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::WishListExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "file_write_error".to_string()),
                        ]
                    );
                    err
                })?;
                info(
                    "Command::ExportWishListJson",
                    format!("Exported wish list to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                track_event!(
                    EventType::WishListExport,
                    [
                        ("success", "true".to_string()),
                        ("count", wish_list.results.len().to_string()),
                    ]
                );
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            track_event!(
                EventType::WishListExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "cancelled".to_string()),
                ]
            );
            return Ok("".to_string());
        }
        Err(e) => {
            track_event!(
                EventType::WishListExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "query_failed".to_string()),
                ]
            );
            return Err(e.with_location(get_location!()));
        }
    }
}
