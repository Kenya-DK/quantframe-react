use crate::{
    error::AppError,
    logger,
    structs::{Item, ItemDetails},
    wfm_client::client::WFMClient,
};

pub struct ItemModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> ItemModule<'a> {
    pub async fn get_all_items(&self) -> Result<Vec<Item>, AppError> {
        let (payload, _headers) = self.client.get("items", Some("items")).await?;
        Ok(payload)
    }
    pub async fn get_item(&self, item: String) -> Result<ItemDetails, AppError> {
        let url = format!("items/{}", item);
        match self.client.get(&url, Some("item")).await {
            Ok((item, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("For Item: {:?}", item).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(item)
            }
            Err(e) => Err(e),
        }
    }
}
