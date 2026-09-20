use qf_api::enums::app_events::ApplicationEvent as EventType;
use qf_api::types::*;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{app::AppState, track_event, utils::ErrorFromExt, APP};

#[tauri::command]
pub async fn syndicate_item_prices_lookup(
    query: SyndicateItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<SyndicateItemPrice>, Error> {
    let app_state = app.lock().unwrap().clone();
    match app_state.qf_client.syndicate().get_prices(query).await {
        Ok(data) => {
            track_event!(
                EventType::SyndicateItemPricesLookup,
                [
                    ("success", "true".to_string()),
                    ("count", data.results.len().to_string()),
                ]
            );
            return Ok(data);
        }
        Err(e) => {
            let error_type = e.error_type().to_string();
            let error = Error::from_qf(
                "SyndicateItemPricesLookup",
                "Failed to lookup syndicate item prices: {}",
                e,
                get_location!(),
            )
            .log("syndicate_item_prices_lookup.log");
            track_event!(
                EventType::SyndicateItemPricesLookup,
                [
                    ("success", "false".to_string()),
                    ("error_type", error_type),
                ]
            );
            return Err(error);
        }
    };
}
#[tauri::command]
pub async fn export_syndicate_item_price_data(
    mut query: SyndicateItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app_state = app.lock().unwrap().clone();
    let app = APP.get().unwrap();
    query.pagination.limit = -1;
    match app_state.qf_client.syndicate().get_prices(query).await {
        Ok(data) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_SyndicateItemPrices", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&data.results).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportSyndicateItemPriceData",
                        format!("Failed to serialize syndicate item prices to JSON: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::SyndicateItemPricesExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "serialization_error".to_string()),
                        ]
                    );
                    err
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    let err = Error::new(
                        "Command::ExportSyndicateItemPriceData",
                        format!("Failed to write syndicate item prices to file: {}", e),
                        get_location!(),
                    );
                    track_event!(
                        EventType::SyndicateItemPricesExport,
                        [
                            ("success", "false".to_string()),
                            ("error_type", "file_write_error".to_string()),
                        ]
                    );
                    err
                })?;
                info(
                    "Command::ExportSyndicateItemPriceData",
                    format!("Exported syndicate item prices to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                track_event!(
                    EventType::SyndicateItemPricesExport,
                    [
                        ("success", "true".to_string()),
                        ("count", data.results.len().to_string()),
                    ]
                );
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            track_event!(
                EventType::SyndicateItemPricesExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", "cancelled".to_string()),
                ]
            );
            return Ok("".to_string());
        }
        Err(e) => {
            let error_type = e.error_type().to_string();
            let error = Error::from_qf(
                "SyndicateItemPricesLookup",
                "Failed to lookup syndicate item prices: {}",
                e,
                get_location!(),
            )
            .log("syndicate_item_prices_lookup.log");
            track_event!(
                EventType::SyndicateItemPricesExport,
                [
                    ("success", "false".to_string()),
                    ("error_type", error_type),
                ]
            );
            return Err(error);
        }
    };
}
