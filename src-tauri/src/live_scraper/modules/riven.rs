use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{auth::AuthState, error::AppError, live_scraper::client::LiveScraperClient, logger};
pub struct RivenModule<'a> {
    pub client: &'a LiveScraperClient,
}

impl<'a> RivenModule<'a> {
    pub async fn check_stock(&self) -> Result<(), AppError> {
        logger::info_con("RivenModule", "Run riven module");
        Ok(())
    }
}
