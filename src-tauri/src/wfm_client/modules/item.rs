use crate::{
    error::{AppError, ApiResult},
    structs::{Item, ItemDetails},
    wfm_client::client::WFMClient, enums::LogLevel,
};

use  eyre::eyre;
pub struct ItemModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> ItemModule<'a> {
    pub async fn get_all_items(&self) -> Result<Vec<Item>, AppError> {
        match self.client.get("items", Some("items")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
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
