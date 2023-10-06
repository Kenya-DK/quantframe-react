use serde_json::json;

use crate::{
    error::{self, AppError},
    helper, logger,
    structs::{Auction, AuctionItem, Item, ItemDetails, RivenAttributeInfo, RivenTypeInfo},
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
    pub async fn create(
        &self,
        auction_type: &str,
        note: &str,
        starting_price: &str,
        buyout_price: f64,
        minimal_reputation: i32,
        minimal_increment: i32,
        private: bool,
        item: AuctionItem,
    ) -> Result<Auction<String>, AppError> {
        // Construct any JSON body
        let mut body = json!({
            "note": note,
            "starting_price": starting_price,
            "buyout_price": buyout_price,
            "minimal_reputation": minimal_reputation,
            "minimal_increment": minimal_increment,
            "private": private
        });

        if auction_type == "riven" {
            let item_riven = json!({
                "type": "riven",
                "re_rolls": item.re_rolls,
                "attributes": item.attributes,
                "name": item.name,
                "weapon_url_name": item.weapon_url_name,
                "mod_rank": item.mod_rank,
                "polarity": item.polarity
            });
            body["item"] = item_riven;
        } else if auction_type == "item" {

        }

        match self
            .client
            .post("auctions/create", Some("auction"), body)
            .await
        {
            Ok((auction, _headers)) => {
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&auction).unwrap());
                Ok(auction)
            }
            Err(e) => Err(e),
        }
    }
}
