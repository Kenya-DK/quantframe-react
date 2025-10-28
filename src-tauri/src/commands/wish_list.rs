use std::{collections::HashMap, sync::Mutex};

use entity::{dto::*, wish_list::*};
use serde_json::{json, Value};
use service::{WishListMutation, WishListQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, group_by, info, Error, LoggerOptions};
use wf_market::enums::OrderType;

use crate::{
    APP, DATABASE, add_metric, app::client::AppState, enums::FindByType, handlers::{handle_wfm_item, handle_wish_list, handle_wish_list_by_entity}, helper, types::PermissionsFlags
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
    match handle_wish_list_by_entity(input, "", OrderType::Sell, FindByType::Url, &[]).await {
        Ok((_, item)) => return Ok(item),
        Err(e) => {
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
        FindByType::Url,
        &[],
    )
    .await
    {
        Ok((_, updated_item)) => return Ok(updated_item),
        Err(e) => {
            return Err(e.with_location(get_location!()).log("wish_list_buy.log"));
        }
    }
}

#[tauri::command]
pub async fn wish_list_delete(id: i64) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = WishListQuery::get_by_id(conn, id)
        .await
        .map_err(|e| e.with_location(get_location!()))?;
    if item.is_none() {
        return Err(Error::new(
            "Command::WishListDelete",
            format!("Wish list item with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    handle_wfm_item(&item.wfm_id, &item.sub_type, 1, OrderType::Buy, true)
        .await
        .map_err(|e| e.with_location(get_location!()).log("wish_list_delete.log"))?;
    add_metric!("wish_list_delete", "manual");
    match WishListMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => return Err(e.with_location(get_location!())),
    }

    Ok(item)
}
#[tauri::command]
pub async fn wish_list_update(input: UpdateWishList) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    match WishListMutation::update_by_id(conn, input).await {
        Ok(item) => Ok(item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}

#[tauri::command]
pub async fn wish_list_get_by_id(id: i64) -> Result<Value, Error> {
    let conn = DATABASE.get().unwrap();
    let item = match WishListQuery::find_by_id(conn, id).await {
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

    let (mut payload, _, _) = helper::get_item_details(
        FindByType::Id,
        &item.wfm_id,
        item.sub_type.clone(),
        OrderType::Buy,
    )
    .await?;

    payload["stock"] = json!(item);

    Ok(payload)
}
#[tauri::command]
pub async fn export_wish_list_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: WishListPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
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
                    Error::new(
                        "Command::ExportWishListJson",
                        format!("Failed to serialize wish list to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportWishListJson",
                        format!("Failed to write wish list to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportWishListJson",
                    format!("Exported wish list to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                add_metric!("export_wish_list_json", "success");
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
