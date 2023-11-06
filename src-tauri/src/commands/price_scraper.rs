use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{error::{AppError, self}, price_scraper::PriceScraper};

// Create a static variable to store the log file name
static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("price_history.log".to_string()));

#[tauri::command(async)]
pub async fn generate_price_history(
    days: i64,
    price_scraper: tauri::State<'_, Arc<std::sync::Mutex<PriceScraper>>>,
) -> Result<i64, AppError> {
    let price_scraper = price_scraper.lock().unwrap().clone();
    match price_scraper.generate(days).await {
        Ok(_) => Ok(0),
        Err(e) => {
            error::create_log_file(LOG_FILE.lock().unwrap().to_owned(), &e);
            return Err(e)
        }
    }
}
