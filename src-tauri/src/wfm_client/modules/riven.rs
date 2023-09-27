use crate::{
    error::AppError,
    logger,
    structs::{Item, ItemDetails, RivenTypeInfo, RivenAttributeInfo},
    wfm_client::client::WFMClient,
};

pub struct RivenModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> RivenModule<'a> {
    pub async fn get_all_riven_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let (payload, _headers) = self.client.get("riven/items", Some("items")).await?;
        Ok(payload)
    }
    pub async fn get_all_riven_attribute_types(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let (payload, _headers) = self
            .client
            .get("riven/attributes", Some("attributes"))
            .await?;
        Ok(payload)
    }
}
