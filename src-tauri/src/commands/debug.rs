use std::sync::{Arc, Mutex};

use crate::{
    auth::AuthState, debug::DebugClient, structs::GlobleError, wfm_client::WFMClientState,
};

#[tauri::command]
pub async fn import_warframe_algo_trader_data(
    db_path: String,
    import_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), GlobleError> {
    let debug = debug.lock()?.clone();
    debug
        .import_warframe_algo_trader_data(db_path, import_type)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn reset_data(
    reset_type: String,
    debug: tauri::State<'_, Arc<Mutex<DebugClient>>>,
) -> Result<(), GlobleError> {
    let debug = debug.lock()?.clone();
    debug.reset_data(reset_type).await?;
    Ok(())
}
