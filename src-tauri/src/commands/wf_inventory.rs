use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use entity::enums::FieldChange;
use migration::query;
use qf_api::types::*;
use serde_json::{json, Value};
use utils::{get_location, Error};

use crate::wf_inventory::VeiledRivensPaginationDto;
use crate::{
    app::client::AppState, cache::CacheState, utils::ErrorFromExt, wf_inventory::WFInventoryState,
};

#[tauri::command]
pub async fn wf_inventory_get_veiled_rivens(
    query: VeiledRivensPaginationDto,
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<Value, Error> {
    let wf_inventory = wf_inventory.lock()?;
    Ok(json!(wf_inventory.riven().get_veiled(query)?))
}
#[tauri::command]
pub async fn wf_inventory_get_unveiled_rivens(
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<Value, Error> {
    let wf_inventory = wf_inventory.lock()?;
    Ok(json!(wf_inventory.riven().get_unveiled()?))
}

#[tauri::command]
pub async fn wf_inventory_get_path(
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<String, Error> {
    let wf_inventory = wf_inventory.lock()?;
    Ok(wf_inventory.get_path().to_string_lossy().to_string())
}

#[tauri::command]
pub async fn wf_inventory_update_path(
    path: String,
    wf_inventory: tauri::State<'_, Mutex<Arc<WFInventoryState>>>,
) -> Result<(), Error> {
    let mut wf_inventory = wf_inventory.lock()?;
    let new_path = PathBuf::from(path);

    if !new_path.exists() {
        return Err(Error::new(
            "WFInventoryUpdatePath",
            format!("File does not exist: {}", new_path.display()),
            get_location!(),
        ));
    }

    Arc::get_mut(&mut wf_inventory)
        .ok_or_else(|| {
            Error::new(
                "AlecaframeUpdatePath",
                "Cannot update path while other references exist",
                get_location!(),
            )
        })?
        .update_path(new_path);

    Ok(())
}
