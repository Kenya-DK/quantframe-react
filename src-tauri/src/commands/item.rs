use qf_api::types::*;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{add_metric, app::client::AppState, utils::ErrorFromExt, APP};

#[tauri::command]
pub async fn item_prices_lookup(
    query: ItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<ItemPrice>, Error> {
    let app_state = app.lock().unwrap().clone();
    match app_state.qf_client.item().get_prices(query).await {
        Ok(data) => return Ok(data),
        Err(e) => {
            let error = Error::from_qf(
                "ItemPricesLookup",
                "Failed to lookup item prices: {}",
                e,
                get_location!(),
            )
            .log("item_prices_lookup.log");
            return Err(error);
        }
    };
}
#[tauri::command]
pub async fn export_item_price_data(
    mut query: ItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app_state = app.lock().unwrap().clone();
    let app = APP.get().unwrap();
    query.pagination.limit = -1;
    match app_state.qf_client.item().get_prices(query).await {
        Ok(data) => {
            let file_path = app
                .dialog()
                .file()
                .add_filter("Quantframe_ItemPrices", &["json"])
                .blocking_save_file();
            if let Some(file_path) = file_path {
                let json = serde_json::to_string_pretty(&data.results).map_err(|e| {
                    Error::new(
                        "Command::ExportItemPriceData",
                        format!("Failed to serialize item prices to JSON: {}", e),
                        get_location!(),
                    )
                })?;
                std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
                    Error::new(
                        "Command::ExportItemPriceData",
                        format!("Failed to write item prices to file: {}", e),
                        get_location!(),
                    )
                })?;
                info(
                    "Command::ExportItemPriceData",
                    format!("Exported item prices to JSON file: {}", file_path),
                    &LoggerOptions::default(),
                );
                add_metric!("export_item_price_data", "success");
                return Ok(file_path.to_string());
            }
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
            return Ok("".to_string());
        }
        Err(e) => {
            let error = Error::from_qf(
                "ItemPricesLookup",
                "Failed to lookup item prices: {}",
                e,
                get_location!(),
            )
            .log("item_prices_lookup.log");
            return Err(error);
        }
    };
}
