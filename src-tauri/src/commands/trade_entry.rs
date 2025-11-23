use std::sync::Mutex;

use entity::{dto::*, trade_entry::*};
use service::{TradeEntryMutation, TradeEntryQuery};
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{
    add_metric, app::client::AppState, enums::FindByType, types::PermissionsFlags,
    utils::CreateTradeEntryExt, APP, DATABASE,
};

#[tauri::command]
pub async fn get_trade_entry_pagination(
    query: TradeEntryPaginationQueryDto,
) -> Result<PaginatedResult<Model>, Error> {
    let conn = DATABASE.get().unwrap();
    match TradeEntryQuery::get_all(conn, query).await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(e.with_location(get_location!())),
    };
}

#[tauri::command]
pub async fn trade_entry_create(mut input: CreateTradeEntry) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();
    input.validate(FindByType::Id).map_err(|e| {
        let err = e.clone();
        err.with_location(get_location!())
            .log("trade_entry_create.log");
        e
    })?;

    let model = input.to_model();
    match TradeEntryMutation::create_or_update(conn, input.override_existing, &model).await {
        Ok(item) => Ok(item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
#[tauri::command]
pub async fn trade_entry_create_multiple(mut inputs: Vec<CreateTradeEntry>) -> Result<i64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut total = 0;
    for input in inputs.iter_mut() {
        input.validate(FindByType::Id).map_err(|e| {
            let err = e.clone();
            err.with_location(get_location!())
                .log("trade_entry_create_multiple.log");
            e
        })?;
        let model = input.to_model();
        match TradeEntryMutation::create_or_update(conn, input.override_existing, &model).await {
            Ok(_) => total += 1,
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    Ok(total)
}

#[tauri::command]
pub async fn trade_entry_delete(id: i64) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    let item = TradeEntryQuery::get_by_id(conn, id)
        .await
        .map_err(|e| e.with_location(get_location!()))?;
    if item.is_none() {
        return Err(Error::new(
            "Command::TradeEntryDelete",
            format!("Trade entry with ID {} not found", id),
            get_location!(),
        ));
    }
    let item = item.unwrap();

    add_metric!("trade_entry_delete", "manual");
    match TradeEntryMutation::delete_by_id(conn, id).await {
        Ok(_) => {}
        Err(e) => return Err(e.with_location(get_location!())),
    }

    Ok(item)
}
#[tauri::command]
pub async fn trade_entry_delete_multiple(ids: Vec<i64>) -> Result<i64, Error> {
    let conn = DATABASE.get().unwrap();
    let mut deleted_count = 0;

    for id in ids {
        match TradeEntryMutation::delete_by_id(conn, id).await {
            Ok(_) => deleted_count += 1,
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    Ok(deleted_count)
}
#[tauri::command]
pub async fn trade_entry_update(input: UpdateTradeEntry) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    match TradeEntryMutation::update_by_id(conn, input).await {
        Ok(item) => Ok(item),
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
#[tauri::command]
pub async fn trade_entry_get_by_id(id: i64) -> Result<Model, Error> {
    let conn = DATABASE.get().unwrap();

    match TradeEntryQuery::get_by_id(conn, id).await {
        Ok(item) => {
            if let Some(trade_entry) = item {
                Ok(trade_entry)
            } else {
                Err(Error::new(
                    "Command::TradeEntryGetById",
                    format!("Trade entry with ID {} not found", id),
                    get_location!(),
                ))
            }
        }
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
#[tauri::command]
pub async fn trade_entry_update_multiple(
    ids: Vec<i64>,
    input: UpdateTradeEntry,
) -> Result<Vec<Model>, Error> {
    let conn = DATABASE.get().unwrap();
    let mut updated_items = Vec::new();

    for id in ids {
        let mut update_input = input.clone();
        update_input.id = id;
        match TradeEntryMutation::update_by_id(conn, update_input).await {
            Ok(trade_entry) => updated_items.push(trade_entry),
            Err(e) => return Err(e.with_location(get_location!())),
        }
    }
    Ok(updated_items)
}

#[tauri::command]
pub async fn export_trade_entry_json(
    app_state: tauri::State<'_, Mutex<AppState>>,
    mut query: TradeEntryPaginationQueryDto,
) -> Result<String, Error> {
    let app_state = app_state.lock()?.clone();
    let app = APP.get().unwrap();
    if let Err(e) = app_state.user.has_permission(PermissionsFlags::ExportData) {
        e.log("export_trade_entry_json.log");
        return Err(e);
    }
    let conn = DATABASE.get().unwrap();
    query.pagination.limit = -1; // fetch all
    match TradeEntryQuery::get_all(conn, query).await {
        Ok(trade_entry) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("quantframe_trade_entry", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&trade_entry.results).map_err(|e| {
                    Error::new(
                        "Command::ExportTradeEntryJson",
                        format!("Failed to serialize trade entry to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportTradeEntryJson",
                        format!("Failed to write trade entry to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportTradeEntryJson",
                    format!("Exported trade entry to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                add_metric!("export_trade_entry_json", "success");
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => return Err(e.with_location(get_location!())),
    }
}
