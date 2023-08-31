use std::sync::Arc;

use crate::{price_scraper::PriceScraper, structs::GlobleError};

#[tauri::command(async)]
pub async fn generate_price_history(
    days: i64,
    price_scraper: tauri::State<'_, Arc<std::sync::Mutex<PriceScraper>>>,
) -> Result<i64, GlobleError> {
    let price_scraper = price_scraper.lock().unwrap().clone();
    Ok(price_scraper.generate(days).await?)
}
