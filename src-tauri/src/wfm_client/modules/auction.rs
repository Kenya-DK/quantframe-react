use crate::{
    error::{self, AppError},
    logger,
    structs::{Auction, Item, ItemDetails, RivenAttributeInfo, RivenTypeInfo},
    wfm_client::client::WFMClient,
};

pub struct AuctionModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> AuctionModule<'a> {
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

    // User methods (sea-query)
    pub async fn get_user_auctions(
        &self,
        ingame_name: &str,
    ) -> Result<Vec<Auction<String>>, AppError> {
        let url = format!("profile/{}/auctions", ingame_name);
        match self.client.get(&url, Some("auctions")).await {
            Ok((orders, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("From User Auctions: {}", ingame_name).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(orders)
            }
            Err(e) => {
                error::create_log_file("wfm.log".to_string(), &e);
                Err(e)
            }
        }
    }

    pub async fn get_my_auctions(&self) -> Result<Vec<Auction<String>>, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let auctions = self.get_user_auctions(auth.ingame_name.as_str()).await?;
        Ok(auctions)
    }
}
