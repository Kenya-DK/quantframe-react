use crate::{
    enums::LogLevel,
    error::{ApiResult, AppError},
    structs::{Item, ItemDetails},
    wfm_client::client::WFMClient,
};

use eyre::eyre;
pub struct ItemModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> ItemModule<'a> {
    pub async fn get_all_items(&self) -> Result<Vec<Item>, AppError> {
        match self.client.get::<Vec<Item>>("items", Some("items")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    "Item:GetAllItems",
                    format!("{} items were fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "WarframeMarket:Item:GetAllItems",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    pub async fn get_item(&self, item: String) -> Result<ItemDetails, AppError> {
        let url = format!("items/{}", item);
        match self.client.get(&url, Some("item")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    "Item:GetItem",
                    format!("Gettting item: {}", item).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "WarframeMarket:Item:GetItem",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
