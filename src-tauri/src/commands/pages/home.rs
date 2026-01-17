use crate::utils::modules::error::AppError;

#[tauri::command]
pub async fn get_statistic() -> Result<(), AppError> {
    Ok(())
}
