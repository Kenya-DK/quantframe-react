use entity::stock_riven::StockRivenPaginationQueryDto;
use qf_api::enums::app_events::ApplicationEvent as EventType;
use serde_json::{json, Value};
use service::StockRivenQuery;
use std::sync::{Arc, Mutex};
use utils::Error;

use crate::wf_inventory::WFInventoryState;
use crate::wf_inventory::WFItemPaginationDto;
use crate::{track_event, DATABASE};

#[tauri::command]
pub async fn wf_inventory_get_rivens(
    query: WFItemPaginationDto,
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<Value, Error> {
    let wf_inventory = wf_inventory.lock()?.clone();
    let conn = DATABASE.get().unwrap();
    let rivens = StockRivenQuery::get_all(conn, StockRivenPaginationQueryDto::new(1, -1)).await?;
    let uuids: Vec<String> = rivens.results.iter().map(|r| r.uuid.clone()).collect();
    let mut veiled = wf_inventory.riven().get_rivens(query)?;
    for item in veiled.results.iter_mut() {
        item.base
            .properties
            .set_property_value("is_in_stock", uuids.contains(&item.base.uuid));
    }

    Ok(json!(veiled))
}
#[tauri::command]
pub async fn wf_inventory_get_syndicates(
    query: WFItemPaginationDto,
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<Value, Error> {
    let wf_inventory = wf_inventory.lock()?.clone();
    Ok(json!(&wf_inventory.syndicate().get_syndicates(query)?))
}

#[tauri::command]
pub async fn wf_inventory_update(
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<(), Error> {
    let wf_inventory = wf_inventory.lock()?.clone();
    wf_inventory.update().map_err(|e| {
        track_event!(
            EventType::WFInventoryUpdate,
            [
                ("success", "false".to_string()),
                ("error_type", "update_failed".to_string()),
            ]
        );
        e
    })?;
    track_event!(EventType::WFInventoryUpdate, [("success", "true".to_string())]);
    Ok(())
}
