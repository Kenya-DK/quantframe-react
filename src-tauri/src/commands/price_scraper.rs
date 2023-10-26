use std::sync::Arc;

use crate::{error::AppError, price_scraper::PriceScraper};

// Create a static variable to store the log file name
// static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("app.log".to_string()));

#[tauri::command(async)]
pub async fn generate_price_history(
    days: i64,
    price_scraper: tauri::State<'_, Arc<std::sync::Mutex<PriceScraper>>>,
) -> Result<i64, AppError> {
    let price_scraper = price_scraper.lock().unwrap().clone();
    Ok(price_scraper.generate(days).await?)
}
