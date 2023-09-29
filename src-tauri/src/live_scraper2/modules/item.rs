use crate::{
    error::AppError,
    logger,
};

pub struct ItemModule<'a> {
    pub client: &'a LiveScraperClient,
}

impl<'a> ItemModule<'a> {

    pub async fn check_stock(&self) -> Result<(), AppError> {
        logger::info_con("ItemModule", "Run item module");
        Ok(())
    }
}
