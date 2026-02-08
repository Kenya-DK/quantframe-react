use crate::handlers::*;
use utils::{get_location, Error};

#[tauri::command]
pub async fn handles_handle_items(items: Vec<ItemEntity>) -> Result<i32, Error> {
    match handle_items(items).await {
        Ok((total, _)) => Ok(total),
        Err(e) => {
            return Err(e.with_location(get_location!()).log("handle_items.log"));
        }
    }
}
