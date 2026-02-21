use std::sync::{Arc, Mutex};

use entity::{
    dto::{PaginatedResult, PaginationQueryDto},
    enums::FieldChange,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri_plugin_dialog::DialogExt;
use utils::*;

use crate::{add_metric, app::client::AppState, helper::paginate, log_parser::LogParserState, APP};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineEntryPaginationQueryDto {
    #[serde(flatten)]
    pub pagination: PaginationQueryDto,

    #[serde(default)]
    pub query: FieldChange<String>,

    #[serde(default)]
    pub sort_by: FieldChange<String>,

    #[serde(default)]
    pub sort_direction: FieldChange<SortDirection>,

    #[serde(default)]
    pub hide_empty: FieldChange<bool>,

    #[serde(default)]
    pub start_index: FieldChange<usize>,

    #[serde(default)]
    pub end_index: FieldChange<usize>,
}

#[tauri::command]
pub fn debug_get_wfm_state(app: tauri::State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    let app = app.lock()?.clone();
    let orders = app.wfm_client.order().cache_orders();
    let user_auctions = app.wfm_client.auction().cache_auctions();
    let tracking = app.wfm_client.get_tracking().clone();
    let mut payload = json!({
      "user_orders": json!(orders),
      "user_auctions": json!(user_auctions),
      "order_limit": app.wfm_client.order().get_order_limit(),
      "tracking": json!(tracking),
      "limiters": {}
    });
    let per_rate_limit = app.wfm_client.get_per_route_limiter().clone();
    for (key, route) in per_rate_limit.lock()?.iter() {
        payload["limiters"][key] = json!({"limit": route.quota_type.current_limit(), "wait_time_sec": route.wait_time_sec, "quota_type": route.quota_type.quota_type()});
    }
    Ok(payload)
}

#[tauri::command]
pub fn debug_get_ee_logs(
    query: LineEntryPaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<PaginatedResult<LineEntry>, Error> {
    let log_parser = log_parser.lock()?;
    let cached_lines = if let (FieldChange::Value(start), FieldChange::Value(end)) =
        (&query.start_index, &query.end_index)
    {
        log_parser.get_cached_lines_between(*start, *end)
    } else {
        log_parser.get_all_cached_lines()
    };
    let filtered_auctions = filters_by(&cached_lines, |o| {
        match &query.query {
            FieldChange::Value(q) => {
                if !o.line.as_str().contains(q) {
                    return false;
                }
            }
            _ => {}
        }
        match &query.hide_empty {
            FieldChange::Value(hide_empty) => {
                if *hide_empty && o.line.is_empty() {
                    return false;
                }
            }
            _ => {}
        }

        true
    });

    let paginate = paginate(
        &filtered_auctions,
        query.pagination.page,
        query.pagination.limit,
    );
    Ok(paginate)
}
#[tauri::command]
pub async fn debug_export_ee_logs(
    mut query: LineEntryPaginationQueryDto,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
) -> Result<String, Error> {
    let app = APP.get().unwrap();
    query.pagination.limit = -1; // fetch all
    let items = debug_get_ee_logs(query, log_parser)?;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Quantframe_EE_Logs", &["json"])
        .blocking_save_file();
    if let Some(file_path) = file_path {
        let json = serde_json::to_string_pretty(&items.results).map_err(|e| {
            Error::new(
                "Command::ExportEELogs",
                format!("Failed to serialize EE logs to JSON: {}", e),
                get_location!(),
            )
        })?;
        std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
            Error::new(
                "Command::ExportEELogs",
                format!("Failed to write EE logs to file: {}", e),
                get_location!(),
            )
        })?;
        info(
            "Command::ExportEELogs",
            format!("Exported EE logs to JSON file: {}", file_path),
            &LoggerOptions::default(),
        );
        add_metric!("export_ee_logs", "success");
        return Ok(file_path.to_string());
    }
    Ok("".to_string())
}
