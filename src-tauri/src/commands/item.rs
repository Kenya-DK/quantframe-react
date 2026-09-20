use qf_api::{enums::ApplicationEvent, types::*};
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use utils::{get_location, info, Error, LoggerOptions};

use crate::{app::AppState, track_event, utils::ErrorFromExt, APP};

#[tauri::command]
pub async fn item_prices_lookup(
    query: ItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Paginated<ItemPrice>, Error> {
    let app = app.lock()?.clone();

    match app.qf_client.item().get_prices(query).await {
        Ok(data) => {
            track_event!(
                ApplicationEvent::ItemPriceLookup,
                [
                    ("success", "true".to_string()),
                    ("count", data.results.len().to_string()),
                ]
            );
            Ok(data)
        }
        Err(e) => {
            track_event!(
                ApplicationEvent::ItemPriceLookup,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

            Err(Error::from_qf(
                "ItemPricesLookup",
                "Failed to lookup item prices",
                e,
                get_location!(),
            )
            .log("item_prices_lookup.log"))
        }
    }
}
#[tauri::command]
pub async fn item_price_lookup(
    id: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<ItemPriceDetails, Error> {
    let app = app.lock()?.clone();

    match app.qf_client.item().get_price_by_id(&id).await {
        Ok(data) => {
            track_event!(
                ApplicationEvent::ItemPriceLookup,
                [("success", "true".to_string())]
            );
            Ok(data)
        }
        Err(e) => {
            track_event!(
                ApplicationEvent::ItemPriceLookup,
                [
                    ("success", "false".to_string()),
                    ("error_type", e.error_type().to_string()),
                ]
            );

            Err(Error::from_qf(
                "ItemPriceLookup",
                "Failed to lookup item price",
                e,
                get_location!(),
            )
            .log("item_price_lookup.log"))
        }
    }
}
#[tauri::command]
pub async fn export_item_price_data(
    mut query: ItemPricePaginationQueryDto,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let app_state = app.lock()?.clone();
    let app = APP.get().unwrap();

    query.pagination.limit = -1;

    let trace_event_error = |error_type: &str| {
        track_event!(
            ApplicationEvent::ItemPriceExport,
            [
                ("success", "false".to_string()),
                ("error_type", error_type.to_string()),
            ]
        );
    };

    let data = match app_state.qf_client.item().get_prices(query).await {
        Ok(data) => data,
        Err(e) => {
            trace_event_error(&e.error_type().to_string());

            return Err(Error::from_qf(
                "ItemPricesLookup",
                "Failed to lookup item prices",
                e,
                get_location!(),
            )
            .log("item_prices_lookup.log"));
        }
    };

    let Some(file_path) = app
        .dialog()
        .file()
        .add_filter("Quantframe_ItemPrices", &["json"])
        .blocking_save_file()
    else {
        trace_event_error("cancelled");

        return Ok(String::new());
    };

    let json = serde_json::to_string_pretty(&data.results).map_err(|e| {
        trace_event_error("serialization_error");
        Error::new(
            "Command::ExportItemPriceData",
            format!("Failed to serialize item prices to JSON: {e}"),
            get_location!(),
        )
    })?;

    std::fs::write(file_path.as_path().unwrap(), json).map_err(|e| {
        trace_event_error("file_write_error");
        Error::new(
            "Command::ExportItemPriceData",
            format!("Failed to write item prices to file: {e}"),
            get_location!(),
        )
    })?;

    info(
        "Command::ExportItemPriceData",
        format!("Exported item prices to JSON file: {file_path}"),
        &LoggerOptions::default(),
    );

    track_event!(
        ApplicationEvent::ItemPriceExport,
        [
            ("success", "true".to_string()),
            ("count", data.results.len().to_string()),
        ]
    );

    Ok(file_path.to_string())
}
