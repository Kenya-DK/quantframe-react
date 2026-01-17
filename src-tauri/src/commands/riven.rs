use qf_api::types::*;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{add_metric, app::client::AppState, utils::ErrorFromExt, APP};

#[tauri::command]
pub async fn riven_prices_lookup(
    query: RivenPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<RivenPrice>, Error> {
    let app_state = app.lock().unwrap().clone();
    match app_state.qf_client.riven().get_prices(query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_qf(
                "RivenPricesLookup",
                "Failed to lookup riven prices: {}",
                e,
                get_location!(),
            )
            .log("riven_prices_lookup.log");
            return Err(error);
        }
    };
}
#[tauri::command]
pub async fn export_riven_price_data(
    mut query: RivenPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app_state = app.lock().unwrap().clone();
    let app = APP.get().unwrap();
    query.pagination.limit = -1;
    match app_state.qf_client.riven().get_prices(query).await {
        Ok(data) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_RivenPrices", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&data.results).map_err(|e| {
                    Error::new(
                        "Command::ExportRivenPriceData",
                        format!("Failed to serialize riven prices to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportRivenPriceData",
                        format!("Failed to write riven prices to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportRivenPriceData",
                    format!("Exported riven prices to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                add_metric!("export_riven_price_data", "success");
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => {
            let error = Error::from_qf(
                "RivenPricesLookup",
                "Failed to lookup riven prices: {}",
                e,
                get_location!(),
            )
            .log("riven_prices_lookup.log");
            return Err(error);
        }
    };
}
