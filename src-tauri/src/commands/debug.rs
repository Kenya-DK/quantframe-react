use std::sync::{Arc, Mutex};

use crate::{debug::DebugClient, utils::modules::error::AppError};

#[tauri::command]
pub fn migrate_data_base(// debug_client: tauri_api::State<Arc<Mutex<DebugClient>>>,
) -> Result<(), AppError> {
    // let debug_client = debug_client.lock().unwrap();
    Ok(())
}
