use crate::{error::AppError, live_scraper::client::LiveScraperClient, logger};

pub struct HelperModule<'a> {
    pub client: &'a LiveScraperClient,
}

impl<'a> HelperModule<'a> {}
